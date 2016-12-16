extern crate rustc_serialize;
extern crate sdl2;
extern crate sdl2_image;
//modules
pub mod sprite_creator;
pub mod asset_manager;
pub mod engine;
pub mod game_object;
pub mod pool;
pub mod geom;
pub mod sdl_wrapper;
pub mod quadtree;
pub mod storage;
//imports
use engine::game_run;
//Constants
//Main function.
fn main() {
    game_run();
}

