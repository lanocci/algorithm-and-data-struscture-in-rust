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

    let mut ps = vec![0; n + 1];
    let mut memo = vec![vec![0; n + 1]; n + 1];
    for i in 1..n + 1 {
        ps[i - 1] = sc.read();
        ps[i] = sc.read();
    }

    for l in 2..(n + 1) {
        for i in 1..(n - l + 2) {
            let j = i + l - 1;
            memo[i][j] = usize::MAX;
            for k in i..j {
                memo[i][j] = memo[i][j].min(memo[i][k] + memo[k + 1][j] + ps[i - 1] * ps[k] * ps[j]);
            }
        }
    }
    println!("{}", memo[1][n])
}
