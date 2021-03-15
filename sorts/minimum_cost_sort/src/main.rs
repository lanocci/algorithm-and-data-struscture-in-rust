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

    let smallest = *vec.iter().min().unwrap() as i32;

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

    let mut cycle_found: Vec<bool> = vec![false; n];
    let mut ans = 0;

    for i in 0..n {
        if cycle_found[i] { continue; }
        let mut cur = i;
        let mut sum_weights_in_cycle: i32 = 0;
        let mut cycle_size: i32 = 0;
        let mut minimum_in_cycle = i32::MAX;
        loop {
            cycle_found[cur] = true;
            cycle_size += 1;
            let v = vec[cur] as i32;
            minimum_in_cycle = std::cmp::min(minimum_in_cycle, v);
            sum_weights_in_cycle += v;
            cur = sorted_indices[v as usize];
            if cycle_found[cur] { break; }
        }
        ans += std::cmp::min(sum_weights_in_cycle + (cycle_size - 2) * minimum_in_cycle, minimum_in_cycle + sum_weights_in_cycle + (cycle_size + 1) * smallest);
    }

    println!("{}", ans);
}
