use std::io::*;
use std::collections::VecDeque;
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

    let buf = {
        let mut buf: Vec<Vec<usize>> = Vec::new();
        for _ in 0..h {
            let mut row: Vec<usize> = Vec::new();
            for _ in 0..w {
                if sc.read::<usize>() == 0 {
                    row.push(1);
                } else {
                    row.push(0);
                }
            }
            buf.push(row);
        }
        buf
    };

    let result = largest_rectangle(&buf);
    println!("{}", result);
}

fn largest_rectangle(histogram: &Vec<Vec<usize>>) -> usize {
    let mut histogram = histogram.clone();
    let mut size: usize = 0;
    for i in 1..histogram.len() {
        for j in 0..histogram[0].len() {
            histogram[i][j] = (histogram[i - 1][j] + 1) * histogram[i][j];
        }
    }

    for row in histogram.iter() {
        let mut rect: VecDeque<Rect> = VecDeque::new();
        for (i, &tile) in row.iter().enumerate() {
            match rect.back() {
                None => {
                    rect.push_back(Rect{height: tile, left_idx: i});
                },
                Some(r) => {
                    if r.height < tile {
                        rect.push_back(Rect{height: tile, left_idx: i});
                    } else if r.height > tile {
                        let mut target = i;
                        while let Some(prev) = rect.pop_back() {
                            if prev.height < tile {
                                break;
                            } else {
                                size = std::cmp::max(size, prev.height * (i - prev.left_idx));
                                target = prev.left_idx;
                            }
                        }
                        rect.push_back(Rect{height: tile, left_idx: target});
                    }
                }
            }
        }
    }
    size
}

struct Rect{height: usize, left_idx: usize}