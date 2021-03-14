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
    let mut vec: Vec<usize> = sc.vec(n);

    println!("{}", merge_sort_inversion_count(&mut vec, 0, n));
}

fn merge_sort_inversion_count(a: &mut Vec<usize>, left: usize, right: usize) -> usize {
    if left + 2 <= right {
        let mid = (left + right) / 2;
        let v1 = merge_sort_inversion_count(a, left, mid);
        let v2 = merge_sort_inversion_count(a, mid, right);
        let v3 = merge_inversion_count(a, left, mid, right);
        return v1 + v2 + v3;
    }
    0
}

fn merge_inversion_count(a: &mut Vec<usize>, left: usize, mid: usize, right: usize) -> usize {
    let mut cnt = 0;
    let n1 = mid - left;
    let n2 = right - mid;
    let l: Vec<usize> = (0..n1).map(|i| {a[left + i]}).collect();
    let r: Vec<usize> = (0..n2).map(|i| {a[mid + i]}).collect();
    let (mut i, mut j) = (0, 0);
    for k in left..right {
        match (l.get(i), r.get(j)) {
            (Some(le), Some(ri)) => {
                if le <= ri {
                    a[k] = *le;
                    i += 1;
                } else {
                    a[k] = *ri;
                    j += 1;
                    cnt += n1 - i; // = mid + j - k - 1
                }
            },
            (Some(le), None) => {
                a[k] = *le;
                i += 1;
            },
            (None, Some(ri)) => {
                a[k] = *ri;
                j += 1;
                cnt += n1 - i; // = mid + j - k - 1
            },
            (None, None) => {
                unreachable!();
            }
        
        }
    }
    cnt
}
