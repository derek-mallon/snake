use geom::Rect;
use asset_manager::SpriteDataJson;
use std::collections::HashMap;
use game_object::Sprite;
//A set of rects which represent frames.
//A postions in the set;
#[derive(Debug)]
pub struct Animation{
    frames:Vec<usize>
}
impl Animation{
    pub fn new(frames:Vec<usize>)->Animation{
        Animation{frames:frames}
    }
}
//A relative postion and group of animations and a position in the group of animations
#[derive(Debug)]
pub struct SpriteShell{
    animations:Vec<usize>,
}
impl SpriteShell{
    pub fn new(animations:Vec<usize>)->SpriteShell{
        SpriteShell{animations:animations}
    }
}

//Hold and updates the sprite_shell
pub struct SpriteCreator{
    frames:Vec<Rect>,
    animations:Vec<Animation>,
    sprite_shells:Vec<SpriteShell>,
    frame_map:HashMap<String,usize>,
    animation_map:HashMap<String,usize>,
    sprite_shell_map:HashMap<String,usize>,
    sprite_data_pages:Vec<SpriteDataJson>,
}
impl SpriteCreator{
    pub fn new(sprite_data_page:SpriteDataJson)->SpriteCreator{
        SpriteCreator{
            frames:Vec::new(),
            animations:Vec::new(),
            sprite_shells:Vec::new(),
            frame_map:HashMap::new(),
            animation_map:HashMap::new(),
            sprite_shell_map:HashMap::new(),
            sprite_data_pages:vec![sprite_data_page],
        }
    }
    pub fn map(&mut self){
        for data in &self.sprite_data_pages{
            //map frames
            for frame in &data.frames{
                self.frame_map.entry(frame.clone().id).or_insert(self.frames.len());
                self.frames.push(frame.clone().to_rect());
            }
            //map animations
            for animation in &data.animations{
                let mut frames = Vec::new();
                for id in animation.clone().frames{
                    match self.frame_map.get(&id){
                        Some(index) => {
                            frames.push(*index);
                        },
                        None => {}
                    }
                }
                self.animation_map.entry(animation.clone().id).or_insert(self.animations.len());
                self.animations.push(Animation::new(frames));
            }
            //map sprite_shells
            for sprite_shell in &data.sprites{
                let mut animations = Vec::new();
                for id in sprite_shell.clone().animations{
                    match self.animation_map.get(&id){
                        Some(index) => {
                            animations.push(*index);
                        },
                        None => {}
                    }
                }
                self.sprite_shell_map.entry(sprite_shell.clone().id).or_insert(self.sprite_shells.len());
                self.sprite_shells.push(SpriteShell::new(animations));
            }
        }
    }
    pub fn build(&mut self,index:usize)->Sprite{
        let mut frames = Vec::new();
        for animation in &self.sprite_shells[index].animations{
            for frame in &self.animations[*animation].frames{
                frames.push(self.frames[*frame]);
            }
        }
        let mut sprite = Sprite::new();
        sprite.frames = frames;
        sprite
    }
    pub fn build_from_name(&mut self,name:String)->Sprite{
        let index = *self.sprite_shell_map.get(&name).unwrap();
        let mut frames = Vec::new();
        for animation in &self.sprite_shells[index].animations{
            for frame in &self.animations[*animation].frames{
                frames.push(self.frames[*frame]);
            }
        }
        let mut sprite = Sprite::new();
        sprite.frames = frames;
        sprite
    }
}

