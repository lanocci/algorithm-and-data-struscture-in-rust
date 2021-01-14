use std::io::*;
use std::cmp::min;
use std::collections::*;
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
    let q: usize = sc.read();
    let mut names = Vec::with_capacity(n);
    let mut durations = Vec::with_capacity(n);
    let mut queue = VecDeque::with_capacity(n);
    for i in 0..n {
        let s:String = sc.read();
        names.push(s);
        let t = sc.read();
        durations.push(t);
        queue.push_back(i);
    }

    let mut total_duration = 0;
    while let Some(p) = queue.pop_front() {
        let t = min(durations[p], q);
        durations[p] -= t;
        total_duration += t;
        if durations[p] > 0 {
            queue.push_back(p)
        } else {
            println!("{} {}", names[p], total_duration)
        }
    }
}
