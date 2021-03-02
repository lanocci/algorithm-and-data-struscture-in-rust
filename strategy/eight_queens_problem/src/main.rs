use std::io::*;
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

    let presets = {
        let mut presets: Vec<Position> = Vec::new();
        for _ in 0..n{
            let r = sc.read();
            let c = sc.read();
            presets.push(Position(c, r));
        }
        presets
    };

    eight_queen(&presets);
    //for i in 0..8 {
    //    println!("{}Q{}", ".".repeat(result[i]), ".".repeat(7 - result[i]));
    //}
}

struct Position(usize, usize);

fn eight_queen(presets: &Vec<Position>) {
    let mut row: Vec<usize> = Vec::new();
    let mut col_free: Vec<bool> = vec![true; 8];
    let mut dpos_free: Vec<bool> = vec![true; 15];
    let mut dneg_free: Vec<bool> = vec![true; 15];
    let mut preset = vec![vec![false; 8]; 8];

    for p in presets.iter() {
        preset[p.0][p.1] = true;
    }

    backtrack(0, &mut row, &mut col_free, &mut dpos_free, &mut dneg_free, &preset);

}

fn backtrack(i: usize, row: &mut Vec<usize>, col_free: &mut Vec<bool>, dpos_free: &mut Vec<bool>, dneg_free: &mut Vec<bool>, preset: &Vec<Vec<bool>>) {
    if i == 8 { 
        print_board(&preset, &row);
        return;
    }

    for j in 0..8 {
        if !col_free[j] || !dpos_free[i + j] || !dneg_free[7 + i - j] { continue; }
        println!("i: {} j: {}", i, j);
        row.push(j);
        col_free[j] = false;
        dpos_free[i + j] = false;
        dneg_free[7 + i - j] = false;
        backtrack(i + 1, row, col_free, dpos_free, dneg_free, preset);
        row.pop();
        col_free[i] = true;
        dpos_free[i + j] = true;
        dneg_free[7 + i - j] = true;
    }
}

fn print_board(preset: &Vec<Vec<bool>>, rows: &Vec<usize>) {
    for i in 0..8 {
        for j in 0..8 {
            if preset[i][j] {
                if rows[i] != j { return; }
            }
        }
    }
    for i in 0..8 {
        println!("{}Q{}", ".".repeat(rows[i]), ".".repeat(7 - rows[i]));
    }
}