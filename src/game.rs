use raylib::prelude::*;
use crate::level::{load_level_from_file, Cell};
use crate::player::Player;
use crate::texture::Textures;
use crate::renderer::{render, SCREEN_W, SCREEN_H};

pub struct Game {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub grid: Vec<Vec<Cell>>,
    pub player: Player,
    pub textures: Textures,
    pub folders_collected: usize,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let (mut rl, thread) = raylib::init().size(SCREEN_W, SCREEN_H).title("Raycaster - Rust + raylib").build();

        rl.set_target_fps(60);

        let grid = load_level_from_file("levels/level1.txt").map_err(|e| e)?;

        let mut spawn_x = 1usize;
        let mut spawn_y = 1usize;
        'outer: for (r, row) in grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell == Cell::Floor {
                    spawn_x = c;
                    spawn_y = r;
                    break 'outer;
                }
            }
        }
        let player = Player::new(spawn_x as f32 * 64.0 + 32.0, spawn_y as f32 * 64.0 + 32.0);
        let textures = Textures::load_all(&mut rl, &thread);

        Ok(Self {
            rl,
            thread,
            grid,
            player,
            textures,
            folders_collected: 0,
        })
    }

    pub fn run(&mut self) {
        let mut prev = std::time::Instant::now();
        let cell_size = 64.0f32;

        while !self.rl.window_should_close() {
            let now = std::time::Instant::now();
            let dt = (now - prev).as_secs_f32();
            prev = now;

            let mut input = crate::events::Input::new();
            input.poll(&mut self.rl);

            input.apply_to_player(&mut self.player, dt, &self.grid, cell_size);

            let gx = (self.player.x / cell_size).floor() as usize;
            let gy = (self.player.y / cell_size).floor() as usize;
            if gy < self.grid.len() && gx < self.grid[0].len() {
                if self.grid[gy][gx] == Cell::Folder {
                    self.grid[gy][gx] = Cell::Floor;
                    self.folders_collected += 1;
                }
            }

            let mut success = false;
            if gy < self.grid.len() && gx < self.grid[0].len() {
                if self.grid[gy][gx] == Cell::Goal {
                    success = true;
                }
            }

            let fps = self.rl.get_fps() as i32;
            let mut d = self.rl.begin_drawing(&self.thread);
            if success {
                d.clear_background(Color::new(0,0,0,255));
                d.draw_text("¡Nivel completado!", 300, 200, 40, Color::new(0,200,0,255));
                d.draw_text(&format!("Folders recogidos: {}", self.folders_collected), 300, 260, 24, Color::new(255,255,255,255));
                d.draw_text("Presiona ESC para salir", 300, 320, 20, Color::new(255,255,255,255));
                if let Some(k) = d.get_key_pressed() {
                    if k == KeyboardKey::KEY_ESCAPE { break; }
                }
                continue;
            }

            render(&mut d, &self.textures, &self.grid, &self.player, self.folders_collected, fps);

            if input.exit { break; }
        }
    }
}
