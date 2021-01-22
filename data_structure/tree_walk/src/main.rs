use std::io::*;
use util::{Scanner, Joinable};

#[derive(Clone)]
struct Node {
    pub parent: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

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

    let n = sc.read();

    let mut nodes: Vec<Node> = vec![Node{ parent: None, left: None, right: None}; n];

    for _ in 0..n {
        let i: usize = sc.read();
        let left: i32 = sc.read();
        if left >= 0 {
            nodes[i].left = Some(left as usize);
            nodes[left as usize].parent = Some(i);
        }
        let right: i32 = sc.read();
        if right >= 0 {
            nodes[i].right = Some(right as usize);
            nodes[right as usize].parent = Some(i);
        }
    }
    let mut root_idx = 0;
    for i in 0..n {
        if nodes[i].parent.is_none() {
            root_idx = i;
        }
    }

    println!("Preorder");
    println!(" {}", preorder(&nodes, root_idx).iter().join(" "));

    println!("Inorder");
    println!(" {}", inorder(&nodes, root_idx).iter().join(" "));

    println!("Postorder");
    println!(" {}", postorder(&nodes, root_idx).iter().join(" "));

}

fn preorder(nodes: &Vec<Node>, idx: usize) -> Vec<usize> {
    let mut lefts: Vec<usize> = if let Some(left) = nodes[idx].left {
        preorder(nodes, left)
    } else {
        Vec::new()
    };

    let mut rights: Vec<usize> = if let Some(right) = nodes[idx].right {
        preorder(nodes, right)
    } else {
        Vec::new()
    };
    let mut result = vec![idx];
    result.append(&mut lefts);
    result.append(&mut rights);
    result

}

fn inorder(nodes: &Vec<Node>, idx: usize) -> Vec<usize> {
    let mut lefts: Vec<usize> = if let Some(left) = nodes[idx].left {
        inorder(nodes, left)
    } else {
        Vec::new()
    };

    let mut rights: Vec<usize> = if let Some(right) = nodes[idx].right {
        inorder(nodes, right)
    } else {
        Vec::new()
    };

    let mut result = lefts;
    result.push(idx);
    result.append(&mut rights);
    result
}

fn postorder(nodes: &Vec<Node>, idx: usize) -> Vec<usize> {
    let mut lefts: Vec<usize> = if let Some(left) = nodes[idx].left {
        postorder(nodes, left)
    } else {
        Vec::new()
    };

    let mut rights: Vec<usize> = if let Some(right) = nodes[idx].right {
        postorder(nodes, right)
    } else {
        Vec::new()
    };

    let mut result = lefts;
    result.append(&mut rights);
    result.push(idx);
    result
}