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
    println!("done!");
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