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

    let ds = DisjointSet::new();

    for _ in 0..q {
        match sc.read() {
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
            p: vec![0; n],
        }
    }
    
    fn unite(&mut self, this: usize, that: usize) {
    }

    fn same(&mut self, this: usize, that: usize) -> bool {
    }

}