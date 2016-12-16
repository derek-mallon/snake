use std::ops;
use std::fmt;
use std::clone;
use std::marker;
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub struct Vector2D<T>{
    pub x:T,
    pub y:T,
}
impl<T> Vector2D<T>{
    pub fn new(x:T,y:T)->Vector2D<T>{
        Vector2D{x:x,y:y}
    }
}
impl<T> fmt::Display for Vector2D<T> where T:fmt::Display {
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"({0},{1})",self.x,self.y)
    }
}
impl<T> ops::Add<Vector2D<T>> for Vector2D<T> where T: ops::Add<Output=T> +  marker::Copy{
    type Output = Vector2D<T>;
    fn add(self,rhs: Vector2D<T>)-> Vector2D<T>{
        Vector2D::new(self.x+rhs.x,self.y+rhs.y)
    }
}
impl<T> ops::Sub<Vector2D<T>> for Vector2D<T> where T: ops::Sub<Output=T> +  marker::Copy{
    type Output = Vector2D<T>;
    fn sub(self,rhs: Vector2D<T>)-> Vector2D<T>{
        Vector2D::new(self.x-rhs.x,self.y-rhs.y)
    }
}
impl<T> ops::Add<T> for Vector2D<T> where T: ops::Add<Output=T> + marker::Copy{
    type Output = Vector2D<T>;
    fn add(self,rhs: T)-> Vector2D<T>{
        Vector2D::new(self.x+rhs,self.y+rhs)
    }
}
impl<T> ops::Sub<T> for Vector2D<T> where T: ops::Sub<Output=T> +  marker::Copy{
    type Output = Vector2D<T>;
    fn sub(self,rhs: T)-> Vector2D<T>{
        Vector2D::new(self.x-rhs,self.y-rhs)
    }
}
impl<T> ops::Mul<T> for Vector2D<T> where T: ops::Mul<Output=T> +  marker::Copy{
    type Output = Vector2D<T>;
    fn mul(self,rhs: T)-> Vector2D<T>{
        Vector2D::new(self.x*rhs,self.y*rhs)
    }
}
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub struct Rect  {
    pub x:i32,
    pub y:i32,
    pub width:u32,
    pub height:u32,
}
impl Rect{
    pub fn new(x:i32,y:i32,width:u32,height:u32)-> Rect{
        Rect{x:x,y:y,width:width,height:height}
    }
    pub fn from_vector2d(pos:Vector2D<i32>,dimensions:Vector2D<u32>)->Rect{
        Rect{x:pos.x,y:pos.y,width:dimensions.x,height:dimensions.y}
    }
    pub fn from_vector2d_center(center:Vector2D<i32>,dimensions:Vector2D<u32>)->Rect{
        Rect{x:center.x-(dimensions.x/2) as i32,y:center.y-(dimensions.y/2) as i32,width:dimensions.x,height:dimensions.y}
    }
}

