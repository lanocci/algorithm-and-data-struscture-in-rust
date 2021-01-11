use std::io;
use std::fmt::Display;

pub fn trace<T>(a: &Vec<T>) -> () 
    where T: Display {
    for i in 0..a.len() {
      if(i > 0) {print!(" ")};
      print!("{}", a[i]);
    }
    println!("")
}

//pub fn trace(a: &Vec<String>) -> () {
//  for i in 0..a.len() {
//    if(i > 0) {print!(" ")};
//    print!("{}", a[i]);
//  }
//  println!("")
//}

pub struct UserInput {
    pub vec: Vec<i32>,
    pub size: usize
}

pub fn handle_numeric_input() -> UserInput {
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
  UserInput {
      vec,
      size
  }
}

pub struct UserStringInput {
    pub vec: Vec<String>,
    pub size: usize,
}

pub fn handle_string_input() -> UserStringInput {
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
  let mut vec: Vec<String> = vec
    .trim()
    .split(' ')
    .collect::<Vec<&str>>()
    .iter()
    .map(|s| s.to_string())
    .collect();
  UserStringInput {
      vec,
      size
  }
}
