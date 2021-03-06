use std::io::*;
use util::{Scanner, Joinable};
use puzzle::Puzzle;

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
    let puzzle = Puzzle::new(&problem);
    let ans = puzzle.solve_naive();
    println!("{}", ans.get_path().len());
}
