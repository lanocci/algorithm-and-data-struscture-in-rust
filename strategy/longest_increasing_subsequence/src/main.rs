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

    let a = {
        let mut a: Vec<i32> = Vec::new();
        a.push(i32::MIN);
        for _ in 0..n {
            a.push(sc.read());
        }
        a
    };
    let result = lis(&a);
    println!("{}", result);
}

fn naive_lis(a: &Vec<i32>) -> usize {
    let n = a.len();

    // list of length of LIS when a[i] is the last element in the LIS
    let mut l: Vec<usize> = vec![0; n];

    // list of the indices of the second last element in the LIS when a[i] is the last element in the LIS
    let mut p: Vec<usize> = vec![usize::MAX; n];

    for i in 1..n {
        let mut k = 0;
        for j in 0..i {
            if a[j] < a[i] && l[j] > l[k] {
                k = j;
            }
        }
        l[i] = l[k] + 1;
        p[i] = k;
    }
    *l.iter().max().unwrap()
}

fn lis(a: &Vec<i32>) -> usize {
    let n = a.len();

    // a vector of the smallest last element of LIS when length of LIS is i + 1
    let mut l: Vec<i32> = Vec::new();
    l.push(a[0]);

    for &v in a.iter() {
        if *l.last().unwrap() < v {
            l.push(v)
        } else {
            for last in l.iter_mut() {
                if *last > v {
                    *last = v;
                    break;
                }
            }
        }
    }
    l.len() - 1
}