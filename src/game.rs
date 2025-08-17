use raylib::prelude::*;
use crate::level::{load_level_from_file, Cell};
use crate::player::Player;
use crate::texture::Textures;
use crate::renderer::{render, SCREEN_W, SCREEN_H};
use raylib::audio::RaylibAudio;

enum GameState { Menu, Playing, Completed }

pub struct Game {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub grid: Option<Vec<Vec<Cell>>>,
    pub player: Option<Player>,
    pub textures: Textures,
    pub folders_collected: usize,
    state: GameState,
    levels: Vec<(String, String)>,
    selected_level_idx: usize,
    menu_bg: Option<Texture2D>,
    elapsed_time: f32,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let (mut rl, thread) = raylib::init().size(SCREEN_W, SCREEN_H).title("Sector Centinela").build();
        rl.set_target_fps(60);
        let textures = Textures::load_all(&mut rl, &thread);
        let levels = vec![
            ("Nivel 1".to_string(), "levels/level1.txt".to_string()),
            ("Nivel 2".to_string(), "levels/level2.txt".to_string()),
            ("Laberinto".to_string(), "levels/maze.txt".to_string()),
        ];
        let menu_bg = {
            let path = std::path::Path::new("textures/background.png");
            if path.exists() { rl.load_texture(&thread, &path.to_string_lossy()).ok() } else { None }
        };
        Ok(Self {
            rl,
            thread,
            grid: None,
            player: None,
            textures,
            folders_collected: 0,
            state: GameState::Menu,
            levels,
            selected_level_idx: 0,
            menu_bg,
            elapsed_time: 0.0,
        })
    }

    fn setup_level(&mut self, path: &str) -> Result<(), String> {
        let grid = load_level_from_file(path)?;
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
        self.grid = Some(grid);
        self.player = Some(player);
        self.folders_collected = 0;
        Ok(())
    }

    pub fn run(&mut self) {
        let mut prev = std::time::Instant::now();
        let cell_size = 64.0f32;
        let audio = RaylibAudio::init_audio_device().expect("No se pudo inicializar el audio");
        let music_path = "music/fondo.mp3";
        let mut music = audio.new_music(music_path).ok();
        if let Some(m) = music.as_mut() {
            m.set_volume(0.45);
            m.play_stream();
        }
        while !self.rl.window_should_close() {
            let now = std::time::Instant::now();
            let dt = (now - prev).as_secs_f32();
            prev = now;
            self.elapsed_time += dt;
            let mut input = crate::events::Input::new();
            input.poll(&mut self.rl);
            if let Some(m) = music.as_mut() {
                m.update_stream();
                if !m.is_stream_playing() {
                    m.seek_stream(0.0);
                    m.play_stream();
                }
            }
            if input.exit { break; }

            match self.state {
                GameState::Menu => {
                    if self.rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
                        self.selected_level_idx = (self.selected_level_idx + 1) % self.levels.len();
                    }
                    if self.rl.is_key_pressed(KeyboardKey::KEY_UP) {
                        if self.selected_level_idx == 0 { self.selected_level_idx = self.levels.len() - 1; } else { self.selected_level_idx -= 1; }
                    }
                    if self.rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                        let path = self.levels[self.selected_level_idx].1.clone();
                        if self.setup_level(&path).is_ok() { self.state = GameState::Playing; }
                    }
                    let mut d = self.rl.begin_drawing(&self.thread);
                    if let Some(bg) = &self.menu_bg {
                        d.draw_texture_pro(
                            bg,
                            Rectangle { x: 0.0, y: 0.0, width: bg.width() as f32, height: bg.height() as f32 },
                            Rectangle { x: 0.0, y: 0.0, width: SCREEN_W as f32, height: SCREEN_H as f32 },
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::WHITE,
                        );
                    } else {
                        d.clear_background(Color::new(10, 10, 20, 255));
                    }
                    d.draw_text("Selecciona un nivel:", 40, 260, 24, Color::WHITE);
                    let start_y = 300;
                    for (i, (name, _)) in self.levels.iter().enumerate() {
                        let y = start_y + i as i32 * 30;
                        let color = if i == self.selected_level_idx { Color::YELLOW } else { Color::LIGHTGRAY };
                        d.draw_text(name, 60, y, 24, color);
                    }
                    d.draw_text("Enter: Iniciar  •  Esc: Salir", 40, SCREEN_H - 24, 20, Color::WHITE);
                }
                GameState::Playing => {
                    if let (Some(grid), Some(player)) = (self.grid.as_mut(), self.player.as_mut()) {
                        input.apply_to_player(player, dt, grid, cell_size);
                        let gx = (player.x / cell_size).floor() as usize;
                        let gy = (player.y / cell_size).floor() as usize;
                        if gy < grid.len() && gx < grid[0].len() {
                            if grid[gy][gx] == Cell::Folder {
                                grid[gy][gx] = Cell::Floor;
                                self.folders_collected += 1;
                            }
                        }
                        let mut success = false;
                        if gy < grid.len() && gx < grid[0].len() { if grid[gy][gx] == Cell::Goal { success = true; } }
                        let fps = self.rl.get_fps() as i32;
                        let mut d = self.rl.begin_drawing(&self.thread);
                        render(&mut d, &self.textures, grid, player, self.folders_collected, fps, self.elapsed_time);
                        if success { self.state = GameState::Completed; }
                    } else {
                        self.state = GameState::Menu;
                    }
                }
                GameState::Completed => {
                    let back_to_menu = self.rl.is_key_pressed(KeyboardKey::KEY_ENTER);
                    let mut d = self.rl.begin_drawing(&self.thread);
                    d.clear_background(Color::new(0,0,0,255));
                    d.draw_text("¡Nivel completado!", 300, 200, 40, Color::GREEN);
                    d.draw_text(&format!("Folders recogidos: {}", self.folders_collected), 300, 260, 24, Color::WHITE);
                    d.draw_text("Enter: Menu  •  Esc: Salir", 300, 320, 20, Color::WHITE);
                    if back_to_menu { self.state = GameState::Menu; }
                }
            }
        }
    }
}
