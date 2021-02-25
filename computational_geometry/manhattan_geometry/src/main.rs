use std::io::*;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use num_traits::Float;
use util::Scanner;
use geometric_element::point::*;
use geometric_element::segment::*;

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

    let mut segments: Vec<Segment<f32>> = Vec::new();
    let mut et: Vec<EndPoint<f32>> = Vec::new();

    for i in 0..n {
        let x1 = sc.read();
        let y1 = sc.read();
        let p1 = EndPoint{
            seg_id: i,
            point: Point::new(x1, y1),
        };

        let x2 = sc.read();
        let y2 = sc.read();
        let p2 = EndPoint {
            seg_id: i,
            point: Point::new(x2, y2),
        };

        et.push(p1);
        et.push(p2);
    }

    et.sort_by(|a, b| {
        if a.point.y < b.point.y { Ordering::Less } 
        else if a.point.y > b.point.y { Ordering::Greater } 
        else {Ordering::Equal}
    });

    let mut t: BTreeSet<f32> = BTreeSet::new();

}

struct EndPoint<T> where T: Float {
    seg_id: usize,
    point: Point<T>
}