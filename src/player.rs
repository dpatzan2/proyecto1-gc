use crate::level::Cell;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub dir: f32,
    pub radius: f32,
    pub speed: f32,
    pub rot_speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            dir: 0.0,
            radius: 12.0,
            speed: 150.0,
            rot_speed: 2.5,
        }
    }

    pub fn try_move(&mut self, dx: f32, dy: f32, grid: &Vec<Vec<Cell>>, cell_size: f32) {
        let nx = self.x + dx;
        let ny = self.y + dy;
        if !circle_collides(nx, ny, self.radius, grid, cell_size) {
            self.x = nx;
            self.y = ny;
            return;
        }
        if !circle_collides(nx, self.y, self.radius, grid, cell_size) {
            self.x = nx;
            return;
        }
        if !circle_collides(self.x, ny, self.radius, grid, cell_size) {
            self.y = ny;
            return;
        }
    }
}

fn circle_collides(cx: f32, cy: f32, r: f32, grid: &Vec<Vec<Cell>>, cell_size: f32) -> bool {
    let rows = grid.len() as isize;
    let cols = if rows > 0 { grid[0].len() as isize } else { 0 };
    let min_col = ((cx - r) / cell_size).floor().max(0.0) as isize;
    let max_col = ((cx + r) / cell_size).floor().max(0.0) as isize;
    let min_row = ((cy - r) / cell_size).floor().max(0.0) as isize;
    let max_row = ((cy + r) / cell_size).floor().max(0.0) as isize;
    for ry in min_row..=max_row {
        for rx in min_col..=max_col {
            if ry < 0 || rx < 0 || ry >= rows || rx >= cols { continue; }
            match grid[ry as usize][rx as usize] {
                Cell::Floor | Cell::Goal | Cell::Folder | Cell::Guard1 | Cell::Guard2 => {},
                _ => {
                    let cell_cx = (rx as f32 + 0.5) * cell_size;
                    let cell_cy = (ry as f32 + 0.5) * cell_size;
                    let half = cell_size * 0.5;
                    let closest_x = cx.clamp(cell_cx - half, cell_cx + half);
                    let closest_y = cy.clamp(cell_cy - half, cell_cy + half);
                    let dx = cx - closest_x;
                    let dy = cy - closest_y;
                    if dx*dx + dy*dy < r*r {
                        return true;
                    }
                }
            }
        }
    }
    false
}
