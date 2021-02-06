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
    let mut idx: usize = 0;

    while let Some(_) = t.iter().find(|x| x == Status::Black) {
        let min_v = std::i32::MAX;
        let u = -1;
        for i in 0..n {
            if 0 <= matrix[idx][i] && matrix[idx][i] <= min_v && t[i] != Status::Black {
                min_v = matrix[idx][i];
                u = i;
            }
        }

        if distance == std::i32::MAX {

        } else {
            idx = min_i;
            distance += min_v;
            p[min_i] = idx;
            t[]
        }
    }

    println!("{}", distance);

}

#[derive(Clone, PartialEq)]
enum Status {
    White,
    Grey,
    Black,
}