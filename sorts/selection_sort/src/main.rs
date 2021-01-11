use std::io;
use common::{trace, UserInput, handle_input};

fn selection_sort(mut a: Vec<i32>, n: usize) -> () {
  for i in 1..n {
    let mut v = a[i as usize];
    let mut j: usize = i as usize - 1;
    while(j >= 0 && a[j as usize] > v) {
      a[j+1] = a[j as usize];
      j -= 1;
    }
    a[j+1 as usize] = v;
    trace(&a);
  }
}

fn main() {
    let input = handle_input();
    selection_sort(input.vec, input.size);
}
