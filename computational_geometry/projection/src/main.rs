use std::io::*;
use util::Scanner;
use point::{Point, Segment};

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

    let p1x: f64 = sc.read();
    let p1y: f64 = sc.read();
    let p2x: f64 = sc.read();
    let p2y: f64 = sc.read();

    let p1 = Point::new(p1x, p1y);
    let p2 = Point::new(p2x, p2y);
    let segment = Segment::new(p1, p2);

    let q: usize = sc.read();

    for _ in 0..q {
        let px: f64 = sc.read();
        let py: f64 = sc.read();
        let p = Point::new(px, py);
        let ans = segment.project(&p);
        println!("{} {}", ans.x, ans.y);
    }
}
