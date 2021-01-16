use util::Scanner;
use std::io::*;

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
    let a: Vec<i32> = sc.vec(n);
    let q = sc.read();
    let m: Vec<i32> = sc.vec(q);


    for x in m.iter() {
        let res = rec(&a, *x, 0, n);
        if res {
            println!("yes");
        } else {
            println!("no");
        }
    }

}

fn rec(a: &Vec<i32>, x: i32, i: usize, n: usize) -> bool {
    if x == 0 {
        return true;
    }
    if i >= n {
        return false;
    }
    let res = rec(&a, x, i + 1, n) || rec(&a, x - a[i], i + 1, n);
    res
}
