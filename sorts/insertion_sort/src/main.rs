use common::{trace, UserInput, handle_numeric_input};

fn insertion_sort(mut a: Vec<i32>, n: usize) -> () {
  for i in 1..n {
    let mut v = a[i];
    let mut j: usize = i - 1;
    while(j >= 0 && a[j] > v) {
      a[j+1] = a[j];
      j -= 1;
    }
    a[j+1] = v;
    trace(&a);
  }
}

fn main() {
  let input: UserInput = handle_numeric_input();
  insertion_sort(input.vec, input.size);
}
