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
    let mut vec: Vec<i32> = sc.vec(n);

    let q = partition(&mut vec, 0, n - 1);

    println!("{} [{}] {}", vec.iter().take(q).join(" "), vec[q], vec.iter().rev().take(n - (q + 1)).rev().join(" "));
}

fn partition(a: &mut Vec<i32>, p: usize, r: usize) -> usize {
    let x = a[r];
    let mut i: i32 = p as i32 - 1;
    for j in p..r {
        if a[j] <= x {
            i += 1;
            let tmp = a[j];
            a[j] = a[i as usize];
            a[i as usize] = tmp;
        }
    }
    a[r] = a[(i+1) as usize];
    a[(i+1) as usize] = x;
    (i + 1) as usize
}