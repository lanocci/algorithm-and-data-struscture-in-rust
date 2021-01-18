use std::io::*;
use util::Scanner;

#[derive(Clone)]
struct Card {
    suit: String,
    number: i32,
}

trait Quantifiable {
    fn qt(&self) -> i32;
}

impl Quantifiable for Card {
    fn qt(&self) -> i32 {
        self.number
    }
}

impl Card {
    fn cmp(&self, other: &Card) -> bool {
        self.suit == other.suit && self.number == other.number
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(10485760000)
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
    let mut vec: Vec<Card> = Vec::with_capacity(n);
    for _ in 0..n {
        let s: String = sc.read();
        let n: i32 = sc.read();
        vec.push(Card{ suit: s, number: n});
    }
    quick_sort(&mut vec, 0, n - 1);

    let mut merge_sorted = vec.clone();
    merge_sort(&mut merge_sorted, 0, n);
    let mut is_stable = true;
    for i in 0..n {
        if !merge_sorted[i].cmp(&vec[i]) {
            is_stable = false;
        }
    }
    if is_stable {
        println!("Stable");
    } else {
        println!("Not stable");
    }
    for i in 0..n {
        println!("{} {}", vec[i].suit, vec[i].number);
    }
}

fn quick_sort(a: &mut Vec<Card>, p: usize, r: usize) {
    if (r as i32 - p as i32) < 1 { return (); }
    let i = partition(a, p, r);
    quick_sort(a, p, i - 1);
    quick_sort(a, i, r); 
}

fn partition<T>(a: &mut Vec<T>, p: usize, r: usize) -> usize 
where T: Quantifiable + Clone {
    let x: T = a[r].clone();
    let mut i = p as i32 - 1;
    for j in p..r {
        if a[j].qt() <= x.qt() {
            i += 1;
            let tmp = a[j].clone();
            a[j] = a[i as usize].clone();
            a[i as usize] = tmp;
        }
    }
    let i = (i + 1) as usize;
    a[r] = a[i].clone();
    a[i] = x;
    i
}

fn merge(a: &mut Vec<Card>, left: usize, mid: usize, right: usize) {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut a_l: Vec<Card> = Vec::with_capacity(n1);
    for i in left..mid {
        a_l.push(a[i].clone());
    }
    let mut a_r: Vec<Card> = Vec::with_capacity(n2);
    for j in mid..right {
        a_r.push(a[j].clone());
    }

    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        match (a_l.get(i), a_r.get(j)) {
            (Some(l), Some(r)) => {
                if l.number <= r.number {
                    a[k] = l.clone();
                    i += 1;
                } else {
                    a[k] = r.clone();
                    j += 1;
                }
            }
            (Some(l), None) => {
                a[k] = l.clone();
                i += 1;
            }
            (None, Some(r)) => {
                a[k] = r.clone();
                j += 1;
            }
            (None, None) => {
                panic!()
            }
        }
    }
}

fn merge_sort(a: &mut Vec<Card>, left: usize, right: usize) {
    if left + 1 < right {
        let mid = (left + right) / 2;
        merge_sort(a, left, mid);
        merge_sort(a, mid, right);
        merge(a, left, mid, right);
    }
}