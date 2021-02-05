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

    let n: usize = sc.read();
    let mut adj_list: Vec<VecDeque<usize>> = vec![VecDeque::new(); n];

    let m: usize = sc.read();

    for i in 0..m {
        let s: usize = sc.read();
        let t: usize = sc.read();

        adj_list[s].push_back(t);
        adj_list[t].push_back(s);
    }

    let q: usize = sc.read();

    for i in 0..q {
        let s: usize = sc.read();
        let t: usize = sc.read();

        let mut ans = false;
        for &x in adj_list[s].iter() {
            if x == t {
                ans = true;
            }
        }
        println!("{}", if ans { "yes" } else { "no" });
    }
}
