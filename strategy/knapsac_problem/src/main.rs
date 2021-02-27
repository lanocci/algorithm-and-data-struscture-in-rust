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
    let weights: usize = sc.read();

    let items = {
        let mut items: Vec<Item> = Vec::new();

        for _ in 0..n {
            items.push(Item{value: sc.read(), weight: sc.read()});
        }
        items
    };

    let (c, g) = {
        let mut c: Vec<Vec<usize>> = vec![vec![0; weights + 1]; n + 1];
        let mut g: Vec<Vec<bool>> = vec![vec![false; weights + 1]; n + 1];
        for (i, item) in items.iter().enumerate() {
            for w in 1..(weights + 1) {
                if w >= item.weight {
                    if c[i][w] < c[i][w - item.weight] + item.value {
                        g[i + 1][w] = true;
                        c[i + 1][w] = c[i][w - item.weight] + item.value;
                    } else {
                        c[i + 1][w] = c[i][w];
                    }
                } else {
                    c[i + 1][w] = c[i][w];
                }
            }
        }
        (c, g)
    };

    println!("{}", c[n][weights]);
}

struct Item {
    value: usize,
    weight: usize,
}