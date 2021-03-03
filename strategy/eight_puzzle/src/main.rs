use std::io::*;
use std::collections::VecDeque;
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
    println!("{}", eight_puzzle(&problem));
}

#[derive(Eq)]
struct Puzzle {
    f: Vec<usize>,
    space: usize,
    path: String,
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

fn eight_puzzle(problem: &Vec<usize>) -> usize {
    let q: VecDeque<Puzzle> = VecDeque::new();
    0
}

fn is_target(p: Puzzle) -> bool {
    for i in 0..p.f.len() {
        if p.f[i] != i + 1 { return false; }
    }
    true
}