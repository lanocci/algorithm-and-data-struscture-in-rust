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

    let mut nodes: Vec<Node> = Vec::new();
    make_bidimensional_tree(0, n, 0, &mut points, &mut nodes);

    let q: usize = sc.read();

    for _ in 0..q {
        let mut answer = Vec::new();
        let sx: usize = sc.read();
        let tx: usize = sc.read();
        let sy: usize = sc.read();
        let ty: usize = sc.read();
        find(0, sx, tx, sy, ty, 0, &points, &nodes, &mut answer);
        answer.sort_by_key(|a| a.id);
        for a in answer.iter() {
            println!("{}", a.id);
        }
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
    pub left: Option<usize>,
    pub right: Option<usize>,
}

fn make_bidimensional_tree(l: usize, r: usize, depth: usize, points: &mut Vec<Point>, nodes: &mut Vec<Node>) -> Option<usize> {
    if !(l < r) { return None; }
    let mid = (l + r) / 2;
    if depth % 2 == 0 {
        points[l..r].sort_by_key(|p| p.x);
    } else {
        points[l..r].sort_by_key(|p| p.y);
    }
    let t = nodes.len();
    nodes.push(Node{location: mid, left: None, right: None});
    nodes[t].left = make_bidimensional_tree(l, mid, depth + 1, points, nodes);
    nodes[t].right = make_bidimensional_tree(mid + 1, r, depth + 1, points, nodes);
    Some(t)
}

fn find(v: usize, sx: usize, tx: usize, sy: usize, ty: usize, depth: usize, points: &Vec<Point>, nodes: &Vec<Node>, answer: &mut Vec<Point>) {
    let x = points[nodes[v].location].x;
    let y = points[nodes[v].location].y;

    if sx <= x && sy <= y && x <= tx && y <= ty { answer.push(points[nodes[v].location].clone()); }

    if depth % 2 == 0 {
        if let Some(l) = nodes[v].left {
            if sx <= x {
                find(l, sx, tx, sy, ty, depth + 1, points, nodes, answer);
            }
        }
        if let Some(r) = nodes[v].right {
            if x <= tx {
                find(r, sx, tx, sy, ty, depth + 1, points, nodes, answer);
            }
        }
    } else {
        if let Some(l) = nodes[v].left {
            if sy <= y {
                find(l, sx, tx, sy, ty, depth + 1, points, nodes, answer);
            }
        }
        if let Some(r) = nodes[v].right {
            if y <= ty {
                find(r, sx, tx, sy, ty, depth + 1, points, nodes, answer);
            }
        }
    }
}