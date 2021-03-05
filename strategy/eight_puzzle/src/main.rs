use std::io::*;
use std::collections::{VecDeque, HashSet};
use util::Scanner;
use puzzle::{Puzzle, Direction};

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

fn eight_puzzle(problem: &Vec<usize>) -> Vec<String> {
    let mut q: VecDeque<Puzzle> = VecDeque::new();
    let init = Puzzle::new(problem, 3);
    q.push_back(init.clone());

    let directions = vec![Direction::Up, Direction::Left, Direction::Down, Direction::Right];

    let mut v: HashSet<Puzzle> = HashSet::new();
    v.insert(init);

    while let Some(p) = q.pop_front() {
        if is_target(&p) { return p.path; }
        let space_location_x = p.space_idx as i32 / p.row_count;
        let space_location_y = p.space_idx as i32 % p.row_count;

        for d in directions.iter() {
            let target_tile_location_x = space_location_x + d.vertical_move();
            let target_tile_location_y = space_location_y + d.horizontal_move();
            if target_tile_location_x < 0 || target_tile_location_y < 0 || target_tile_location_x >= p.row_count || target_tile_location_y >= p.row_count { continue; }
            let mut u = p.clone();
            let new_space_idx = (target_tile_location_x * p.row_count + target_tile_location_y) as usize;
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
    unreachable!()
}

fn is_target(p: &Puzzle) -> bool {
    for i in 0..(p.f.len()) {
        if p.f[i] != (i + 1) % 9 { return false; }
    }
    true
}