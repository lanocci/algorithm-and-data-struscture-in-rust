use std::io::*;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;
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
    let mut tree: BST::<i32> = BST::new();

    for _ in 0..n {
        let opr: String = sc.read();
        match opr.as_str() {
            "insert" => {
                let key = sc.read();
                tree.insert(key);
            }
            "print" => {
                tree.print();
            }
            "find" => {
                let key = sc.read();
                tree.find(key);
            }
            _ => {
                panic!();
            }
        }
    }

}

#[derive(PartialEq)]
enum BST<T> where T: Ord + Display + Copy + PartialEq {
    Nil,
    Node {
        key: T,
        left: Rc<RefCell<BST<T>>>,
        right: Rc<RefCell<BST<T>>>,
    },
}

impl<T> BST<T> where T: Ord + Display + Copy + PartialEq {
    fn new() -> Self {
        Self::Nil
    }

    fn insert(&mut self, z: T) {
        match self {
            Self::Nil => {
                *self = Self::Node {
                    key: z,
                    left: Rc::new(RefCell::new(Self::Nil)),
                    right: Rc::new(RefCell::new(Self::Nil)),
                }
            },
            Self::Node {
                ref mut key,
                ref mut left,
                ref mut right,
            } => {
                if z < *key {
                    left.borrow_mut().insert(z);
                } else {
                    right.borrow_mut().insert(z);
                }
            }
        }
    }

    fn find(&self, given: T) {
        match self {
            Self::Node {
                ref key,
                ref left,
                ref right,
            } => {
                if &given == key {
                    println!("yes");
                } else if &given < key  {
                    left.borrow().find(given);
                } else {
                    right.borrow().find(given);
                }
            },
            Self::Nil => {
                println!("no");
            }
        }
    }

    fn delete(&mut self, given: T) {
        if let Self::Node {
            ref mut key,
            ref mut left,
            ref mut right,
        } = self {
            if &given == key {
                if *left.borrow() == Self::Nil && *right.borrow() == Self::Nil {
                    *self = Self::Nil;
                } else if *left.borrow() == Self::Nil {
                    *self = *right.borrow();
                } else if *right.borrow() == Self::Nil {
                    *self = *left.borrow();
                } else {
                    let r = *right.borrow_mut();
                    *key = r.get_key();
                    r.delete(*key);
                }
            } else if &given < key {
                left.borrow_mut().delete(given);
            } else {
                right.borrow_mut().delete(given);
            }
        }
    }

    fn get_key(&self) -> T {
        match self {
            Self::Nil => {
                panic!();
            },
            Self::Node { key, .. } => {
                *key
            }
        }
    }

    fn print(&self) {
        self.print_inorder();
        println!("");
        self.print_preorder();
        println!("");
    }

    fn print_inorder(&self) {
        if let Self::Node { 
            ref key,
            ref left,
            ref right,
        } = self {
            left.borrow().print_inorder();
            print!(" {}", key);
            right.borrow().print_inorder();
        }
    }

    fn print_preorder(&self) {
        if let Self::Node { 
            ref key,
            ref left,
            ref right,
        } = self {
            print!(" {}", key);
            left.borrow().print_preorder();
            right.borrow().print_preorder();
        }
    }
}
