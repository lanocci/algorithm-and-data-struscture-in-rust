use std::io::*;
use util::Scanner;
use complete_binary_tree::CompleteBinaryTree;

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
    
    let h = sc.read();

    let nodes: Vec<i32> = sc.vec(h);
    let tree: CompleteBinaryTree<i32> = CompleteBinaryTree::new(nodes);

    for i in 0..h {
        if i == 0 {
            println!("node 1: key = {}, left key = {}, right key = {},", tree.get(i).unwrap(), tree.left(i).unwrap(), tree.right(i).unwrap());
        } else if let Some(v) = tree.right(i) {
            println!("node {}: key = {}, parent key = {}, left key = {}, right key = {},", i + 1, tree.get(i).unwrap(), tree.parent(i).unwrap(), tree.left(i).unwrap(), v);
        } else if let Some(v) = tree.left(i) {
            println!("node {}: key = {}, parent key = {}, left key = {},", i + 1, tree.get(i).unwrap(), tree.parent(i).unwrap(), v);
        } else {
            println!("node {}: key = {}, parent key = {},", i + 1, tree.get(i).unwrap(), tree.parent(i).unwrap());
        }
    }
}