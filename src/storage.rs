use std::mem;
//For storage
pub trait Storable<T> : Sized + Clone{
    fn to_data(t:&Self)->T;
    fn from_data(t:T)->Self;
}
pub trait Zero: Sized + Copy{
    fn to_zero()->Self{
        unsafe {mem::zeroed()}
    }
}
#[derive(Clone)]
struct Test{
    test:Vec<u32>,

}

impl Drop for Test{
    fn drop(&mut self){
        println!("drop");
    }
}
impl Storable<[u8;32]> for Test {
    fn to_data(t:&Self)->[u8;32]{
        unsafe  { mem::transmute(t.clone())}
    }
    fn from_data(t:[u8;32])->Self{
        unsafe {mem::transmute(t)}
    }
}
