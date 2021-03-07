use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BinaryHeap};
use std::fmt;
use util::Joinable;

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

        let directions = Direction::all();
        
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

impl FifteenPuzzle {
    fn vertical_space_location(&self) -> usize {
        self.space_idx / Self::ROW_COUNT
    }
    fn horizontal_space_location(&self) -> usize {
        self.space_idx % Self::ROW_COUNT
    }
    fn vertical_target_tile_location(&self, dir: &Direction) -> i32 {
        self.vertical_space_location() as i32 + dir.vertical_move()
    }
    fn horizontal_target_tile_location(&self, dir: &Direction) -> i32 {
        self.horizontal_space_location() as i32 + dir.horizontal_move()
    }
    fn manhattan_distances() -> Vec<Vec<usize>> {
        (0..Self::TILE_COUNT as i32).map(|i| {
            (0..Self::TILE_COUNT as i32).map(|j| { 
                (i / Self::ROW_COUNT as i32 - j / Self::ROW_COUNT as i32).abs() as usize + (i % Self::ROW_COUNT as i32 - j % Self::ROW_COUNT as i32).abs() as usize
            })
            .collect::<Vec<_>>()
        })
        .collect()
    }
    pub fn solve_with_iterative_deepening_a_star(&self) -> Result<FifteenPuzzle, String> {
        let manhattan_distances = Self::manhattan_distances();
        let sum_md = self.sum_of_manhattan_distance(&Self::manhattan_distances());

        for limit in sum_md..100 {
            if let Ok(ans) = self.dfs(0, limit, &None, &manhattan_distances, &Direction::all()) {
                return Ok(ans)
            }
        }
        Err("unsolvable".to_string())
    }

    fn dfs(&self, depth: usize, limit: usize, prev: &Option<Direction>, mds: &Vec<Vec<usize>>, directions: &Vec<Direction>) -> Result<Self, String> {
        let md = self.sum_of_manhattan_distance(mds);
        if md == 0 { return Ok(self.clone()); }
        if depth + md > limit { return Err("answer not found within limit".to_string()) }

        for d in directions.iter() {
            let target_tile_location_x = self.vertical_target_tile_location(&d);
            let target_tile_location_y = self.horizontal_target_tile_location(&d);
            let target_tile_location = target_tile_location_x * Self::ROW_COUNT as i32 + target_tile_location_y;
            if target_tile_location_x < 0 || target_tile_location_y < 0 || target_tile_location_x >= Self::ROW_COUNT as i32 || target_tile_location_y >= Self::ROW_COUNT as i32 { continue; }
            if let Some(p) = prev { if d.is_opposite(p) { continue; } }
            let mut tmp = self.clone();

            let old_tile = tmp.f[target_tile_location as usize];
            tmp.f[self.space_idx] = old_tile;
            tmp.f[target_tile_location as usize] = 0;
            tmp.space_idx = target_tile_location as usize;
            if let Ok(pzl) = tmp.dfs(depth + 1, limit, &Some(d.clone()), mds, directions) {
                let mut ans = pzl.clone();
                ans.path.push(d.to_string());
                return Ok(ans)
            }
        }
        Err("answer not found".to_string())
    }

    /// manhattan_distances[i][j] は
    /// i 番目の要素が j だったときのマンハッタン距離を表している
    /// このマンハッタン距離は、配列が0, 1, 2, ..., 15 の順番で並んでいる時と比較した時の距離として設定されている
    /// self.f は 1, 2, 3, ..., 15, 0 と並んでいることを期待しているので、jにはself.f[i] - 1 を入れ, self.f[i] == 0の時はスキップする
    fn sum_of_manhattan_distance(&self, manhattan_distances: &Vec<Vec<usize>>) -> usize {
        let mut sum = 0;
        for i in 0..Self::TILE_COUNT {
            if self.f[i] == 0 { continue; }
            sum += manhattan_distances[i][self.f[i] - 1];
        }
        sum
    }

    pub fn solve_with_a_star(&self) -> Result<Self, String> {
        let mut pq: BinaryHeap<FifteenPuzzle> = BinaryHeap::new();
        let directions = Direction::all();
        let mds = Self::manhattan_distances();
        let mut v: HashSet<FifteenPuzzle> = HashSet::new();
        v.insert(self.clone());

        pq.push(self.clone());

        while let Some(pzl) = pq.pop() {
            println!("{}", pzl.f.iter().join(" "));
            if pzl.sum_of_manhattan_distance(&mds) == 0 { return Ok(pzl.clone()); }
            v.insert(pzl.clone());
            let space_idx_x = self.space_idx / Self::ROW_COUNT;
            let space_idx_y = self.space_idx % Self::ROW_COUNT;
            for d in directions.iter() {
                let vtt = space_idx_x as i32 + d.vertical_move();
                let htt = space_idx_y as i32 + d.horizontal_move();
                if vtt < 0 || htt < 0 || vtt >= Self::ROW_COUNT as i32|| htt >= Self::ROW_COUNT as i32 { continue; }
                let tt = (vtt * Self::ROW_COUNT as i32 + htt) as usize;
                let mut u = pzl.clone();
                let old_tile = u.f[tt as usize];
                u.f[pzl.space_idx] = old_tile;
                u.f[tt as usize] = 0;
                u.space_idx = tt;
                println!("vtt: {}, htt: {}, tt: {}, space_idx: {}, u.space: {}, old: {}", vtt, htt, tt, self.space_idx, u.space_idx, old_tile);

                match v.get(&u) {
                    None => {
                        println!("pushing: {}", u.f.iter().join(" "));
                        u.path.push(d.to_string());
                        pq.push(u.clone());
                    },
                    Some(_) => { }
                }
            }
        }
        Err("unsolvable".to_string())
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


#[derive(Clone)]
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

    pub fn all() -> Vec<Self> {
        vec![Self::Up, Self::Left, Self::Down, Self::Right]
    }

    pub fn is_opposite(&self, other: &Self) -> bool {
        match self {
            Self::Up => {
                match other {
                    Self::Down => true,
                    _ => false,
                }
            },
            Self::Down => {
                match other {
                    Self::Up => true,
                    _ => false,
                }
            },
            Self::Right => {
                match other {
                    Self::Left => true,
                    _ => false,
                }
            },
            Self::Left => {
                match other {
                    Self::Right => true,
                    _ => false,
                }
            }
        }
    }
}
