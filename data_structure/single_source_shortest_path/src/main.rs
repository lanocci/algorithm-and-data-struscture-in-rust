use std::io::*;
use std::collections::BinaryHeap;
use core::cmp::Ordering;
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

    let mut matrix: Vec<Vec<usize>> = vec![vec![usize::MAX; n]; n];
    
    for _ in 0..n {
        let idx: usize = sc.read();
        let k = sc.read();
        for _ in 0..k {
            let v: usize = sc.read();
            let c: usize = sc.read();
            matrix[idx][v] = c;
        }
    }

    let d = dijkstra_soph(&matrix, 0);

    for i in 0..n {
        println!("{} {}", i + 1, d[i]);
    }
}

#[derive(Clone, PartialEq)]
enum Status {
    White,
    Black,
}

/// ### Naive Implementation of Dijkstra's Algorithm
/// 
/// - `O(n^2)` time complexity
/// 
/// - In each calculation step, choose a vertex and an edge to add to `S`
/// 
/// 1. Initialize parameters
///   - `S`: empty set
///   - `d[s]`: 0
///   - For all `i`, except when `i = s`, `d[u] = inf.`
/// 2. Repeat following calculations until `S = V`
///   - Select the vertex `u` where `d[u]` is the minimum in `V - S` set
///   - Add `u` to the set `S`, and update all vertices which are next to u and belong to `V - S` with following calculation: 
///     - if `d[u] + w(u, v)` (weight between u and v) < `d[v]`
///       - assign `d[u] + w(u, v)` to `d[v]`
///       - assign `u` to `p[v]`
fn dijkstra_naive(matrix: & Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let n = matrix.len();
    let mut statuses = vec![Status::White; n];
    let mut d = vec![usize::MAX; n];
    d[s] = 0;
    let mut p = vec![usize::MAX; n];

    loop {
        let mut min_d = usize::MAX;
        let mut u = usize::MAX;
        for i in 0..n {
            if d[i] < min_d && statuses[i] != Status::Black {
                min_d = d[i];
                u = i;
            }
        }

        if min_d == usize::MAX { break; }

        statuses[u] = Status::Black;

        for v in 0..n {
            if statuses[v] != Status::Black && matrix[u][v] != usize::MAX {
                if d[u] + matrix[u][v] < d[v] {
                    d[v] = d[u] + matrix[u][v];
                    p[v] = u;
                }
            }
        }
    }
    d

}

/// ### More Sophisticated Implementation of Dijkstra's Algorithm (Using Priority Queue)
/// 
/// - `O((|V| + |E|) log||V|)
///     - log|V| for each pop from priority heap * |V| times
///     - log|V| for each push to priority heap * |E| times
/// 
/// 1. Initialize parameters
///   - `S`: empty set
///   - `d[s]`: 0
///   - For all `i`, except when `i = s`, `d[u] = inf.`
///   - Construct min-heap `H` with `d[u]` as keys
/// 2. Repeat following calculations until `S = V`
///   - Extract vertex `u` where `d[u]` is the minimum (and mark `Black`)
///   - Add `u` to the set `S`, and update all vertices which are next to u and belong to `V - S` with following calculation: 
///     - if `d[u] + w(u, v)` (weight between u and v) < `d[v]`
///       - assign `d[u] + w(u, v)` to `d[v]`
///       - assign `v` to `p[v]`
///       - update heap `H` starts from `v`
fn dijkstra_soph(matrix: & Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let n = matrix.len();
    let mut statuses = vec![Status::White; n];
    let mut d = vec![usize::MAX; n];
    d[s] = 0;
    let mut p = vec![usize::MAX; n];

    let mut h: BinaryHeap<Node> = BinaryHeap::new();
    h.push(Node{id: s, cost: 0});


    while !h.is_empty() {
        let mut min_d = usize::MAX;
        let mut u = n;
        u = h.pop().unwrap().id;
        for i in 0..n {
            if d[i] < min_d && statuses[i] != Status::Black {
                min_d = d[i];
                u = i;
            }
        }

        if min_d == usize::MAX { break; }
        statuses[u] = Status::Black;

        for v in 0..n {
            if statuses[v] != Status::Black && matrix[u][v] != usize::MAX {
                if d[u] + matrix[u][v] < d[v] {
                    d[v] = d[u] + matrix[u][v];
                    p[v] = u;
                    h.push(Node{id: v, cost: d[v]});
                }
            }
        }
    }
    d
}

#[derive(Eq)]
struct Node {
    pub id: usize,
    cost: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Reverse order of cost to construct min-heap for dijkstra's algorithm
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost < other.cost {
            Ordering::Greater
        } else if self.cost == other.cost {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}