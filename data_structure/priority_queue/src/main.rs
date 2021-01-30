use std::io::*;
use util::{Scanner, Joinable};
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

    let mut queue: CompleteBinaryTree<i32> = CompleteBinaryTree::new(Vec::new());

    loop {
        match sc.read::<String>().as_str() {
            "insert" => {
                let key = sc.read();
                queue.insert(key);
                println!("{}", queue.nodes.iter().join(" "));
            },
            "extract" => {
                let extracted = queue.extract();
                println!("{}", extracted);
            },
            "end" => {
                break;
            },
            _ => unreachable!()

        }
    }
}