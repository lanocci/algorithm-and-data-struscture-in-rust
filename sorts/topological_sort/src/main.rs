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

    let v = sc.read();
    let e = sc.read();

    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); v];

    for _ in 0..e {
        let s: usize = sc.read();
        let t: usize = sc.read();

        edges[s].push(t);
    }

    let ans = topological_sort(v, &edges);

    for a in ans.iter() {
        println!("{}", a);
    }


}

/// ## Topological Sort
/// 
/// - DAG(閉路のない有向グラフ) において、全ての頂点を順番通りに一列に並べるソート
/// - 入次数が0の（別のどの頂点にも依存していない）頂点から調べる
/// - 辺をたどって、次の頂点がその頂点にのみ依存していたらその頂点を訪問する
/// - 訪問: その頂点を `訪問済み` にマークして、ソート結果の末尾に追加する
/// - 辺を辿る時、幅優先探索でも、深さ優先探索でも良いが、再帰を用いると、ネストが深いときにスタックオーバーフローの恐れがあるので幅優先探索が適している

fn topological_sort(v: usize, edges: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited = vec![false; v];
    let mut indeg = vec![0; v];

    for e in edges.iter() {
        for &to in e.iter() {
            indeg[to] += 1;
        }
    }

    let mut out = Vec::new();
    
    for u in 0..v {
        if indeg[u] == 0 && !visited[u] {
            out.append(&mut bfs(u, &mut visited, &mut indeg, &edges));
        }
    }
    out
}

fn bfs(s: usize, visited: &mut Vec<bool>, indeg: &mut Vec<usize>, edges: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut out: Vec<usize> = Vec::new();

    q.push_back(s);
    visited[s] = true;

    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        out.push(u);

        for &v in edges[u].iter() {
            indeg[v] -= 1;
            if indeg[v] == 0 && !visited[v] {
                visited[v] = true;
                q.push_back(v);
            }
        }
    }
    out
}
