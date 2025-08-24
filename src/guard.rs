use crate::level::Cell;
use crate::player::circle_collides;
use crate::cast;

pub struct Guard {
    pub x: f32,
    pub y: f32,
    pub kind: Cell,
    pub speed: f32,
    pub radius: f32,
}

impl Guard {
    pub fn new(x: f32, y: f32, kind: Cell) -> Self {
        let speed = match kind {
            Cell::Guard1 => 70.0,
            Cell::Guard2 => 90.0,
            _ => 60.0,
        };
        Self { x, y, kind, speed, radius: 12.0 }
    }

    pub fn update(&mut self, player_x: f32, player_y: f32, dt: f32, grid: &Vec<Vec<Cell>>, cell_size: f32) {
    
        let dx = player_x - self.x;
        let dy = player_y - self.y;
        let dist = (dx*dx + dy*dy).sqrt();
        if dist < 1.0 { return; }
        let angle = dy.atan2(dx);
        let hit = cast::cast_ray(grid, cell_size, self.x, self.y, angle, dist as f32);
        if hit.is_some() {
            return;
        }

        let dist = dist.max(0.0001) as f32;
        let vx = dx as f32 / dist * self.speed * dt;
        let vy = dy as f32 / dist * self.speed * dt;

        if !circle_collides(self.x + vx, self.y + vy, self.radius, grid, cell_size) {
            self.x += vx;
            self.y += vy;
            return;
        }


        if !circle_collides(self.x + vx, self.y, self.radius, grid, cell_size) {
            self.x += vx;
            return;
        }
        if !circle_collides(self.x, self.y + vy, self.radius, grid, cell_size) {
            self.y += vy;
            return;
        }

        let jiggle = 6.0 * dt;
        if !circle_collides(self.x + jiggle, self.y, self.radius, grid, cell_size) { self.x += jiggle; }
        else if !circle_collides(self.x - jiggle, self.y, self.radius, grid, cell_size) { self.x -= jiggle; }
        else if !circle_collides(self.x, self.y + jiggle, self.radius, grid, cell_size) { self.y += jiggle; }
        else if !circle_collides(self.x, self.y - jiggle, self.radius, grid, cell_size) { self.y -= jiggle; }
    }
}
