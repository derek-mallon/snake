use sdl2::event::Event;
use std::path::Path;
use sdl_wrapper::*;
use asset_manager::*;
use sprite_creator::*;
use game_object::*;
use pool::*;
const NUMB_OF_GAME_OBJS: u16 = 50000;
pub struct Game{
    pub game_objects:Pool<GameObject>,
}
impl Game{
    pub fn new()->Game{
        Game{
            game_objects:Pool::new(NUMB_OF_GAME_OBJS),
        }
    }
    pub fn init(&mut self){

    }
}
pub fn game_run(){
    let mut asset_manager = AssetManger::new();
    asset_manager.load_asset_list();
    asset_manager.load_assets_all();
    let mut sdl_manager = SdlManager::new(asset_manager.sdl_config);
    sdl_manager.load_texture(Path::new(asset_manager.texture_path.as_str()));
    let mut sprite_creator = SpriteCreator::new(asset_manager.sprite_data[0].clone());
    sprite_creator.map();
    let sprite = sprite_creator.build_from_name(String::from("snake_sprite"));
    let mut game_object = GameObject::new();
    game_object.sprite = sprite;

    'gameloop: loop {
        for event in sdl_manager.event_handle(){
            match event {
                Event::Quit{..} => {
                    break 'gameloop
                },
                _=> {}

            }
        }
        sdl_manager.render_clear();
        sdl_manager.copy_frame(game_object.update());
        sdl_manager.render_present();
    }
}
