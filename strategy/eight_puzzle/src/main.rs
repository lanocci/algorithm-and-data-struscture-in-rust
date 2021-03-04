use std::io::*;
use std::fmt;
use std::collections::{VecDeque, HashSet};
use std::cmp::Ordering;
use util::Scanner;

fn main() {
    std::thread::Builder::new()
        .stack_size(1048576)
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}

fn solve() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let problem = {
        let mut problem: Vec<usize> = Vec::new();
        for _ in 0..9 {
            problem.push(sc.read());
        }
        problem
    };
    println!("{}", eight_puzzle(&problem).len());
}

#[derive(Eq, Clone, Hash)]
struct Puzzle {
    f: Vec<usize>,
    space_idx: usize,
    path: Vec<String>,
}

impl Puzzle {
    const ROW_COUNT: i32 = 3;
    const TILE_COUNT: i32 = 9;
    fn new(tiles: &Vec<usize>) -> Self {
        let mut space_idx = 0;
        let mut f: Vec<usize> = Vec::new();
        for &t in tiles.iter() {
            if t == 0 {
                space_idx = t;
            }
            f.push(t);
        }
        let path = Vec::new();
        Self { f, space_idx, path }
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

enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Directions {
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

impl Directions {
    fn horizontal_move(&self) -> i32 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
            _ => 0
            
        }
    }

    fn vertical_move(&self) -> i32 {
        match self {
            Self::Up => -1,
            Self::Down => 1,
            _ => 0
        }
    }
}

fn eight_puzzle(problem: &Vec<usize>) -> Vec<String> {
    let mut q: VecDeque<Puzzle> = VecDeque::new();
    let init = Puzzle::new(problem);
    q.push_back(init.clone());

    let directions = vec![Directions::Up, Directions::Left, Directions::Down, Directions::Right];

    let mut v: HashSet<Puzzle> = HashSet::new();
    v.insert(init);
    let mut ans: Vec<String> = Vec::new();

    while let Some(p) = q.pop_front() {
        if is_target(&p) { return (p.path); }
        let space_location_x = p.space_idx as i32 / Puzzle::ROW_COUNT;
        let space_location_y = p.space_idx as i32 % Puzzle::ROW_COUNT;

        for d in directions.iter() {
            let target_tile_location_x = space_location_x + d.vertical_move();
            let target_tile_location_y = space_location_y + d.horizontal_move();
            if target_tile_location_x < 0 || target_tile_location_y < 0 || target_tile_location_x >= Puzzle::ROW_COUNT || target_tile_location_y >= Puzzle::ROW_COUNT { continue; }
            let mut u = p.clone();
            let new_space_idx = (target_tile_location_x * Puzzle::ROW_COUNT + target_tile_location_y) as usize;
            let old_tile = u.f[new_space_idx];
            u.f[p.space_idx] = old_tile;
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
    ans
}

fn is_target(p: &Puzzle) -> bool {
    for i in 0..p.f.len() {
        if p.f[i] != i + 1 { return false; }
    }
    true
}