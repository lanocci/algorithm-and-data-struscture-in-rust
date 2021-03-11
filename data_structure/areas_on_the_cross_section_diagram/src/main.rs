use std::io::*;
use util::{Scanner, Joinable};

fn main() {
    std::thread::Builder::new()
        .stack_size(1048576)
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}

/// example input: `\\///\_/\/\\\\/_/\\///__\\\_\\/_\/_/\`
/// output: 
/// ```
/// 35
/// 5 4 2 1 19 9
/// ```
fn solve() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let mut slopes: Vec<usize> = Vec::new();
    let mut areas: Vec<(usize, usize)> = Vec::new();

    let input: String = sc.read();
    let input: Vec<char> = input.chars().collect();

    let mut sum = 0;

    for i in 0..input.len() {
        if input[i] == '\\' { 
            slopes.push(i);
         }
        else if input[i] == '/' && !slopes.is_empty() {
            let j = slopes.pop().unwrap();
            let mut a = i - j;
            sum += a;
            while let Some((corresonding_down_slope_location, area)) = areas.last() {
                if corresonding_down_slope_location <= &j { break; }
                else {
                    a += area;
                    areas.pop();
                }
            }
            areas.push((j, a));
        }
    }
    //areas.reverse();
    println!("{}", sum);
    print!("{} ", areas.len());
    println!("{}", areas.iter().map(|(_, a)| a).join(" "));
}

