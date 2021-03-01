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

    let numbers = {
        let mut numbers: Vec<usize> = Vec::new();
        for _ in 0..n {
            numbers.push(sc.read());
        }
        numbers
    };

    let largest_num = numbers.iter().max().unwrap();
    let is_prime = eratos(&largest_num);

    let mut prime_count = 0;

    for &b in is_prime.iter() {
        prime_count += if b {1} else {0};
    }

    println!("{}", prime_count);
}

fn eratos(n: &usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..is_prime.len() {
        for j in ((i+i)..(n+1)).step_by(i) {
            is_prime[j] = false;
        }
    }
    is_prime
}