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
    let mut s: Vec<i32> = sc.vec(n);
    let q = sc.read();
    let mut t: Vec<i32> = sc.vec(q);

    s.sort();
    s.dedup();

    let mut cnt = 0;

    for key in t.iter() {
        let mut left = 0;
        let mut right = n;
        while left < right {
            let mid = (left + right) / 2;
            if key == &s[mid] {
                cnt += 1;
                break;
            }
            else if key < &s[mid] {
                right = mid;
            }
            else {
                left = mid + 1;
            }
        }
    }
    println!("{}", cnt);

}
