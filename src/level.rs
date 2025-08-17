use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Floor,
    Wall,
    Goal,
    Guard1,
    Guard2,
    Folder,
    Unknown,
}

pub fn load_level_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<Cell>>, String> {
    let s = fs::read_to_string(path).map_err(|e| format!("Error reading level: {}", e))?;
    let mut lines: Vec<String> = s.lines().map(|l| l.trim_end_matches('\r').to_string()).collect();
    let maxw = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    for line in &mut lines {
        if line.len() < maxw {
            line.push_str(&" ".repeat(maxw - line.len()));
        }
    }
    let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(lines.len());
    for line in lines {
        let mut row = Vec::with_capacity(maxw);
        for ch in line.chars() {
            let cell = match ch {
                ' ' => Cell::Floor,
                '+' | '-' | '|' => Cell::Wall,
                'g' => Cell::Goal,
                'G' => Cell::Guard1,
                'H' => Cell::Guard2,
                'f' => Cell::Folder,
                _ => Cell::Unknown,
            };
            row.push(cell);
        }
        grid.push(row);
    }
    Ok(grid)
}
