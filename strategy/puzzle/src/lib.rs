use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};
use std::fmt;

pub trait Puzzle {
    const ROW_COUNT: usize;
    const TILE_COUNT: usize;
    fn generate(tiles: &Vec<usize>) -> Self;
    fn get_tiles(&self) -> Vec<usize>;
    fn get_path(&self) -> Vec<String>;
    fn get_move_count(&self) -> usize {
        self.get_path().len()
    }
    fn is_target(&self) -> bool {
        let tile_count = Self::TILE_COUNT;
        for i in 0..tile_count {
            if self.get_tiles()[i] !=  (i + 1) % tile_count { return false; }
        }
        true
    }
}

#[derive(Eq, PartialOrd, Clone, Hash)]
pub struct EightPuzzle {
    f: Vec<usize>,
    space_idx: usize,
    path: Vec<String>,
}

impl Puzzle for EightPuzzle {

    const ROW_COUNT: usize = 3;
    const TILE_COUNT: usize = 9;

    fn generate(tiles: &Vec<usize>) -> Self {
        if tiles.len() != Self::TILE_COUNT { panic!(); }
        let mut space_idx = 0;
        let mut f: Vec<usize> = Vec::new();
        for (i, &t) in tiles.iter().enumerate() {
            if t == 0 {
                space_idx = i;
            }
            f.push(t);
        }
        let path = Vec::new();
        EightPuzzle { f, space_idx, path }
    }

    fn get_path(&self) -> Vec<String> {
        self.path.clone()
    }

    fn get_tiles(&self) -> Vec<usize> {
        self.f.clone()
    }
}

impl EightPuzzle {
    pub fn solve(&self) -> Result<Self, &str> {
        let mut q: VecDeque<EightPuzzle> = VecDeque::new();
        q.push_back(self.clone());

        let directions = vec![Direction::Up, Direction::Left, Direction::Down, Direction::Right];
        
        let mut v: HashSet<EightPuzzle> = HashSet::new();
        v.insert(self.clone());
        
        while let Some(pzl) = q.pop_front() {
            if pzl.is_target() { return Ok(pzl); }
            let space_location_x = pzl.space_idx / Self::ROW_COUNT;
            let space_location_y = pzl.space_idx % Self::ROW_COUNT;
        
            for d in directions.iter() {
                let target_tile_location_x = space_location_x as i32 + d.vertical_move();
                let target_tile_location_y = space_location_y as i32 + d.horizontal_move();
                if target_tile_location_x < 0 || target_tile_location_y < 0 || target_tile_location_x >= Self::ROW_COUNT as i32|| target_tile_location_y >= Self::ROW_COUNT as i32 { continue; }
                let mut u = pzl.clone();
                let new_space_idx = (target_tile_location_x * Self::ROW_COUNT as i32 + target_tile_location_y) as usize;
                let old_tile = u.f[new_space_idx];
                u.f[pzl.space_idx] = old_tile;
                u.f[new_space_idx] = 0;
                u.space_idx = new_space_idx;
                match v.get(&u) {
                    None => {
                        v.insert(u.clone());
                        u.path.push(d.to_string());
                        q.push_back(u);
                    },
                    Some(_) => {}
                }
            }
        }
        Err("could not solve")
    }

}

impl PartialEq for EightPuzzle {
    fn eq(&self, other: &Self) -> bool {
        let self_tiles = self.get_tiles();
        let other_tiles = other.get_tiles();
        for i in 0..self_tiles.len() {
            if self_tiles[i] != other_tiles[i] { return false; }
        }
        true
    }

}

impl Ord for EightPuzzle {
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

#[derive(Eq, PartialOrd, Clone, Hash)]
pub struct FifteenPuzzle {
    f: Vec<usize>,
    space_idx: usize,
    path: Vec<String>,
}

impl Puzzle for FifteenPuzzle {

    const ROW_COUNT: usize = 4;
    const TILE_COUNT: usize = 16;

    fn generate(tiles: &Vec<usize>) -> Self {
        if tiles.len() != Self::TILE_COUNT { panic!(); }
        let mut space_idx = 0;
        let mut f: Vec<usize> = Vec::new();
        for (i, &t) in tiles.iter().enumerate() {
            if t == 0 {
                space_idx = i;
            }
            f.push(t);
        }
        let path = Vec::new();
        FifteenPuzzle { f, space_idx, path }
    }

    fn get_path(&self) -> Vec<String> {
        self.path.clone()
    }

    fn get_tiles(&self) -> Vec<usize> {
        self.f.clone()
    }

}

impl PartialEq for FifteenPuzzle {
    fn eq(&self, other: &Self) -> bool {
        let self_tiles = self.get_tiles();
        let other_tiles = other.get_tiles();
        for i in 0..self_tiles.len() {
            if self_tiles[i] != other_tiles[i] { return false; }
        }
        true
    }

}

impl Ord for FifteenPuzzle {
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
