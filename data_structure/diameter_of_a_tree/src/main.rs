use std::io::*;
use std::collections::VecDeque;
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

    let n: usize = sc.read();

    let mut edges: Vec<Vec<Edge>> = vec![Vec::new(); n];

    for _ in 1..n {
        let s: usize = sc.read();
        let t: usize = sc.read();
        let w: usize = sc.read();
        edges[s].push(Edge{to: t, weight: w});
        edges[t].push(Edge{to: s, weight: w});
    }

    let t = Tree::new(edges);
    let (x, _) = t.find_farthest(0);
    let (_, d) = t.find_farthest(x);
    println!("{}", d);
}

#[derive(Clone)]
struct Edge {
    to: usize,
    weight: usize,
}

struct Tree {
    edges: Vec<Vec<Edge>>,
}

impl Tree {
    fn new(edges: Vec<Vec<Edge>>) -> Tree {
        Tree{edges}
    }

    fn find_farthest(&self, from: usize) -> (usize, usize) {
        let mut weights: Vec<usize> = vec![usize::MAX; self.edges.len()];

        self.bfs(&mut weights, from);

        let mut max_idx = 0;
        let mut max_val = 0;
        for i in 0..weights.len() {
            if max_val <= weights[i] { 
                max_idx = i; 
                max_val = weights[i];
            }
        }
        (max_idx, max_val)
    }

    fn bfs(&self, weights: &mut Vec<usize>, idx: usize) {
        let mut q = VecDeque::new();
        q.push_back(idx);
        weights[idx] = 0;
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for c in self.edges[u].iter() {
                if weights[c.to] == usize::MAX {
                    weights[c.to] = weights[u] + c.weight; 
                    q.push_back(c.to);
                }
            }
        }

    }
}
