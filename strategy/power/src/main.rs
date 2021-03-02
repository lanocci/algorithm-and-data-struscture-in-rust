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

    let x: usize = sc.read();
    let n: usize = sc.read();
    let m = 1_000_000_007;

    println!("{}", power_mod(x, n, m));
}

fn power_mod(x: usize, n: usize, m: usize) -> usize {
    if n == 0 { return 1 }
    let mut res = power_mod(x * x % m, n / 2, m);
    if n % 2 == 1 {
        res = res * x % m;
    }
    res
}

fn power(x: usize, n: usize, m: usize) -> {
    if n == 0 { return 1 }
    else n % 2 {
        power(x * x, n / 2)
    } else {
        power(x * x, n / 2) * x
    }
}
