use geom::Vector2D;
use std::mem;
const NODE_CAP : usize = 4;
pub trait Item : Copy{
    fn inside(&self,range:&AABB)->bool;
    fn empty()->Self;
    fn intersects(&self,range:&AABB)->bool;
    fn contains(&self,point:&Vector2D<f32>)->bool;
}

impl Item for Vector2D<f32> {
    fn inside(&self,range:&AABB)->bool{
        if self.x >= range.bottom_right_corner.x &&
            self.x <= range.bottom_right_corner.x+range.dimension && 
            self.y >= range.bottom_right_corner.y && 
            self.y <= range.bottom_right_corner.y+range.dimension {
                return true;
            }
        return false;
    }
    fn empty() ->Vector2D<f32>{
        Vector2D::new(0.0,0.0)
    }
    fn intersects(&self,range:&AABB)->bool{
        self.inside(range)
    }
    fn contains(&self, point:&Vector2D<f32>)->bool{
        false
    }
}
#[derive(Clone,Copy)]
pub struct AABB{
    bottom_right_corner:Vector2D<f32>,
    dimension:f32
}
impl AABB{
    pub fn new(bottom_right_corner:Vector2D<f32>,dimension:f32)->AABB{
        AABB{bottom_right_corner:bottom_right_corner,dimension:dimension}
    }
    pub fn contains_item<T>(&self, item:&T)->bool where T:Item{
        item.inside(self)
    }
    pub fn intersects_item<T>(&self, item:&T)->bool where T:Item{
        item.intersects(self)
    }
    pub fn intersects_aabb(&self, other:&AABB)->bool{
        if self.contains_item(&other.bottom_right_corner) ||
           self.contains_item(&Vector2D{x:other.bottom_right_corner.x+other.dimension,y:other.bottom_right_corner.y}) ||
           self.contains_item(&Vector2D{x:other.bottom_right_corner.x+other.dimension,y:other.bottom_right_corner.y+other.dimension}) ||
           self.contains_item(&Vector2D{x:other.bottom_right_corner.x,y:other.bottom_right_corner.y+other.dimension}) {
               return true;
           }
        return false;
    }
}
impl Item for AABB{
    fn inside(&self,range:&AABB)->bool{
        if range.contains_item(&self.bottom_right_corner) &&
           range.contains_item(&Vector2D{x:self.bottom_right_corner.x+self.dimension,y:self.bottom_right_corner.y}) &&
           range.contains_item(&Vector2D{x:self.bottom_right_corner.x+self.dimension,y:self.bottom_right_corner.y+self.dimension}) &&
           range.contains_item(&Vector2D{x:self.bottom_right_corner.x,y:self.bottom_right_corner.y+self.dimension}) {
               return true;
           }
        return false;
    }
    fn empty()->AABB{
        AABB{
            bottom_right_corner:Vector2D::empty(),
            dimension:0.0
        }
    }
    fn intersects(&self,range:&AABB)->bool{
        self.intersects_aabb(range)
    }
    fn contains(&self,point:&Vector2D<f32>)->bool{
        self.contains_item(point)
    }
}
enum Quadrant{
    NorthWest = 0,
    NorthEast = 1,
    SouthWest = 2,
    SouthEast = 3,
}
pub struct QuadTree<T:Item>{
    boundary:AABB,
    items:Vec<T>,
    children:Vec<QuadTree<T>>
}
impl<T> QuadTree<T> where T:Item{
    pub fn new(boundary:AABB)->QuadTree<T>{
        QuadTree{
            boundary:boundary,
            items:Vec::with_capacity(4),
            children:Vec::with_capacity(4)
        }
    }
    pub fn insert(&mut self, item:T)->bool{
        if !self.boundary.contains_item(&item) {
            return false;
        }
        if self.items.len() <= NODE_CAP{
            self.items.push(item);
            return true;
        }
        if self.children.len()  == 0 {
            let half_dimension = self.boundary.dimension/2.0;
            self.children.push(QuadTree::new(AABB::new(Vector2D::new(self.boundary.bottom_right_corner.x,self.boundary.bottom_right_corner.y+half_dimension),half_dimension)));
            self.children.push(QuadTree::new(AABB::new(Vector2D::new(self.boundary.bottom_right_corner.x+half_dimension,self.boundary.bottom_right_corner.y+half_dimension),half_dimension)));
            self.children.push(QuadTree::new(AABB::new(Vector2D::new(self.boundary.bottom_right_corner.x,self.boundary.bottom_right_corner.y+half_dimension),half_dimension)));
            self.children.push(QuadTree::new(AABB::new(Vector2D::new(self.boundary.bottom_right_corner.x,self.boundary.bottom_right_corner.y),half_dimension)));
            for child in &mut self.children{
                if child.insert(item) { return true;}
            }
        }
        false
    }
    pub fn query_range(&self,range:&AABB)->Vec<T>{
        let mut vector = Vec::new();
        if !self.boundary.intersects_aabb(range) {
            return vector;
        }
        for item in &self.items {
            if range.contains_item(item) {
                vector.push(*item);
            }
        }
        for child in &self.children {
            vector.extend_from_slice(child.query_range(range).as_slice());
        }
        return vector;
    }
    pub fn query_range_allow_intersects(&self,range:&AABB)->Vec<T>{
        let mut vector = Vec::new();
        if !self.boundary.intersects_aabb(range) {
            return vector;
        }
        for item in &self.items {
            if range.intersects_item(item) {
                vector.push(*item);
            }
        }
        for child in &self.children {
            vector.extend_from_slice(child.query_range_allow_intersects(range).as_slice());
        }
        return vector;
    }
    pub fn query_point(&self,point:&Vector2D<f32>)->Vec<T>{
        let mut vector = Vec::new();
        if !self.boundary.contains(point) {
            return vector;
        }
        for item in &self.items {
            if item.contains(point){
                vector.push(*item);
            }
        }
        for child in &self.children {
            vector.extend_from_slice(child.query_point(point).as_slice());
        }
        return vector;
    }
    pub fn query_all(&self)->Vec<T>{
        let mut vector = Vec::new();
        for item in &self.items{
            vector.push(*item);
        }
        for child in &self.children {
            vector.extend_from_slice(child.query_all().as_slice());
        }
        return vector;
    }
    pub fn update_items<F>(&mut self,closure:&mut F)->Action where F : FnMut(&mut T)->Action{
        let mut remove = Vec::new();
        for i in 0..self.items.len(){
            match closure(&mut self.items[i]){
                Action::Delete => {
                    remove.push(i);
                }
                _=>{
                }
            } 
        }
        for i in remove.into_iter().rev() {
            self.items.swap_remove(i);
        }
        //Clean up
        let numb_item_needed = NODE_CAP - self.items.len();
        if numb_item_needed > 0 {
            //if this is a base block
            if self.children.len() == 0{
                //this is a empty block throw it away
                if self.items.len() == 0 {
                    return Action::Delete;
                }
                //otherwise don't do anything because there are no children to steal from.
                return Action::Nothing;
            //if there is child block try to grab a item from each of them them
            }else{
                let mut current_child = 0;
                let mut numb_items_taken = 0;
                while numb_items_taken < numb_item_needed {
                    //if we have not looped over all of the children
                    if current_child < self.children.len(){
                        //Check for items
                        if self.children[current_child].items.len() != 0{
                            //Take an item
                            numb_items_taken += 1;
                            self.items.push(self.children[current_child].items.pop().unwrap());
                        }
                        //go to the next child
                        current_child += 1;
                    }else {
                        //break early if there are no children to steal from
                        break;
                    }
                }
            }
        }
        //make list of possibly dead children.
        let mut remove = Vec::new();
        //send it down and watch for the delete action.
        for i in 0..self.children.len(){
            if let Action::Delete = self.children[i].update_items(closure){
                remove.push(i);
            }
        }
        //delete if needed
        if remove.len() == 4{
            self.children.clear();
        }
        for child in &mut self.children{
            child.update_items(closure);
        }
        return Action::Nothing;
    }
}
pub enum Action{
    Delete,
    Nothing,
}
#[test]
pub fn test_contains_point(){
    let point = Vector2D{x:2.0,y:0.0};
    let point2 = Vector2D{x:13.0,y:1.0};
    let aabb= AABB{bottom_right_corner:Vector2D{x:0.0,y:0.0},dimension:10.0};
    assert!(aabb.contains_item(&point));
    assert!(!aabb.contains_item(&point2))
}
#[test]
pub fn test_intersects_aabb(){
    let aabb= AABB{bottom_right_corner:Vector2D{x:0.0,y:0.0},dimension:10.0};
    let aabb2= AABB{bottom_right_corner:Vector2D{x:2.0,y:0.0},dimension:10.0};
    let aabb3= AABB{bottom_right_corner:Vector2D{x:12.0,y:0.0},dimension:10.0};
    assert!(aabb.intersects_aabb(&aabb2));
    assert!(!aabb.intersects_aabb(&aabb3));
}
#[test]
pub fn test_insert_and_query(){
    let inital_boundary = AABB::new(Vector2D::new(0.0,0.0),50.0);
    let mut quadtree = QuadTree::new(inital_boundary);
    quadtree.insert(Vector2D::new(1.0,3.9));
    quadtree.insert(Vector2D::new(333.0,3.9));
    quadtree.insert(Vector2D::new(13.0,25.9));
    quadtree.insert(Vector2D::new(40.0,32.7));
    assert!(quadtree.query_range(&inital_boundary).len() == 3);
    assert!(quadtree.query_range(&AABB::new(Vector2D::new(0.0,0.0),10.0)).len()==1);
    let mut rect_quad = QuadTree::new(inital_boundary);
    rect_quad.insert(AABB::new(Vector2D::new(0.5,0.5),9.0));
    assert!(rect_quad.query_range(&inital_boundary).len() == 1);
    assert!(rect_quad.query_range(&AABB::new(Vector2D::new(1.0,1.0),1.0)).len() == 0);
    assert!(rect_quad.query_range_allow_intersects(&AABB::new(Vector2D::new(1.0,1.0),1.0)).len() == 1);
    assert!(rect_quad.query_point(&Vector2D::new(1.0,1.0)).len() == 1);
    quadtree.update_items(&mut move|item|{
        println!("{:?}",item);
        return Action::Delete;
    });
    println!("{}",quadtree.query_range(&inital_boundary).len() == 0);
}

