use std::io::*;
use std::cmp::min;
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

    let v: usize = sc.read();
    let e: usize = sc.read();

    let mut weights = vec![vec![i32::MAX; v]; v];
    for (i, w) in weights.iter_mut().enumerate() {
        w[i] = 0;
    }

    for i in 0..e {
        let s: usize = sc.read();
        let t: usize = sc.read();
        let d: i32 = sc.read();
        weights[s][t] = d;
    }
    let mut mf = WarshallFloyd(weights);
    mf.find_apsp();
    let mut negative = false;
    for (i, d) in mf.0.iter().enumerate() {
        if d[i] < 0 {
            negative = true;
        }
    }
    if negative {
        println!("NEGATIVE CYCLE");
    } else {
        for d in mf.0.iter() {
            for &w in d.iter() {
                let s = if w == i32::MAX {
                    String::from("INF")
                } else {
                    w.to_string()
                };
                print!("{} ", s);
            }
            println!("");
        }
    }
}

/// represents warshall-floyd's algorithm
struct WarshallFloyd(Vec<Vec<i32>>);

impl WarshallFloyd {

    /// solves all pairs shortest path problem
    /// self.a[i][j] will hold smallest cost from index i to index j
    /// if given a[v][v] holds negative value, it means the graph contains negative cycle.
    fn find_apsp(&mut self) {
        let n = self.0.len();
        for k in 0..n {
            for i in 0..n {
                if self.0[i][k] == i32::MAX { continue; }
                for j in 0..n {
                    if self.0[k][j] == i32::MAX { continue; }
                    self.0[i][j] = min(self.0[i][j], self.0[i][k] + self.0[k][j])
                }
            }
        }
    }

}