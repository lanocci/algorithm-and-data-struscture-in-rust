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

    let h: usize = sc.read();
    let w: usize = sc.read();

    let tiles = {
        let mut tiles: Vec<Vec<usize>> = Vec::new();
        for _ in 0..h {
            let mut row: Vec<usize> = Vec::new();
            for _ in 0..w {
                row.push(sc.read());
            }
            tiles.push(row);
        }
        tiles
    };

    let result = largest_square(&tiles);
    println!("{}", result);
}

fn largest_square(tiles: &Vec<Vec<usize>>) -> usize {
    let mut max_width = 0;
    let mut dp = {
        let mut dp: Vec<Vec<usize>> = Vec::new();
        for row in tiles.iter() {
            let mut r: Vec<usize> = Vec::new();
            for tile in row.iter() {
                if tile % 2 == 0 {
                    r.push(1);
                } else {
                    r.push(0);
                }
            }
            dp.push(r);
        }
        dp
    };

    for (i, row) in tiles.iter().enumerate().skip(1) {
        for (j, &tile) in row.iter().enumerate().skip(1) {
            if tile == 0 {
                let mut minimum_neighbor = std::cmp::min(dp[i][j - 1], dp[i - 1][j]);
                minimum_neighbor = std::cmp::min(minimum_neighbor, dp[i-1][j-1]);
                dp[i][j] = minimum_neighbor + 1;
                max_width = std::cmp::max(minimum_neighbor + 1, max_width);
            }
        }
    }
    max_width.pow(2)
}
