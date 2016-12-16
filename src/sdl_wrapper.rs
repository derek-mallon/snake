use sdl2::{self,Sdl};
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2_image::{self,LoadTexture, INIT_PNG};
use geom::Vector2D;
use geom::Rect;
use std::path::Path;
use asset_manager::SdlConfigJson;
pub struct SdlManager<'a>{
    sdl_context:Sdl,
    image_context:sdl2_image::Sdl2ImageContext,
    renderer:sdl2::render::Renderer<'a>,
    video_subsystem:sdl2::VideoSubsystem,
    texture:sdl2::render::Texture,
    event_pump:sdl2::EventPump
}
impl<'a> SdlManager<'a>{
    pub fn new(config:SdlConfigJson)->SdlManager<'a>{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let image_context = sdl2_image::init(INIT_PNG).unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let mut renderer = video_subsystem.window(config.title.as_str(),config.window_width,config.window_height)
                .position_centered()
                .build().unwrap()
                .renderer()
                .build().unwrap();
        let texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24,256,256).unwrap();
        SdlManager{
            sdl_context:sdl_context,
            image_context:image_context,
            renderer:renderer,
            video_subsystem:video_subsystem,
            texture:texture,
            event_pump:event_pump
        }
    }
    pub fn load_texture(&mut self,path:&Path){
        self.texture = self.renderer.load_texture(path).unwrap();
    }
    pub fn copy_frame(&mut self,package:(Rect,Rect,f64,Vector2D<i32>,bool,bool)){
        self.renderer.copy_ex(&self.texture,Option::Some(rect_to_sdl_rect(package.0)),Option::Some(rect_to_sdl_rect(package.1)),package.2,Option::Some(vector2d_to_sdl_point(package.3)),package.4,package.5);
    }
    pub fn event_handle(&mut self)->sdl2::event::EventPollIterator{
         self.event_pump.poll_iter()
    }
    pub fn render_present(&mut self){
        self.renderer.present();
    }
    pub fn render_clear(&mut self){
        self.renderer.set_draw_color(Color::RGB(255,0,0));
        self.renderer.clear();
    }

}
//Helper functions
fn rect_to_sdl_rect(rect:Rect)->sdl2::rect::Rect{
    sdl2::rect::Rect::new(rect.x,rect.y,rect.width,rect.height)
}
fn vector2d_to_sdl_point(vec:Vector2D<i32>)->sdl2::rect::Point{
    sdl2::rect::Point::new(vec.x,vec.y)
}

