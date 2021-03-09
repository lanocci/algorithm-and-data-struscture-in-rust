use util::{Scanner, Joinable};

fn insertion_sort(a: &Vec<i32>) -> Vec<i32> {
  let mut result = a.clone();
  let n = a.len() as i32;
  for i in 1..n {
    let v = result[i as usize];
    let mut j: i32 = i - 1;
    while j >= 0 && result[j as usize] > v {
      result[(j+1) as usize] = result[j as usize];
      j -= 1;
    }
    result[(j+1) as usize] = v;
  }
  result
}

fn main() {
  let cin = std::io::stdin();
  let cin = cin.lock();
  let mut sc = Scanner::new(cin);
  let n = sc.read();
  let vec = sc.vec(n);
  let result = insertion_sort(&vec);
  println!("{}", result.iter().join(" "));
}
