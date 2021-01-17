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

struct Coord {
    x: f32,
    y: f32,
}

fn solve() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let n: usize = sc.read();

    println!("{} {}", 0.0, 0.0);
    koch(n, &Coord{ x: 0., y: 0., }, &Coord{ x: 100., y: 0. });
    println!("{} {}", 100.0, 0.0);
}

const RADIAN: f32 = std::f32::consts::PI / 3.;

fn koch(d: usize, p1: &Coord, p2: &Coord) {
    if d == 0 { return (); }

    let s = Coord{ 
        x: (2. * p1.x + p2.x) / 3.,
        y: (2. * p1.y + p2.y) / 3.,
    };
    let t = Coord{ 
        x: (p1.x + 2. * p2.x) / 3.,
        y: (p1.y + 2. * p2.y) / 3.,
    };
    let u = Coord{
        x: (t.x - s.x) * RADIAN.cos() - (t.y - s.y) * RADIAN.sin() + s.x,
        y: (t.x - s.x) * RADIAN.sin() + (t.y - s.y) * RADIAN.cos() + s.y,
    };

    koch(d - 1, p1, &s);
    println!("{} {}", s.x, s.y);
    koch(d - 1, &s, &u);
    println!("{} {}", u.x, u.y);
    koch(d - 1, &u, &t);
    println!("{} {}", t.x, t.y);
    koch(d - 1, &t, p2);
}
