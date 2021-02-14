use std::io::*;
use std::cmp::min;
use util::{Joinable, Scanner};

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
    
    let v = sc.read();
    let e = sc.read();

    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); v];
    for _ in 0..e {
        let s: usize = sc.read();
        let t: usize = sc.read();
        edges[s].push(t);
        edges[t].push(s);
    }
    let dfs_tree = DFSTree::new(edges, v);
    let arts = dfs_tree.find_articulation_point();
    for a in arts.iter() {
        println!("{}", a);
    }
}

struct DFSTree {
    prenum: Vec<Option<usize>>,
    parent: Vec<Option<usize>>,
    lowlink: Vec<usize>,
}

impl DFSTree {
    fn new(edges: Vec<Vec<usize>>, v: usize) -> DFSTree {
        let mut prenum: Vec<Option<usize>> = vec![None; v];
        let mut parent: Vec<Option<usize>> = vec![None; v];
        let mut lowlink: Vec<usize> = vec![usize::MAX; v];
        Self::dfs(&edges, 0, 0, &mut prenum, &mut parent, &mut lowlink);
        DFSTree { prenum, parent, lowlink }
    }

    fn dfs(edges: &Vec<Vec<usize>>, current: usize, time: usize, prenum: &mut Vec<Option<usize>>, parent: &mut Vec<Option<usize>>, lowlink: &mut Vec<usize>) {
        prenum[current] = Some(time);
        for &next in edges[current].iter() {
            match prenum[next] {
                None => {
                    parent[next] = Some(current);
                    Self::dfs(edges, next, time + 1, prenum, parent, lowlink);
                    lowlink[idx] = min(lowlink[current], lowlink[next]);
                },
                Some(n) => {
                    lowlink[current] = min(lowlink[current], n);
                }
            }
        }
    }

    fn find_articulation_point(&self) -> Vec<usize> {
        let n = self.prenum.len();
        let mut ans: Vec<usize> = Vec::new();

        let mut root_children = 0;
        for i in 1..n {
            if let Some(p) = self.parent[i] {
                if p == 0 { root_children += 1; }
                else if self.prenum[p].unwrap() <= self.lowlink[i] { ans.push(p); }
            }
        }
        if root_children > 1 { ans.push(0); }
        ans
    }
}