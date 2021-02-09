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

struct DisjointSet {
    rank: Vec<usize>,
    p: Vec<usize>,
}


impl DisjointSet {
    fn new(n: usize) -> DisjointSet {
        DisjointSet {
            rank: vec![0; n],
            p: (0..n).collect::<Vec<usize>>(),
        }
    }
    
    fn unite(&mut self, this: usize, that: usize) {
        let this = self.find_set(this);
        let that = self.find_set(that);
        self.link(this, that);
    }

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

    fn same(&mut self, this: usize, that: usize) -> bool {
        self.find_set(this) == self.find_set(that)
    }

    fn find_set(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find_set(self.p[x]);
        }
        self.p[x]
    }

}