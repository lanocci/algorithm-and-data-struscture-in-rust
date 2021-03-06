use std::cmp::Ordering;
use std::fmt;

#[derive(Eq, Clone, Hash)]
pub enum Puzzle {
    EightPuzzle {
        f: Vec<usize>,
        space_idx: usize,
        path: Vec<String>,
    },
    FifteenPuzzle {
        f: Vec<usize>,
        space_idx: usize,
        path: Vec<String>,
    }
}

impl Puzzle {
    pub fn new(tiles: &Vec<usize>) -> Self {
        let mut space_idx = 0;
        let mut f: Vec<usize> = Vec::new();
        for (i, &t) in tiles.iter().enumerate() {
            if t == 0 {
                space_idx = i;
            }
            f.push(t);
        }
        let path = Vec::new();
        match tiles.len() {
            9 => Self::EightPuzzle { f, space_idx, path },
            15 => Self::FifteenPuzzle { f, space_idx, path },
            _ => panic!()
        }
    }

    pub fn get_tiles(&self) -> Vec<usize> {
        match self {
            Self::EightPuzzle{f,..} => f.clone(),
            Self::FifteenPuzzle{f,..} => f.clone(),
        }
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        let self_tiles = self.get_tiles();
        let other_tiles = other.get_tiles();
        for i in 0..self_tiles.len() {
            if self_tiles[i] != other_tiles[i] { return false; }
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
        let self_tiles = self.get_tiles();
        let other_tiles = other.get_tiles();
        for i in 0..self_tiles.len() {
            if self_tiles[i] == other_tiles[i] { continue; }
            else if self_tiles[i] > other_tiles[i] { return Ordering::Greater; }
            else if self_tiles[i] < other_tiles[i] { return Ordering::Less; }
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
