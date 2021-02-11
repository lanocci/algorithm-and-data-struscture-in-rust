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

    let mut bt = BidimensionalTree::new(points);

    let q: usize = sc.read();

    for _ in 0..q {
        let sx: usize = sc.read();
        let tx: usize = sc.read();
        let sy: usize = sc.read();
        let ty: usize = sc.read();
        let mut answer = bt.find(sx, tx, sy, ty);
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

struct BidimensionalTree {
    points: Vec<Point>,
    nodes: Vec<Node>,
}

impl BidimensionalTree {

    fn new(points: Vec<Point>) -> BidimensionalTree {
        let mut bt = BidimensionalTree {
            points,
            nodes: Vec::new(),
        };
        bt.make_nodes(0, bt.points.len(), 0);
        bt
    }

    fn make_nodes(&mut self, l: usize, r: usize, depth: usize) -> Option<usize> {
        if !(l < r) { return None; }
        let mid  = (l + r) / 2;
        if depth % 2 == 0 {
            self.points[l..r].sort_by_key(|p| p.x);
        } else {
            self.points[l..r].sort_by_key(|p| p.y);
        }
        let t = self.nodes.len();
        self.nodes.push(Node{location: mid, left: None, right: None});
        self.nodes[t].left = self.make_nodes(l, mid, depth + 1);
        self.nodes[t].right = self.make_nodes(mid + 1, r, depth + 1);
        Some(t)
    }

    fn find(&mut self, sx: usize, tx: usize, sy: usize, ty: usize) -> Vec<Point>{
        let mut ans: Vec<Point> = Vec::new();
        self.find_rec(0, sx, tx, sy, ty, 0, &mut ans);
        ans
    }

    fn find_rec(&mut self, v: usize, sx: usize, tx: usize, sy: usize, ty: usize, depth: usize, ans: &mut Vec<Point>) {
        let x = self.points[self.nodes[v].location].x;
        let y = self.points[self.nodes[v].location].y;

        if sx <= x && x <= tx && sy <= y && y <= ty {
            ans.push(self.points[self.nodes[v].location].clone());
        }

        if depth % 2 == 0 {
            if let Some(l) = self.nodes[v].left {
                if sx <= x {
                    self.find_rec(l, sx, tx, sy, ty, depth + 1, ans);
                }
            }
            if let Some(r) = self.nodes[v].right {
                if x <= tx {
                    self.find_rec(r, sx, tx, sy, ty, depth + 1, ans);
                }
            }
        } else {
            if let Some(l) = self.nodes[v].left {
                if sy <= y {
                    self.find_rec(l, sx, tx, sy, ty, depth + 1, ans);
                }
            }
            if let Some(r) = self.nodes[v].right {
                if y <= ty {
                    self.find_rec(r, sx, tx, sy, ty, depth + 1, ans);
                }
            }
        }
    }

}