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

    println!("{}", fib(n));

}

fn fib(n: usize) -> usize {
    let mut memo = vec![None; n];
    fib_rec(n, &mut memo)
}

fn fib_rec(n: usize, memo: &mut Vec<Option<usize>>) -> usize {
    if n == 0 || n == 1 {
        1
    } else {
        match memo[n - 1] {
            Some(r) => r,
            None => {
                let result = fib_rec(n - 1, memo) + fib_rec(n - 2, memo);
                memo[n - 1] = Some(result);
                result
            }
        }
    }
}
