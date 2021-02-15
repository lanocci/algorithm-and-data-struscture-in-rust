use std::io::*;
use util::Scanner;
use disjoint_set::DisjointSet;

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
