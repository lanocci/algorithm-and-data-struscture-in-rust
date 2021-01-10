use std::io;

fn trace(a: &Vec<i32>) -> () {
  for i in 0..a.len() {
    if(i > 0) {print!(" ")};
    print!("{}", a[i as usize]);
  }
  println!("")
}

fn insertion_sort(mut a: Vec<i32>, n: usize) -> () {
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
  println!("please specify size of the array");
  let mut size: String = "".to_string();
  io::stdin()
    .read_line(&mut size)
    .expect("failed to read size line");

  println!("please specify an array to be sorted");
  let mut vec: String = "".to_string();
  io::stdin()
    .read_line(&mut vec)
    .expect("failed to read vector");

  let mut size: usize = size
    .trim()
    .parse()
    .expect("failed to parse size");
  let mut vec: Vec<i32> = vec
    .trim()
    .split(' ')
    .collect::<Vec<&str>>()
    .iter()
    .map(|s| s.parse().expect("failed to parse vec element")).
    collect();
  insertion_sort(vec, size);
}
