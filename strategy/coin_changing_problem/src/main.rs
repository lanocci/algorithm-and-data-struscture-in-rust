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
    let m: usize = sc.read();

    let c = {
        let mut c: Vec<usize> = Vec::new();
        for _ in 0..m {
            c.push(sc.read());
        }
        c
    };


    let t = {
        let mut t: Vec<usize> = vec![usize::MAX; n + 1];
        t[0] = 0;
        for &face in c.iter() {
            for j in face..(n + 1) {
                t[j] = std::cmp::min(t[j], t[j - face] + 1);
            }
        }
        t
    };
    println!("{}", t[n]);
}
