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
    let n = sc.read();
    let vec: Vec<usize> = sc.vec(n);
    let res = counting_sort(&vec);
    println!("{}", res.iter().join(" "));
}

fn counting_sort(a: &Vec<usize>) -> Vec<usize> {
    let n = a.len() as usize;
    let mut b: Vec<usize> = vec![0; n];
    let mut c: Vec<usize> = vec![0; 10_001];

    for &x in a.iter() {
        c[x] += 1;
    }

    for i in 1..c.len() {
        c[i] = c[i] + c[i-1];
    }

    for &x in a.iter().rev() {
        b[c[x] - 1] = x;
        c[x] -= 1;
    }

    b
}