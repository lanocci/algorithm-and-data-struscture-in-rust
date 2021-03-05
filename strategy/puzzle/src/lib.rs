use std::cmp::Ordering;
use std::fmt;

#[derive(Eq, Clone, Hash)]
pub struct Puzzle {
    pub f: Vec<usize>,
    pub space_idx: usize,
    pub path: Vec<String>,
    pub row_count: i32,
}

impl Puzzle {
    pub fn new(tiles: &Vec<usize>, row_count: i32) -> Self {
        let mut space_idx = 0;
        let mut f: Vec<usize> = Vec::new();
        for (i, &t) in tiles.iter().enumerate() {
            if t == 0 {
                space_idx = i;
            }
            f.push(t);
        }
        let path = Vec::new();
        Self { f, space_idx, path, row_count }
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.f.len() {
            if self.f[i] != other.f[i] { return false; }
        }
        true
    }
}

impl PartialOrd for Puzzle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Puzzle {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.f.len() {
            if self.f[i] == other.f[i] { continue; }
            else if self.f[i] > other.f[i] { return Ordering::Greater; }
            else if self.f[i] < other.f[i] { return Ordering::Less; }
        }
        Ordering::Equal
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Up => f.write_str("u")?,
            Self::Down => f.write_str("d")?,
            Self::Left => f.write_str("l")?,
            Self::Right => f.write_str("r")?,
        }
        Ok(())
    }
}

impl Direction {
    pub fn horizontal_move(&self) -> i32 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
            _ => 0
            
        }
    }

    pub fn vertical_move(&self) -> i32 {
        match self {
            Self::Up => -1,
            Self::Down => 1,
            _ => 0
        }
    }
}
