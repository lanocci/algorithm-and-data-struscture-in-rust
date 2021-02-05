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

    let n: usize = sc.read();
    let mut adj_list: Vec<VecDeque<usize>> = vec![VecDeque::new(); n];

    let m: usize = sc.read();

    for i in 0..m {
        let s: usize = sc.read();
        let t: usize = sc.read();

        adj_list[s].push_back(t);
        adj_list[t].push_back(s);
    }

    let q: usize = sc.read();

    for i in 0..q {
        let s: usize = sc.read();
        let t: usize = sc.read();

        let mut statuses = vec![Status::Remaining; n];
        let ans = dfs(&adj_list, s, t, &mut statuses);

        println!("{}", if ans { "yes" } else { "no" });
    }
}

#[derive(Clone, PartialEq)]
enum Status {
    Remaining,
    Visited,
}

fn dfs(adj_list: &Vec<VecDeque<usize>>, s: usize, t: usize, statuses: &mut Vec<Status>) -> bool {
    for &x in adj_list[s].iter() {
        if x == t {
            return true
        } else if statuses[x] == Status::Remaining {
            statuses[s] = Status::Visited;
            return dfs(adj_list, x, t, statuses)
        }
    }
    false
}