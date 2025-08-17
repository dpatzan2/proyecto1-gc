use raylib::prelude::*;
use crate::player::Player;

pub struct Input {
    pub move_forward: bool,
    pub move_back: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub turn_left: bool,
    pub turn_right: bool,
    pub mouse_dx: f32,
    pub mouse_down: bool,
    pub exit: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            move_forward: false,
            move_back: false,
            move_left: false,
            move_right: false,
            turn_left: false,
            turn_right: false,
            mouse_dx: 0.0,
            mouse_down: false,
            exit: false,
        }
    }

    pub fn poll(&mut self, rl: &mut RaylibHandle) {
        self.move_forward = rl.is_key_down(KeyboardKey::KEY_W);
        self.move_back = rl.is_key_down(KeyboardKey::KEY_S);
        self.move_left = rl.is_key_down(KeyboardKey::KEY_A);
        self.move_right = rl.is_key_down(KeyboardKey::KEY_D);
        self.turn_left = rl.is_key_down(KeyboardKey::KEY_LEFT);
        self.turn_right = rl.is_key_down(KeyboardKey::KEY_RIGHT);
        self.exit = rl.is_key_down(KeyboardKey::KEY_ESCAPE);
        let md = rl.get_mouse_delta();
        self.mouse_dx = md.x as f32;
        self.mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
    }

    pub fn apply_to_player(&self, player: &mut Player, dt: f32, grid: &Vec<Vec<crate::level::Cell>>, cell_size: f32) {
        let mut rot = 0.0;
        if self.turn_left { rot += player.rot_speed * dt; }
        if self.turn_right { rot -= player.rot_speed * dt; }
        
        if self.mouse_down {
            rot += -self.mouse_dx * 0.003;
        }
        
        player.dir += rot;

        let mut mvx = 0.0f32;
        let mut mvy = 0.0f32;
        if self.move_forward {
            mvx += player.dir.cos() * player.speed * dt;
            mvy += player.dir.sin() * player.speed * dt;
        }
        if self.move_back {
            mvx -= player.dir.cos() * player.speed * dt;
            mvy -= player.dir.sin() * player.speed * dt;
        }
        if self.move_left {
            mvx += (player.dir - std::f32::consts::FRAC_PI_2).cos() * player.speed * dt;
            mvy += (player.dir - std::f32::consts::FRAC_PI_2).sin() * player.speed * dt;
        }
        if self.move_right {
            mvx += (player.dir + std::f32::consts::FRAC_PI_2).cos() * player.speed * dt;
            mvy += (player.dir + std::f32::consts::FRAC_PI_2).sin() * player.speed * dt;
        }

        player.try_move(mvx, mvy, grid, cell_size);
    }
}
