use std::io::*;
use std::fmt;
use util::Scanner;

fn main() {
    std::thread::Builder::new()
        .stack_size(1048576)
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}

fn solve() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let n: usize = sc.read();

    let mut matrix: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut colors: Vec<Color> = vec![Color::White; n];
    let mut discover: Vec<usize> = vec![0; n];
    let mut finish: Vec<usize> = vec![0; n];

    for _ in 0..n {
        let idx: usize = sc.read();
        let k: usize = sc.read();
        let adj: Vec<usize> = sc.vec(k);
        for &aid in adj.iter() {
            matrix[idx - 1][aid - 1] = true;
        }
    }
    dfs_init(&mut matrix, &mut colors, &mut discover, &mut finish);

    for i in 0..n {
        println!("{} {} {}", i + 1, discover[i], finish[i]);
    }

}

#[derive(Clone, PartialEq)]
enum Color {
    White,
    Black,
    Grey,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Color::White => "White",
            Color::Black => "Black",
            Color::Grey => "Grey",
        };
        write!(f, "{}", printable)
    }
}

fn dfs_init(matrix: &Vec<Vec<bool>>, colors: &mut Vec<Color>, discover: &mut Vec<usize>, finish: &mut Vec<usize>) {
    let mut time: usize = 0;
    dfs(matrix, colors, discover, finish, 0, &mut time);
}

fn dfs(matrix: &Vec<Vec<bool>>, colors: &mut Vec<Color>, discover: &mut Vec<usize>, finish: &mut Vec<usize>, idx: usize, time: &mut usize) {
    colors[idx] = Color::Grey;
    *time += 1;
    discover[idx] = *time;
    for i in 0..matrix[idx].len() {
        if matrix[idx][i] && colors[i] == Color::White {
            dfs(matrix, colors, discover, finish, i, time);
        }
    }
    colors[idx] = Color::Black;
    *time += 1;
    finish[idx] = *time;
}