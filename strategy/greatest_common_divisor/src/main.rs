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
    let y: usize = sc.read();

    println!("{}", gcd(&x, &y));

}

fn gcd(x: &usize, y: &usize) -> usize {
    let (mut x, mut y) = (*x, *y);
    if x < y { std::mem::swap(&mut x, &mut y); }
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}