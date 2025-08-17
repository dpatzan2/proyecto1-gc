use raylib::prelude::*;

pub struct Textures {
    pub floor: Texture2D,
    pub wall1: Texture2D,
    pub wall2: Texture2D,
    pub guard1: Texture2D,
    pub guard2: Texture2D,
    pub folder: Texture2D,
}

impl Textures {
    pub fn load_all(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let floor = rl.load_texture(thread, "textures/floor.jpg").unwrap_or_else(|e| {
            println!("Warning: Failed to load floor texture: {}", e);
            let img = Image::gen_image_color(1,1, Color::WHITE);
            rl.load_texture_from_image(thread, &img).unwrap()
        });
        
        let wall1 = rl.load_texture(thread, "textures/wall1.jpg").unwrap_or_else(|e| {
            println!("Warning: Failed to load wall1 texture: {}", e);
            let img = Image::gen_image_color(1,1, Color::BROWN);
            rl.load_texture_from_image(thread, &img).unwrap()
        });
        
        let wall2 = rl.load_texture(thread, "textures/wall2.jpg").unwrap_or_else(|e| {
            println!("Warning: Failed to load wall2 texture: {}", e);
            let img = Image::gen_image_color(1,1, Color::DARKGRAY);
            rl.load_texture_from_image(thread, &img).unwrap()
        });
        
        let guard1 = rl.load_texture(thread, "textures/guard1.png").unwrap_or_else(|e| {
            println!("Warning: Failed to load guard1 texture: {}", e);
            let img = Image::gen_image_color(1,1, Color::RED);
            rl.load_texture_from_image(thread, &img).unwrap()
        });
        
        let guard2 = rl.load_texture(thread, "textures/guard2.png").unwrap_or_else(|e| {
            println!("Warning: Failed to load guard2 texture: {}", e);
            let img = Image::gen_image_color(1,1, Color::PINK);
            rl.load_texture_from_image(thread, &img).unwrap()
        });
        
        let folder = rl.load_texture(thread, "textures/folder.jpg").unwrap_or_else(|e| {
            println!("Warning: Failed to load folder texture: {}", e);
            let img = Image::gen_image_color(1,1, Color::YELLOW);
            rl.load_texture_from_image(thread, &img).unwrap()
        });
        
        Self { floor, wall1, wall2, guard1, guard2, folder }
    }
}
