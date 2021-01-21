use std::io::*;
use util::{Scanner, Joinable};

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

    let mut nodes: Vec<Node> = vec![Node{ parent: None, left_child: None, right_sibling: None}; n];
    let mut depths: Vec<usize> = vec![0; n];

    for _ in 0..n {
        let id: usize = sc.read();
        let degree: usize = sc.read();
        let mut left_sibling: Option<usize> = None;
        for j in 0..degree {
            let child: usize = sc.read();
            if j == 0 {
                nodes[id].set_left_child(child);
            }
            else {
                nodes[left_sibling.unwrap()].set_right_sibling(child);
            }
            left_sibling = Some(child);
            nodes[child].set_parent(id);
        }
    }
    let mut root = 0;
    for i in 0..n {
        if nodes[i].parent.is_none() {
            root = i;
        }
    }
    calc_depth(&mut nodes, &mut depths, root, 0);

    let get_children = |node: &Node| -> Vec<usize> {
        let mut children: Vec<usize> = Vec::new();
        if let Some(child) = node.left_child {
            children.push(child);
            let mut sibling = child;
            loop {
                match nodes[sibling].right_sibling {
                    Some(r) => {
                        children.push(r);
                        sibling = r;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        children
    };
    
    for i in 0..n {
        let node = &nodes[i];
        let depth = depths[i];
        let children = get_children(node);
        println!("node {}: parent = {}, depth = {}, {}, [{}]", i, node.parent.map(|p| p.to_string()).unwrap_or(String::from("root")), depth, node_type(&node), children.iter().join(", "));
    }

}

fn calc_depth(nodes: &mut Vec<Node>, depths: &mut Vec<usize>, id: usize, prev_depth: usize) {
    depths[id] = prev_depth;
    if let Some(child) = nodes[id].left_child {
        calc_depth(nodes, depths, child, prev_depth + 1);
    }
    if let Some(sibling) = nodes[id].right_sibling {
        calc_depth(nodes, depths, sibling, prev_depth);
    }
}

fn node_type(node: &Node) -> &str {
    match node.parent {
        None => "root",
        Some(_) => {
            match node.left_child {
                None => "leaf",
                Some(_) => "internal node"
            }
        }
    }
}

#[derive(Clone)]
struct Node {
    pub parent: Option<usize>,
    pub left_child: Option<usize>,
    pub right_sibling: Option<usize>,
}

impl Node {
    fn set_parent(&mut self, parent: usize) {
        self.parent = Some(parent);
    }

    fn set_left_child(&mut self, left: usize) {
        self.left_child = Some(left);
    }

    fn set_right_sibling(&mut self, right: usize) {
        self.right_sibling = Some(right);
    }
}