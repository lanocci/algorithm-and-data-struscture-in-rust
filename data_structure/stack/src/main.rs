use std::io;

struct Stack {
  items: Vec<i32>,
}

pub fn handle_string_input() -> Vec<String> {
  println!("please type a reverse polish notation");
  let mut vec: String = "".to_string();
  io::stdin()
    .read_line(&mut vec)
    .expect("failed to read vector");

  vec
    .trim()
    .split(' ')
    .collect::<Vec<&str>>()
    .iter()
    .map(|s| s.to_string())
    .collect()
}

fn main() {
    let mut input = handle_string_input();
    let mut stack = Stack{ items: vec![] };
    for item in input.iter() {
      match item.parse::<i32>() {
        Ok(n) => stack.items.push(n),
        Err(err) => {
          if item == "*" {
            let right = stack.items.pop().unwrap();
            let left = stack.items.pop().unwrap();
            stack.items.push(left * right);
          }
          else if item == "+" {
            let right = stack.items.pop().unwrap();
            let left = stack.items.pop().unwrap();
            stack.items.push(left + right);
          }
          else if item == "/" {
            let right = stack.items.pop().unwrap();
            let left = stack.items.pop().unwrap();
            stack.items.push(left / right);
          }
          else if item == "-" {
            let right = stack.items.pop().unwrap();
            let left = stack.items.pop().unwrap();
            stack.items.push(left - right);
          }
        }
      }
    }
    println!("result: {}", stack.items.pop().unwrap());
}
