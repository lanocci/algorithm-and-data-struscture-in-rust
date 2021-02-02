use std::io::*;
use util::{Scanner, Joinable};

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

    let mut m = vec![vec![0; n]; n];

    for _ in 0..n {
        let u: usize = sc.read();
        let k: usize = sc.read();
        let vec: Vec<usize> = sc.vec(k);

        for &adj in vec.iter() {
            m[u - 1][adj - 1] = 1;
        }
    }

    for v in m.iter() {
        println!("{}", v.iter().join(" "));
    }

}
