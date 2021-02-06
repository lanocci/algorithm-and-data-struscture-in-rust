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

    let mut matrix = vec![vec![-1; n]; n];

    for i in 0..n {
        matrix[i] = sc.vec(n);
    }

    let mut t = vec![Status::White; n];
    let mut p: Vec<usize> = vec![n; n];
    let mut d: Vec<i32> = vec![std::i32::MAX; n];
    d[0] = 0;

    loop {
        let mut min_v = std::i32::MAX;
        let mut u = n;
        for i in 0..n {
            if d[i] < min_v && t[i] != Status::Black {
                min_v = d[i];
                u = i;
            }
        }
        if min_v == std::i32::MAX { break; }

        t[u] = Status::Black;

        for v in 0..n {
            if t[v] != Status::Black && matrix[u][v] != -1 {
                if matrix[u][v] < d[v] {
                    d[v] = matrix[u][v];
                    p[v] = u;
                }
            }
        }
    }

    println!("{}", d.iter().fold(0, |sum, x| sum + x));

}

#[derive(Clone, PartialEq)]
enum Status {
    White,
    Black,
}