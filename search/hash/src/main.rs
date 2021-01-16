use std::io::*;
use util::Scanner;

fn main() {
    std::thread::Builder::new()
        .stack_size(1073741824)
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}

fn solve() {
    const MAX_LENGTH: u32 = 1_000_000;

    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let n = sc.read();
    let mut vec: Vec<String> = Vec::with_capacity(MAX_LENGTH as usize);
    for _ in 0..MAX_LENGTH {
        vec.push(String::from(""));
    }

    for _ in 0..n {
        let op: String = sc.read();
        match op.as_str() {
            "insert" => {
                let key = sc.read();
                let mut i = 0;
                loop {
                    let hash = open_hash(&key, i, MAX_LENGTH);
                    match vec.get(hash) {
                        Some(_) => {
                            vec[hash] = key.clone();
                            break;
                        }
                        None => {
                            i += 1;
                        }
                    }
                }
            }
            "find" => {
                let key = sc.read();
                let mut i = 0;
                loop {
                    let hash = open_hash(&key, i, MAX_LENGTH);
                    println!("{}", hash);
                    match vec.get(hash) {
                        Some(found) => {
                            if &key == found {
                                println!("yes");
                                break;
                            }
                            else if found == &String::from("") {
                                println!("no");
                                break;
                            } else {
                                i += 1;
                            }
                        }
                        None => {
                            panic!()
                        }
                    }
                }
            }
            _ => {
                panic!()
            }
        }
    }

}

fn open_hash(key: &String, i: u32, m: u32) -> usize {
    (hash_1(key, m) + i as usize * hash_2(key, m)) % m as usize
}

fn hash_1(key: &String, m: u32) -> usize {
    let mut r: u32 = 0;
    for ch in key.chars() {
        r <<= 6;
        r += ch.to_digit(36).unwrap();
    }
   (r % m) as usize
}

fn hash_2(key: &String, m: u32) -> usize {
    let mut r: u32 = 0;
    for ch in key.chars() {
        r <<= 6;
        r += ch.to_digit(36).unwrap();
    }
    (1 + (r % (m - 1))) as usize
}
