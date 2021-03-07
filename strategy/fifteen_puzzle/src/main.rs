use std::io::*;
use util::Scanner;
use puzzle::{Puzzle, FifteenPuzzle};

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
        for _ in 0..16 {
            problem.push(sc.read());
        }
        problem
    };
    let puzzle = FifteenPuzzle::generate(&problem);
    let result = puzzle.solve_with_iterative_deepening_a_star().unwrap();
    println!("{}", result.get_move_count());
}
