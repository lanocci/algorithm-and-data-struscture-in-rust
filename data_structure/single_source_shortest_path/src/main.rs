use std::io::*;
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

    let n: usize = sc.read();

    let mut matrix: Vec<Vec<usize>> = vec![vec![usize::MAX; n]; n];
    
    for _ in 0..n {
        let idx: usize = sc.read();
        let k = sc.read();
        for _ in 0..k {
            let v: usize = sc.read();
            let c: usize = sc.read();
            matrix[idx][v] = c;
        }
    }

    let d = dijkstra(&matrix, 0);

    for i in 0..n {
        println!("{} {}", i + 1, d[i]);
    }
}

#[derive(Clone, PartialEq)]
enum Status {
    White,
    Black,
}

fn dijkstra(matrix: & Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let n = matrix.len();
    let mut statuses = vec![Status::White; n];
    let mut d = vec![usize::MAX; n];
    d[s] = 0;
    let mut p = vec![usize::MAX; n];

    loop {
        let mut min_d = usize::MAX;
        let mut u = usize::MAX;
        for i in 0..n {
            if d[i] < min_d && statuses[i] != Status::Black {
                min_d = d[i];
                u = i;
            }
        }

        if min_d == usize::MAX { break; }

        statuses[u] = Status::Black;

        for v in 0..n {
            if statuses[v] != Status::Black && matrix[u][v] != usize::MAX {
                if d[u] + matrix[u][v] < d[v] {
                    d[v] = d[u] + matrix[u][v];
                    p[v] = u;
                }
            }
        }
    }
    d

}