use std::io;
use common::{trace, UserInput, handle_input};

fn selection_sort(mut a: Vec<i32>) -> () {
    for i in 0..a.len() {
        let mut minj = i;
        for j in i..a.len() {
            if a[j] < a[minj] {
                minj = j;
            }
        }
        let tmp = a[minj];
        a[minj] = a[i];
        a[i] = tmp;
        trace(&a);
    }
}

fn main() {
    let input = handle_input();
    selection_sort(input.vec);
}
