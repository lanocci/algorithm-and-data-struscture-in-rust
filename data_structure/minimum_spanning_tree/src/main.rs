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

    let mut t = vec![false; n];
    let mut idx: usize = 0;

    while if let Some(_) = t.iter().find(|x| !x) {
        t[idx] = true;
        let min_v = std::i32::MAX;
        let min_i = 0;
        for i in 0..n {
            if 0 <= matrix[idx][i] && matrix[idx][i] <= min_v && !t[i] {
                min_v = cmp::min(min_v, matrix[idx][i]);
            }
        }
    }

}
