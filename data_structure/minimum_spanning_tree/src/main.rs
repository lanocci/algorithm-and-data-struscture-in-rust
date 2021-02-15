use std::io::*;
use util::Scanner;
use core::cmp::Ordering;
use disjoint_set::DisjointSet;

fn main() {
    std::thread::Builder::new()
        .stack_size(1048576)
        .spawn(solve_with_kruskal)
        .unwrap()
        .join()
        .unwrap();
}

fn solve_with_naive_prim() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let n: usize = sc.read();

    let mut matrix = vec![vec![-1; n]; n];

    for i in 0..n {
        matrix[i] = sc.vec(n);
    }

    let ans = naive_prim(&matrix);

    println!("{}", ans.iter().fold(0, |sum, x| sum + x));
}

fn naive_prim(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    let mut t = vec![Status::New; n];
    let mut p: Vec<usize> = vec![n; n];
    let mut d: Vec<i32> = vec![std::i32::MAX; n];
    d[0] = 0;

    loop {
        let mut min_v = std::i32::MAX;
        let mut u = n;
        for i in 0..n {
            if d[i] < min_v && t[i] != Status::Active {
                min_v = d[i];
                u = i;
            }
        }
        if min_v == std::i32::MAX { break; }

        t[u] = Status::Active;

        for v in 0..n {
            if t[v] != Status::Active && matrix[u][v] != -1 {
                if matrix[u][v] < d[v] {
                    d[v] = matrix[u][v];
                    p[v] = u;
                }
            }
        }
    }
    d
}

#[derive(Clone, PartialEq)]
enum Status {
    New,
    Active,
}

fn solve_with_kruskal() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let v: usize = sc.read();
    let e: usize = sc.read();

    let mut adjs: Vec<Edge> = Vec::new();

    for _ in 0..e {
        let s: usize = sc.read();
        let t: usize = sc.read();
        let w: usize = sc.read();
        adjs.push(Edge{from: s, to: t, weight: w});
    }
    let total = kruskal(&mut adjs);
    println!("{}", total);
}

fn kruskal(adjs: &mut Vec<Edge>) -> usize {
    let mut total = 0;
    let n = adjs.len();
    let mut dset = DisjointSet::new(n + 1);
    adjs.sort();

    for e in adjs.iter() {
        if !dset.same(e.from, e.to) {
            total += e.weight;
            dset.unite(e.from, e.to);
        }
    }
    total
}

#[derive(Clone, Eq)]
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.weight > other.weight {
            Ordering::Greater
        } else if self.weight < other.weight {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}