use std::io::*;
use std::collections::*;
use util::{Scanner, Joinable};

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
    let mut list: VecDeque<i32> = VecDeque::with_capacity(n);

    for _ in 0..n {
        let inst: String = sc.read();
        match inst.as_str() {
            "insert" => {
                let x = sc.read();
                list.push_front(x);
            }
            "delete" => {
                let x = sc.read();
                let p = list.iter().position(|&e| e == x).unwrap_or(list.len());
                let mut tail = list.split_off(p);
                tail.pop_front();
                list.append(&mut tail);
            }
            "deleteFirst" => {
                list.pop_front();
            }
            "deleteLast" => {
                list.pop_back();
            }
            _ => panic!()
        }
    }
    println!("{}", list.iter().join(" "));

}
