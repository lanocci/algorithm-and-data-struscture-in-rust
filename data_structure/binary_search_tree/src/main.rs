use std::io::*;
use util::Scanner;

#[derive(Clone)]
struct Node<'a> {
    key: usize,
    parent: Option<&'a Node<'a>>,
    left: Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn initialize(key: usize) -> Node<'a> {
        Node{
            key: key,
            parent: None.as_ref::<'a>(),
            left: None.as_ref::<'a>(),
            right: None.as_ref::<'a>(),
        }
    }

    fn set_parent(&mut self, node: &'a Node<'a>) {
        self.parent = Some(node);
    }

    fn set_left(&mut self, node: Node<'a>) {
        self.left = Some(&node);
    }

    fn set_right(&mut self, node: Node<'a>) {
        self.right = Some(&node);
    }
}

struct Tree<'a> {
    root: Option<&'a Node<'a>>,
    nodes: Vec<&'a Node<'a>>,
}

impl<'a> Tree<'a> {
    fn new() -> Tree<'a> {
        Tree {
            root: None,
            nodes: Vec::new(),
        }
    }

    fn set_root(&mut self, node: Node<'a>) {
        self.root = Some(&node);
    }

    fn insert(&mut self, mut z: Node<'a>) {
        let mut y: Option<&Node> = None;
        let mut x = self.root;
        while !x.is_none() {
            y = x;
            if z.key < x.unwrap().key {
                x = x.unwrap().left;
            }
            else {
                x  = x.unwrap().right;
            }
        }

        if let Some(p) = y {
            z.set_parent(p);
        }

        if y.is_none() {
            self.set_root(z);
        } else if z.key < y.unwrap().key {
            y.unwrap().set_left(z);
        } else {
            y.unwrap().set_right(z);
        }
    }

    fn print(&self) {
        self.print_preorder(self.root);
        self.print_inorder(self.root);
    }

    fn print_preorder(&self, node: Option<&Node>) {
        print!(" {}", node.unwrap().key);

        if let Some(left) = node.and_then(|r| r.left) {
            self.print_preorder(Some(left));
            print!(" {}", left.key);
        }

        if let Some(right) = node.and_then(|r| r.right) {
            self.print_preorder(Some(right));
            print!(" {}", right.key);
        }
    }

    fn print_inorder(&self, node: Option<&Node>) {
        if let Some(left) = node.and_then(|r| r.left) {
            self.print_inorder(Some(left));
            print!(" {}", left.key);
        }

        print!(" {}", node.unwrap().key);

        if let Some(right) = node.and_then(|r| r.right) {
            self.print_preorder(Some(right));
            print!(" {}", right.key);
        }
    }
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

    let n: usize = sc.read();
    let mut tree = Tree::new();

    for _ in 0..n {
        match sc.read::<String>().as_str() {
            "insert" => {
                tree.insert(Node{ key: sc.read(), parent: None, left: None, right: None });
            },
            "print" => {
                tree.print();
            },
            _ => {
                panic!();
            },
        }
    }
}
