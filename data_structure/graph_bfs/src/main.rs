use std::io::*;
use std::collections::VecDeque;
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

    let n = sc.read();

    let mut distances: Vec<usize> = vec![0; n];
    let mut statuses: Vec<Status> = vec![Status::Remaining; n];

    let mut matrix: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for _ in 0..n {
        let idx: usize = sc.read();
        let k: usize = sc.read();
        let vs: Vec<usize> = sc.vec(k);
        for &v in vs.iter() {
            matrix[idx - 1][v - 1] = true;
        }
    }

    bfs(&matrix, &mut distances, &mut statuses);
    for i in 0..n {
        println!("{} {}", i + 1, distances[i]);
    }
}

#[derive(Clone, PartialEq)]
enum Status {
    Done,
    Remaining,
}

fn bfs(matrix: &Vec<Vec<bool>>, distances: &mut Vec<usize>, statuses: &mut Vec<Status>) {
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for i in 0..matrix.len() {
            if matrix[u][i] && statuses[i] == Status::Remaining {
                q.push_back(i);
                distances[i] = distances[u] + 1;
            }
        }
        statuses[u] = Status::Done;
    }
}