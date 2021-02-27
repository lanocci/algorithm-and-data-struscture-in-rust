use std::io::*;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::mem;
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

    let mut segments: Vec<Segment<f64>> = Vec::new();
    let mut ep: Vec<EndPoint<f64>> = Vec::new();

    for i in 0..n {
        let x1 = sc.read();
        let y1 = sc.read();
        let mut p1 = Point::new(x1, y1);

        let x2 = sc.read();
        let y2 = sc.read();
        let mut p2 = Point::new(x2, y2);

        if y1 > y2 || y1 == y2 && x1 >= x2 {
            mem::swap(&mut p1, &mut p2);
        }
        segments.push(Segment::new(p1.clone(), p2.clone()));

        let (ep1, ep2) = if y1  == y2 {
            (EndPoint{
                position: Position::Left,
                seg_id: i,
                point: p1,
            }, EndPoint {
                position: Position::Right,
                seg_id: i,
                point: p2
            })
        } else {
            (EndPoint{
                position: Position::Bottom,
                seg_id: i,
                point: p1,
            }, EndPoint {
                position: Position::Top,
                seg_id: i,
                point: p2
            })
        };

        ep.push(ep1);
        ep.push(ep2);
    }

    ep.sort_by(|a, b| {
        if a.point.y < b.point.y { Ordering::Less } 
        else if a.point.y > b.point.y { Ordering::Greater } 
        else {Ordering::Equal}
    });

    let mut t: BTreeSet<usize> = BTreeSet::new();

    let mut cnt = 0;
    for p in ep.iter() {
        match p.position {
            Position::Top => {
                t.remove(&(p.point.x as usize));
            },
            Position::Bottom => {
                t.insert(p.point.x as usize);
            },
            Position::Left => {
                let seg = segments[p.seg_id].clone();
                let b = t.range((seg.p1.x as usize)..);
                let e = t.range(..(seg.p2.x as usize));
                for w in b {
                    for x in e.clone() {
                        if w == x { cnt += 1; }
                    }
                }
            },
            Position::Right => {}
        }
    }

    println!("{}", cnt);

}

struct EndPoint<T> where T: Float {
    position: Position,
    seg_id: usize,
    point: Point<T>
}

enum Position {
    Left,
    Right,
    Top,
    Bottom,
}