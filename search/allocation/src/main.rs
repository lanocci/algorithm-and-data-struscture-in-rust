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

    let n: usize = sc.read();
    let k: usize = sc.read();

    let w: Vec<usize> = (0..n).map(|_| {sc.read()}).collect();

    let mut left: usize = 0;
    let mut right: usize = usize::MAX;
    let mut mid: usize = 0;

    while right - left > 1 {
        mid = (left + right) / 2;
        let v = check(&w, k, mid);
        if v >= n { right = mid; }
        else { left = mid; }
    }
    println!("{}", right);
}

/// Returns how many items k trucks can be loaded.
/// 
/// # Arguments: 
/// 
/// * `items` - a vector of unsigned integers that holds the weight of each items.
/// * `k` - an unsigned integer that holds the number of trucks
/// * `p` - an unsigned integer that holds the load capacity of the trucks
fn check(items: &Vec<usize>, k: usize, p: usize) -> usize {
    let mut i = 0;
    for _ in 0..k {
        let mut s = 0;
        while s + items[i] <= p {
            s += items[i];
            i += 1;
            if i == items.len() { return items.len(); }
        }
    }
    i
}
