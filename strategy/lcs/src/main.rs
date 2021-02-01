use std::io::*;
use util::{Joinable, Scanner};

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
    
    let q: usize = sc.read();
    for _ in 0..q {
        let a: String = sc.read();
        let b: String = sc.read();
        let res = lcs(&a, &b);
        println!("{}", res);
    }
}

fn lcs(a: &String, b: &String) -> usize {
    let mut memo: Vec<Vec<usize>> = vec![vec![0; b.len() + 1]; a.len() + 1];
    let mut max: usize = 0;
    for i in 1..a.as_str().len() + 1 {
        for j in 1..b.as_str().len() + 1 {
            if let (Some(x), Some(y)) = (a.as_str().chars().nth(i - 1), b.as_str().chars().nth(j - 1)) {
                if x == y {
                    memo[i][j] = memo[i - 1][j - 1] + 1;
                } else {
                    memo[i][j] = memo[i - 1][j].max(memo[i][j - 1]);
                }
            } else {
                memo[i][j] = memo[i - 1][j].max(memo[i][j - 1]);

            }
            max = max.max(memo[i][j]);
        }
    }

    max
