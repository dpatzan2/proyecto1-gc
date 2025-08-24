use raylib::prelude::*;
use crate::player::Player;
use crate::level::Cell;
use crate::cast;
use crate::texture::Textures;

pub const SCREEN_W: i32 = 1024;
pub const SCREEN_H: i32 = 640;
pub const FOV: f32 = std::f32::consts::FRAC_PI_3;
const RAY_STRIDE: usize = 2;

pub fn render(
    d: &mut RaylibDrawHandle,
    textures: &Textures,
    grid: &Vec<Vec<Cell>>,
    player: &Player,
    folders_collected: usize,
    fps: i32,

    elapsed_time: f32,
    guards: &Vec<crate::guard::Guard>,
) {
    let cell_size = 64.0f32;
    d.clear_background(Color::new(100, 150, 255, 255));
    
    let floor_tile_size = 128;
    let tiles_x = (SCREEN_W + floor_tile_size - 1) / floor_tile_size;
    let tiles_y = (SCREEN_H/2 + floor_tile_size - 1) / floor_tile_size;
    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            let x = tx * floor_tile_size;
            let y = SCREEN_H/2 + ty * floor_tile_size;
            let dest_rect = Rectangle::new(x as f32, y as f32, floor_tile_size as f32, floor_tile_size as f32);
            let src_rect = Rectangle::new(0.0, 0.0, textures.floor.width as f32, textures.floor.height as f32);
            d.draw_texture_pro(&textures.floor, src_rect, dest_rect, Vector2::zero(), 0.0, Color::new(100,100,100,255));
        }
    }

  
    overlay_goal_floor(d, grid, player, cell_size);

    let screen_w = SCREEN_W as usize;
    for x in (0..screen_w).step_by(RAY_STRIDE) {
        let camera_x = 2.0 * x as f32 / screen_w as f32 - 1.0;
        let ray_angle = player.dir + camera_x * (FOV / 2.0);
        if let Some(hit) = cast::cast_ray(grid, cell_size, player.x, player.y, ray_angle, 2000.0) {
            let corrected = hit.distance * (ray_angle - player.dir).cos().abs().max(0.0001);
            let mut line_height = (cell_size * (SCREEN_H as f32) / corrected) as i32;
            if line_height > SCREEN_H { line_height = SCREEN_H; }
            let draw_x = x as i32;
            let start_y = (SCREEN_H as i32 - line_height) / 2;
            
            let tex = if ((hit.map_x + hit.map_y) % 2) == 0 {
                &textures.wall1
            } else {
                &textures.wall2
            };
            
            let wall_x = if hit.hit_vertical { hit.y } else { hit.x };
            let tex_x = ((wall_x % cell_size) * tex.width as f32 / cell_size) as i32;
            
            let tint = if hit.hit_vertical { 
                Color::new(255,255,255,255) 
            } else { 
                Color::new(200,200,200,255) 
            };
            
            let slice_w = (RAY_STRIDE.min(screen_w - x)) as f32;
            let src_rect = Rectangle::new(tex_x as f32, 0.0, 1.0, tex.height as f32);
            let dest_rect = Rectangle::new(draw_x as f32, start_y as f32, slice_w, line_height as f32);
            d.draw_texture_pro(tex, src_rect, dest_rect, Vector2::zero(), 0.0, tint);
        }
    }

    draw_minimap(d, grid, player, guards);
    draw_world_sprites(d, grid, cell_size, player, textures, elapsed_time, guards);
    d.draw_text(&format!("Folders: {}", folders_collected), 10, SCREEN_H - 28, 20, Color::new(255,255,255,255));
    d.draw_text(&format!("FPS: {}", fps), SCREEN_W - 100, 10, 20, Color::new(255,255,255,255));
}

fn overlay_goal_floor(d: &mut RaylibDrawHandle, grid: &Vec<Vec<Cell>>, player: &Player, cell_size: f32) {

    let dir_x = player.dir.cos();
    let dir_y = player.dir.sin();
    let plane_len = (FOV * 0.5).tan();
    let plane_x = -dir_y * plane_len;
    let plane_y =  dir_x * plane_len;

    let ray0_x = dir_x - plane_x;
    let ray0_y = dir_y - plane_y;
    let ray1_x = dir_x + plane_x;
    let ray1_y = dir_y + plane_y;

    let w = SCREEN_W as f32;
    let h = SCREEN_H as f32;
    let pos_z = h * 0.5; 

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;


    for y in (SCREEN_H/2 + 1)..SCREEN_H {
        let p = (y - SCREEN_H/2) as f32;
        let row_dist = pos_z / p; 

        let mut floor_x = player.x + row_dist * ray0_x;
        let mut floor_y = player.y + row_dist * ray0_y;
        let step_x = row_dist * (ray1_x - ray0_x) / w;
        let step_y = row_dist * (ray1_y - ray0_y) / w;

        let stride = RAY_STRIDE as i32;
        let mut x = 0i32;
        while x < SCREEN_W {
            let cell_x = (floor_x / cell_size) as i32;
            let cell_y = (floor_y / cell_size) as i32;

            if cell_x >= 0 && cell_y >= 0 && cell_y < rows && cell_x < cols {
                if grid[cell_y as usize][cell_x as usize] == Cell::Goal {
                    // Draw a small horizontal segment covering the stride at this scanline
                    d.draw_rectangle(x, y, stride.min(SCREEN_W - x), 1, Color::GOLD);
                }
            }

            floor_x += step_x * (RAY_STRIDE as f32);
            floor_y += step_y * (RAY_STRIDE as f32);
            x += stride;
        }
    }
}

fn draw_minimap(d: &mut RaylibDrawHandle, grid: &Vec<Vec<Cell>>, player: &Player, guards: &Vec<crate::guard::Guard>) {
    let cols = grid[0].len() as i32;
    let rows = grid.len() as i32;
    let scale: i32 = 10;
    let pad: i32 = 6;
    let map_w = cols * scale;
    let map_h = rows * scale;
    let panel_x = 10;
    let panel_y = 10;
    let panel_w = map_w + pad * 2;
    let panel_h = map_h + pad * 2;

    d.draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::new(0, 0, 0, 170));
    d.draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, Color::new(255, 255, 255, 120));

    let tile_padding = 1;
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = match cell {
                Cell::Wall => Color::DARKGRAY,
                Cell::Goal => Color::GOLD,
                Cell::Folder => Color::LIME,
                _ => Color::GRAY,
            };
            let rx = panel_x + pad + (x as i32) * scale + tile_padding;
            let ry = panel_y + pad + (y as i32) * scale + tile_padding;
            let sz = scale - tile_padding * 2;
            if sz > 0 {
                d.draw_rectangle(rx, ry, sz, sz, Color::new(color.r, color.g, color.b, 220));
            }
        }
    }

    // draw guards from dynamic list
    for g in guards.iter() {
        let gx = panel_x as f32 + pad as f32 + (g.x / 64.0) * scale as f32;
        let gy = panel_y as f32 + pad as f32 + (g.y / 64.0) * scale as f32;
        let s = 3;
        d.draw_rectangle(gx as i32 - s, gy as i32 - s, s*2, s*2, Color::RED);
    }

    let px = panel_x as f32 + pad as f32 + (player.x / 64.0) * scale as f32;
    let py = panel_y as f32 + pad as f32 + (player.y / 64.0) * scale as f32;
    let dir_x = player.dir.cos();
    let dir_y = player.dir.sin();
    let size = scale as f32 * 0.7;
    let tip = Vector2 { x: px + dir_x * size, y: py + dir_y * size };

    let ang = 140f32.to_radians();
    let cos_a = ang.cos();
    let sin_a = ang.sin();
    let lx = dir_x * cos_a - dir_y * sin_a;
    let ly = dir_x * sin_a + dir_y * cos_a;
    let rx = dir_x * cos_a + dir_y * sin_a;
    let ry = -dir_x * sin_a + dir_y * cos_a;

    let base_len = size * 0.7;
    let left = Vector2 { x: px + lx * base_len, y: py + ly * base_len };
    let right = Vector2 { x: px + rx * base_len, y: py + ry * base_len };

    d.draw_triangle(left, tip, right, Color::BLUE);
    d.draw_triangle_lines(left, tip, right, Color::new(255, 255, 255, 180));

    let fov_len = (scale * 6) as f32;
    let fov_half = FOV / 2.0;
    let dir_lx = (player.dir - fov_half).cos();
    let dir_ly = (player.dir - fov_half).sin();
    let dir_rx = (player.dir + fov_half).cos();
    let dir_ry = (player.dir + fov_half).sin();
    let l_end = Vector2 { x: px + dir_lx * fov_len, y: py + dir_ly * fov_len };
    let r_end = Vector2 { x: px + dir_rx * fov_len, y: py + dir_ry * fov_len };
    d.draw_line(px as i32, py as i32, l_end.x as i32, l_end.y as i32, Color::new(255, 255, 255, 140));
    d.draw_line(px as i32, py as i32, r_end.x as i32, r_end.y as i32, Color::new(255, 255, 255, 140));
}

// Tipo de animación por sprite
#[derive(Clone, Copy)]
enum SpriteAnim { Guard, Folder }

fn draw_world_sprites(d: &mut RaylibDrawHandle, grid: &Vec<Vec<Cell>>, cell_size: f32, player: &Player, textures: &Textures, elapsed_time: f32, guards: &Vec<crate::guard::Guard>) {
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut sprites: Vec<(f32, f32, &raylib::texture::Texture2D, SpriteAnim)> = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            match grid[r][c] {
                Cell::Guard1 => {
                    let sx = (c as f32 + 0.5) * cell_size;
                    let sy = (r as f32 + 0.5) * cell_size;
                    sprites.push((sx, sy, &textures.guard1, SpriteAnim::Guard));
                }
                Cell::Guard2 => {
                    let sx = (c as f32 + 0.5) * cell_size;
                    let sy = (r as f32 + 0.5) * cell_size;
                    sprites.push((sx, sy, &textures.guard2, SpriteAnim::Guard));
                }
                Cell::Folder => {
                    let sx = (c as f32 + 0.5) * cell_size;
                    let sy = (r as f32 + 0.5) * cell_size;
                    sprites.push((sx, sy, &textures.folder, SpriteAnim::Folder));
                }
                _ => {}
            }
        }
    }

    // Add dynamic guards as sprites
    for g in guards.iter() {
        match g.kind {
            crate::level::Cell::Guard1 => sprites.push((g.x, g.y, &textures.guard1, SpriteAnim::Guard)),
            crate::level::Cell::Guard2 => sprites.push((g.x, g.y, &textures.guard2, SpriteAnim::Guard)),
            _ => {}
        }
    }

    sprites.sort_by(|a, b| {
        let da = ((a.0 - player.x).powi(2) + (a.1 - player.y).powi(2)).partial_cmp(&((b.0 - player.x).powi(2) + (b.1 - player.y).powi(2))).unwrap_or(std::cmp::Ordering::Equal)
            .reverse();
        da
    });

    for (i, (sx, sy, tex, kind)) in sprites.iter().enumerate() {
        if i > 64 { break; }
        draw_sprite_animated(d, grid, cell_size, player, *sx, *sy, tex, *kind, elapsed_time);
    }
}

fn has_wall_between(grid: &Vec<Vec<Cell>>, cell_size: f32, player_x: f32, player_y: f32, sprite_x: f32, sprite_y: f32) -> bool {
    let dx = sprite_x - player_x;
    let dy = sprite_y - player_y;
    let distance = (dx * dx + dy * dy).sqrt();
    
    let steps = (distance / (cell_size * 0.25)).max(1.0) as i32;
    
    for i in 1..steps {
        let t = i as f32 / steps as f32;
        let check_x = player_x + dx * t;
        let check_y = player_y + dy * t;
        
        let grid_x = (check_x / cell_size) as usize;
        let grid_y = (check_y / cell_size) as usize;
        
        if grid_y < grid.len() && grid_x < grid[0].len() {
            if grid[grid_y][grid_x] == Cell::Wall {
                return true;
            }
        }
    }
    
    false
}

fn draw_sprite_animated(d: &mut RaylibDrawHandle, grid: &Vec<Vec<Cell>>, cell_size: f32, player: &Player, sx: f32, sy: f32, tex: &raylib::texture::Texture2D, kind: SpriteAnim, elapsed: f32) {
    let dx = sx - player.x;
    let dy = sy - player.y;
    let dist = (dx*dx + dy*dy).sqrt();
    
    if dist < 8.0 { return; }
    
    let sprite_angle = dy.atan2(dx);
    let mut relative_angle = sprite_angle - player.dir;
    while relative_angle > std::f32::consts::PI { relative_angle -= std::f32::consts::TAU; }
    while relative_angle < -std::f32::consts::PI { relative_angle += std::f32::consts::TAU; }

    let half_fov = FOV / 2.0;
    if relative_angle.abs() > half_fov { return; }

    if has_wall_between(grid, cell_size, player.x, player.y, sx, sy) {
        return;
    }
    
    let screen_x = ((relative_angle + half_fov) / FOV) * (SCREEN_W as f32);
    
    // Altura base por perspectiva
    let mut sprite_height = (64.0 * (SCREEN_H as f32) / dist) as i32;
    sprite_height = sprite_height.min(SCREEN_H * 2);

    // Animaciones simples usando seno: bobbing y pulso de escala
    let phase = (sx + sy) * 0.05; // desfasar por posición para evitar sincronía perfecta
    let (scale_mul, y_bob) = match kind {
        SpriteAnim::Folder => {
            // Más notorio para resaltar que es coleccionable
            let s = (elapsed * 3.0 + phase).sin();
            (1.0 + 0.20 * s, (elapsed * 4.0 + phase).sin() * 6.0)
        }
        SpriteAnim::Guard => {
            // Sutil respiración
            let s = (elapsed * 2.0 + phase).sin();
            (1.0 + 0.05 * s, (elapsed * 2.0 + phase).sin() * 3.0)
        }
    };

    let sprite_height = (sprite_height as f32 * scale_mul) as i32;
    let mut sprite_y = (SCREEN_H - sprite_height) / 2;
    sprite_y += y_bob as i32;
    let sprite_x = screen_x as i32 - (sprite_height / 2);
    
    if sprite_x + sprite_height < 0 || sprite_x >= SCREEN_W { return; }
    
    let scale = sprite_height as f32 / tex.height as f32;
    d.draw_texture_ex(
        tex, 
        Vector2::new(sprite_x as f32, sprite_y as f32), 
        0.0, 
        scale, 
        Color::WHITE
    );
}
