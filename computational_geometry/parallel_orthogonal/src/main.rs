use std::io::*;
use util::Scanner;
use geometric_element::point::Point;
use geometric_element::segment::Segment;

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

    let q: usize = sc.read();

    for _ in 0..q {
        let p0x: f64 = sc.read();
        let p0y: f64 = sc.read();
        let p1x: f64 = sc.read();
        let p1y: f64 = sc.read();
        let p2x: f64 = sc.read();
        let p2y: f64 = sc.read();
        let p3x: f64 = sc.read();
        let p3y: f64 = sc.read();
        let p0 = Point::new(p0x, p0y);
        let p1 = Point::new(p1x, p1y);
        let p2 = Point::new(p2x, p2y);
        let p3 = Point::new(p3x, p3y);
        let s1 = Segment::new(p0, p1);
        let s2 = Segment::new(p2, p3);
        if s1.is_orthogonal(&s2) {
            println!("1");
        } else if s1.is_parallel(&s2) {
            println!("2");
        } else {
            println!("0");
        }
    }

}
