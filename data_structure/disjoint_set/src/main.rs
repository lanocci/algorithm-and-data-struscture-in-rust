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
    let q = sc.read();

    let mut ds = DisjointSet::new(n);

    for _ in 0..q {
        match sc.read::<String>().as_str() {
            "0" => {
                ds.unite(sc.read(), sc.read());
            },
            "1" => {
                if ds.same(sc.read(), sc.read()) {
                    println!("1");
                } else {
                    println!("0");
                }
            },
            _ => unreachable!(),
        }
    }
}

/// contains metadata for a disjoint set
/// rank represents hight of each set represented as tree
/// p represents index of each node's parent node
struct DisjointSet {
    rank: Vec<usize>,
    p: Vec<usize>,
}


/// ## DisjointSet implementation with ranks and path compression
impl DisjointSet {
    fn new(n: usize) -> DisjointSet {
        DisjointSet {
            rank: vec![0; n],
            p: (0..n).collect::<Vec<usize>>(),
        }
    }
    
    /// unite two disjointsets by linking each representative node with link method
    fn unite(&mut self, this: usize, that: usize) {
        let this = self.find_set(this);
        let that = self.find_set(that);
        self.link(this, that);
    }

    /// link representative nodes
    /// new representative node will be the one that holds higher rank
    /// if the both nodes hold the same rank, choose either of them and increment rank by 1
    fn link(&mut self, this: usize, that: usize) {
        if self.rank[this] > self.rank[that] {
            self.p[that] = this;
        } else {
            self.p[this] = that;
            if self.rank[this] == self.rank[that] {
                self.rank[that] += 1;
            }
        }
    }

    /// compare representative nodes of the sets where x and y node belongs respectively to.
    fn same(&mut self, this: usize, that: usize) -> bool {
        self.find_set(this) == self.find_set(that)
    }

    /// return the representative node in a set where the x node belongs to; by following parent nodes. 
    /// (each node's  parent is initialized with self)
    /// at the same time perform path compression
    /// path compression makes it more efficient to perform find_set next time
    fn find_set(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find_set(self.p[x]);
        }
        self.p[x]
    }

}