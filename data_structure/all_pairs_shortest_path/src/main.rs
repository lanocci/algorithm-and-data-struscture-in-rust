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

    let v: usize = sc.read();
    let e: usize = sc.read();

    for i in 0..e {
        let s: usize = sc.read();
        let t: usize = sc.read();
        let d: i32 = sc.read();
    }

}
