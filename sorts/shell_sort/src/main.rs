use std::io::*;
use util::{Joinable, Scanner};

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
    let v = {
        let mut v: Vec<usize> = Vec::new();
        for _ in 0..n {
            v.push(sc.read());
        }
        v
    };
    let result = shell_sort(&v, n);
    for r in result.iter() {
        println!("{}", r);
    }
}

fn insertion_sort(a: &mut Vec<usize>, n: usize, g: usize) {
    let mut cnt = 0;
    for i in g..n {
        let buf = a[i];
        let mut j = i as i32 - g as i32;
        while j >= 0 && a[j as usize] > buf {
            a[j as usize + g] = a[j as usize];
            j = j - g as i32;
            cnt += 1;
        }
        a[(j + g as i32) as usize] = buf;
    }
}

fn shell_sort(a: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut sorted = a.clone();
    let mut cnt = 0;
    let g = {
        let mut g: Vec<usize> = Vec::new();
        for i in 0..=n/3 {
            let x = i * 3 + 1;
            if x <= n {
                g.push(x);
            }
        }
        g.reverse();
        g
    };
    for i in g.iter() {
        insertion_sort(&mut sorted, n, *i);
    }
    println!("{}", g.iter().join(" "));
    sorted
}