use geom::Vector2D;
use geom::Rect;
pub struct GameObject{
    pub pos:Vector2D<i32>,
    pub sprite:Sprite
}
impl GameObject{
    pub fn new()->GameObject{
        GameObject{
            pos:Vector2D::new(0,0),
            sprite:Sprite::new()
        }
    }
    pub fn update(&mut self)->(Rect,Rect,f64,Vector2D<i32>,bool,bool){
        let update = self.sprite.update();
        (update.0,Rect::from_vector2d(self.pos,Vector2D::new(update.0.width,update.0.height)),update.1,update.2,update.3,update.4)
    }
    pub fn set_pos(&mut self){

    }
}
#[derive(Debug,Clone)]
pub struct Sprite{
    pub frames:Vec<Rect>,
    pub start:usize,
    pub end:usize,
    counter:usize,
    pub angle:f64,
    pub pivot:Vector2D<i32>,
    pub flipped_horizontal:bool,
    pub flipped_vertical:bool
}
impl Sprite{
    pub fn new()->Sprite{
        Sprite{
            frames:Vec::new(),
            start:0,
            end:0,
            counter:0,
            angle:0.0,
            pivot:Vector2D::new(0,0),
            flipped_horizontal:false,
            flipped_vertical:false
        }
    }
    pub fn update(&mut self)->(Rect,f64,Vector2D<i32>,bool,bool){
        let rect = self.frames[self.counter];
        self.counter += 1;
        if self.counter > self.end {
            self.counter = self.start;
        }
        (rect,self.angle,self.pivot,self.flipped_horizontal,self.flipped_vertical)
    }
}
