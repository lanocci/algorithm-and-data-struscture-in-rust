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

    let h: usize = sc.read();

    let nodes: Vec<i32> = sc.vec(h);

    let tree: CompleteBinaryTree<i32> = CompleteBinaryTree::new(nodes);

}
