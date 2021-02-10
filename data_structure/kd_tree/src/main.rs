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

    let n = sc.read();

    let mut points = Vec::new();
    for i in 0..n {
        let x: usize = sc.read();
        let y: usize = sc.read();
        points.push(Point{id: i, x, y,});
    }

    let mut nodes = vec![Node{location: 0, parent: 0, left: None, right: None}; n];
    make_bidimensional_tree(0, n, 0, 0, &mut points, &mut nodes);

    let q: usize = sc.read();

    for j in 0..n {
        let sx: usize = sc.read();
        let tx: usize = sc.read();
        let sy: usize = sc.read();
        let ty: usize = sc.read();
    }

}

#[derive(Clone)]
struct Point {
    id: usize,
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Node {
    pub location: usize,
    pub parent: usize,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

fn make_bidimensional_tree(l: usize, r: usize, depth: usize, idx: usize, points: &mut Vec<Point>, nodes: &mut Vec<Node>) -> Option<usize> {
    if !(l < r) { return None; }
    let mid = (l + r) / 2;
    let t = idx + 1;
    if depth % 2 == 0 {
        points[l..r].sort_by_key(|p| p.x);
    } else {
        points[l..r].sort_by_key(|p| p.y);
    }
    nodes[t].location = mid;
    nodes[t].left = make_bidimensional_tree(l, mid, depth + 1, t, points, nodes);
    nodes[t].right = make_bidimensional_tree(mid + 1, r, depth + 1, t, points, nodes);
    Some(t)
}