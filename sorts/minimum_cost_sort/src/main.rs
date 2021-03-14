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

    let vec: Vec<usize> = sc.vec(n);

    let sorted = {
        let mut a = vec.clone();
        a.sort();
        a
    };

    let sorted_indices: Vec<usize> = {
        let mut t: Vec<usize> = vec![0; *sorted.iter().max().unwrap() + 1];
        for (i, s) in sorted.iter().enumerate() {
            t[*s] = i;
        }
        t
    };

    let mut has_loop: Vec<bool> = vec![false; n];
    let mut ans = 0;

    for i in 0..n {
        if has_loop[i] { continue; }
        let mut cur = i;
        let mut s: i32 = 0;
        let mut an: i32 = 0;
        let mut m = i32::MAX;
        loop {
            has_loop[cur] = true;
            an += 1;
            let v = vec[cur] as i32;
            m = std::cmp::min(m, v);
            s += v;
            cur = sorted_indices[v as usize];
            if has_loop[cur] { break; }
        }
        ans += std::cmp::min(s + (an - 2) * m, m + s + (an + 1) * s);
    }

    println!("{}", ans);
}
