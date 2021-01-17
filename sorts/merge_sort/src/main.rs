use std::io::*;
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
    let mut vec = sc.vec(n);
    let mut cnt: i32 = 0;
    merge_sort(&mut vec, 0, n, &mut cnt);
    println!("{}", vec.iter().join(" "));
    println!("{}", cnt);
}

fn merge(a: &mut Vec<i32>, left: usize, mid: usize, right: usize, cnt: &mut i32) {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut a_l: Vec<i32> = Vec::with_capacity(n1);
    for i in left..mid {
        a_l.push(a[i]);
    }
    let mut a_r: Vec<i32> = Vec::with_capacity(n2);
    for j in mid..right {
        a_r.push(a[j]);
    }

    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        *cnt += 1;
        match (a_l.get(i), a_r.get(j)) {
            (Some(l), Some(r)) => {
                if l <= r {
                    a[k] = *l;
                    i += 1;
                } else {
                    a[k] = *r;
                    j += 1;
                }
            }
            (Some(l), None) => {
                a[k] = *l;
                i += 1;
            }
            (None, Some(r)) => {
                a[k] = *r;
                j += 1;
            }
            (None, None) => {
                panic!()
            }
        }
    }
}

fn merge_sort(a: &mut Vec<i32>, left: usize, right: usize, cnt: &mut i32) {
    if left + 1 < right {
        let mid = (left + right) / 2;
        merge_sort(a, left, mid, cnt);
        merge_sort(a, mid, right, cnt);
        merge(a, left, mid, right, cnt);
    }
}