use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::time::{SystemTime};
use geom::Rect;
use rustc_serialize::json;
const PATH_TO_ASSET_LIST:&'static str = "new.json";
#[derive(Debug,Clone,Copy)]
pub enum AssetType{
    Texture,
    SpriteData,
    SdlConfig,
    AssetList
}
#[derive(Debug)]
pub struct AssetWithFile{
    type_of_asset:AssetType,
    last_time_modified:SystemTime,
    changed:bool,
    path:String,
    file:File
}
impl AssetWithFile{
    pub fn new(type_of_asset:AssetType,path:String)->AssetWithFile{
        let file = File::open(path.clone()).unwrap();
        let last_time_modified = file.metadata().unwrap().modified().unwrap();
        AssetWithFile{
            type_of_asset:type_of_asset,
            last_time_modified:last_time_modified,
            changed:false,
            file:file,
            path:path
        } 
    }
}
pub struct AssetManger{
    assets:Vec<AssetWithFile>,
    asset_list:AssetListJson,
    pub sprite_data:Vec<SpriteDataJson>,
    pub sdl_config:SdlConfigJson,
    pub texture_path:String,
}
impl AssetManger{
    pub fn new()->AssetManger{
        let mut assets = Vec::new();
        assets.push(AssetWithFile::new(AssetType::AssetList,String::from(PATH_TO_ASSET_LIST)));
        AssetManger{
            assets:assets,
            asset_list:AssetListJson::new(),
            sprite_data:Vec::new(),
            sdl_config:SdlConfigJson::new(),
            texture_path:String::new()
        }
    }
    pub fn load_asset_list(&mut self){
        self.generate_asset(0);
        for path in &self.asset_list.sprite_data {
            self.assets.push(AssetWithFile::new(AssetType::SpriteData,path.clone()));
        }
        self.assets.push(AssetWithFile::new(AssetType::SdlConfig,self.asset_list.sdl_config.clone()));
        self.assets.push(AssetWithFile::new(AssetType::Texture,self.asset_list.texture_path.clone()));
    }
    pub fn load_assets_all(&mut self){
        //exclude the first asset which is the asset list
        for i in 1..self.assets.len(){
            self.generate_asset(i);
        }
    }
    pub fn generate_asset(&mut self,index:usize){
        //non utf8 assets
        match self.assets[index].type_of_asset{
            AssetType::Texture =>{
                self.texture_path = self.assets[index].path.clone(); 
                return
            },
            _=> {}
        }
        //utf8 assets
        let mut contents = Vec::new();
        self.assets[index].file.read_to_end(&mut contents).unwrap();
        let mut contents = String::from_utf8(contents).unwrap();
        match self.assets[index].type_of_asset{
            AssetType::AssetList => {
                self.asset_list = json::decode(&contents).unwrap();
            },
            AssetType::SpriteData => {
                self.sprite_data.push(json::decode(&contents).unwrap());
            },
            AssetType::SdlConfig => {
                self.sdl_config = json::decode(&contents).unwrap();
            },
            _=>{}
        }
    }
}
#[derive(Debug,RustcDecodable,RustcEncodable,Clone)]
pub struct AssetListJson{
    sprite_data:Vec<String>,
    sdl_config:String,
    texture_path:String
}
impl AssetListJson{
    pub fn new()->AssetListJson{
        AssetListJson{
            sprite_data:Vec::new(),
            sdl_config:String::new(),
            texture_path:String::new()

        }
    }
}
#[derive(RustcDecodable,RustcEncodable,Clone,Debug)]
pub struct FrameJson{
    pub id:String,
    pub x:i32,
    pub y:i32,
    pub width:u32,
    pub height:u32
}
impl FrameJson{
    pub fn to_rect(self)->Rect{
        Rect::new(self.x,self.y,self.width,self.height)
    }
}

#[derive(RustcDecodable,RustcEncodable,Clone,Debug)]
pub struct AnimationsJson{
    pub id:String,
    pub frames:Vec<String>
}
#[derive(RustcDecodable,RustcEncodable,Clone,Debug)]
pub struct SpriteJson{
    pub id:String,
    pub animations:Vec<String>,
}
#[derive(RustcDecodable,RustcEncodable,Clone,Debug)]
pub struct SpriteDataJson{
    pub frames:Vec<FrameJson>,
    pub animations:Vec<AnimationsJson>,
    pub sprites:Vec<SpriteJson>,
}
#[derive(RustcEncodable,RustcDecodable,Clone)]
pub struct SdlConfigJson{
    pub title:String,
    pub window_width:u32,
    pub window_height:u32
}
impl SdlConfigJson{
    pub fn new()->SdlConfigJson{
        SdlConfigJson{
            title:String::new(),
            window_width:0,
            window_height:0,
        }
    }
}
