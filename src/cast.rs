use crate::level::Cell;

pub struct RayHit {
    pub distance: f32,
    pub x: f32,
    pub y: f32,
    pub map_x: isize,
    pub map_y: isize,
    pub hit_vertical: bool,
}

pub fn cast_ray(grid: &Vec<Vec<Cell>>, cell_size: f32, px: f32, py: f32, angle: f32, max_dist: f32) -> Option<RayHit> {
    let step = 4.0;
    let mut rx = px;
    let mut ry = py;
    let dx = angle.cos() * step;
    let dy = angle.sin() * step;
    let rows = grid.len() as isize;
    let cols = if rows > 0 { grid[0].len() as isize } else { 0 };
    let mut traveled = 0.0;
    while traveled < max_dist {
        rx += dx;
        ry += dy;
        traveled += step;
        let cx = (rx / cell_size).floor() as isize;
        let cy = (ry / cell_size).floor() as isize;
        if cx < 0 || cy < 0 || cx >= cols || cy >= rows {
            return None;
        }
        match grid[cy as usize][cx as usize] {
            Cell::Floor | Cell::Goal | Cell::Folder | Cell::Guard1 | Cell::Guard2 => continue,
            _ => {
                let hit_vertical = (rx - cx as f32 * cell_size).abs() < (ry - cy as f32 * cell_size).abs();
                return Some(RayHit { 
                    distance: traveled, 
                    x: rx,
                    y: ry,
                    map_x: cx, 
                    map_y: cy, 
                    hit_vertical 
                });
            }
        }
    }
    None
}
