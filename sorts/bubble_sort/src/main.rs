use std::io;
use common::{trace, UserInput, handle_input};

fn bubble_sort(mut a: Vec<i32>) -> () {
    let mut flag: bool = true;
    let n: usize = a.len();
    while flag {
        flag = false;
        for j in (1..n).rev() {
            if a[j - 1] > a[j] {
                let tmp = a[j];
                a[j] = a[j - 1];
                a[j - 1] = tmp;
                flag = true;
            }
        }
        trace(&a);
    }
}

fn main() {
    let input = handle_input();
    bubble_sort(input.vec);
}
