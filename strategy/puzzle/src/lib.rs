use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};
use std::fmt;
use util::Joinable;

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
    
    pub fn get_space_idx(&self) -> usize {
        match self {
            Self::EightPuzzle{space_idx, ..} => *space_idx,
            Self::FifteenPuzzle{space_idx, ..} => *space_idx,
        }
    }
    pub fn get_path(&self) -> Vec<String> {
        match self {
            Self::EightPuzzle{path, ..} => path.clone(),
            Self::FifteenPuzzle{path, ..} => path.clone(),
        }
    }

    fn push_path(&mut self, x: String) {
        match self {
            Self::EightPuzzle{path, ..} => path.push(x),
            Self::FifteenPuzzle{path, ..} => path.push(x),
        }
    }

    fn set_space_idx(&mut self, x: usize) {
        match self {
            Self::EightPuzzle{space_idx, ..} => {
                if space_idx <= &mut 9 {
                    *space_idx = x;
                } else {
                    panic!();
                }
            },
            Self::FifteenPuzzle{space_idx, ..} => {
                if space_idx <= &mut 16 {
                    *space_idx = x;
                } else {
                    panic!();
                }
            },
        }
    }
    fn set_new_tiles(&mut self, tiles: &Vec<usize>) {
        match self {
            Self::EightPuzzle{f, ..} => {
                if tiles.len() == 9 {
                    *f = tiles.clone();
                } else {
                    panic!();
                }
            },
            Self::FifteenPuzzle{f, ..} => {
                if tiles.len() == 16 {
                    *f = tiles.clone();
                } else {
                    panic!();
                }
            },
        }
    }

    pub fn solve_naive(&self) -> Self {
        match self {
            p @ Self::EightPuzzle {..} => {
                const ROW_COUNT: i32 = 3;
                let mut q: VecDeque<Puzzle> = VecDeque::new();
                q.push_back(p.clone());

                let directions = vec![Direction::Up, Direction::Left, Direction::Down, Direction::Right];
            
                let mut v: HashSet<Puzzle> = HashSet::new();
                v.insert(p.clone());
            
                while let Some(pzl) = q.pop_front() {
                    if pzl.is_target() { return pzl; }
                    let space_location_x = pzl.get_space_idx() / ROW_COUNT as usize;
                    let space_location_y = pzl.get_space_idx() % ROW_COUNT as usize;
            
                    for d in directions.iter() {
                        let target_tile_location_x = space_location_x as i32 + d.vertical_move();
                        let target_tile_location_y = space_location_y as i32 + d.horizontal_move();
                        if target_tile_location_x < 0 || target_tile_location_y < 0 || target_tile_location_x >= ROW_COUNT || target_tile_location_y >= ROW_COUNT { continue; }
                        let mut u = pzl.clone();
                        let new_space_idx = (target_tile_location_x * ROW_COUNT + target_tile_location_y) as usize;
                        let mut new_tiles = u.get_tiles();
                        let old_tile = new_tiles[new_space_idx];
                        new_tiles[pzl.get_space_idx()] = old_tile;
                        new_tiles[new_space_idx] = 0;
                        u.set_space_idx(new_space_idx);
                        u.set_new_tiles(&new_tiles);
                        match v.get(&u) {
                            None => {
                                v.insert(u.clone());
                                u.push_path(d.to_string());
                                q.push_back(u);
                            },
                            Some(_) => {}
                        }
                    }
                }
                unreachable!()
            },
            p @ Self::FifteenPuzzle {..} => {
                p.clone()
            }
        }
    }

    fn is_target(&self) -> bool {
        match self {
            Self::EightPuzzle {f, ..} => {
                for i in 0..9 {
                    if f[i] !=  (i + 1) % 9 { return false; }
                }
                true
            },
            Self::FifteenPuzzle {f, ..} => {
                for i in 0..16 {
                    if f[i] != (i + 1) % 16 { return false; }
                }
                true
            }
        }
    }

    fn row_count(&self) -> i32 {
        match self {
            Self::EightPuzzle {..} => 3,
            Self::FifteenPuzzle {..} => 4,
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
