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
    let mut tree: BST<i32> = BST::new();

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
            "delete" => {
                let key = sc.read();
                tree.delete(key);
            }
            _ => {
                panic!();
            }
        }
    }

}

#[derive(PartialEq, Clone)]
enum BST<T> where T: Ord + Display + Copy + PartialEq {
    Nil,
    Node {
        key: T,
        parent: Rc<RefCell<BST<T>>>,
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
                    parent: Rc::new(RefCell::new(Self::Nil)),
                    left: Rc::new(RefCell::new(Self::Nil)),
                    right: Rc::new(RefCell::new(Self::Nil)),
                }
            },
            Self::Node {
                ref mut key,
                ref mut left,
                ref mut right,
                ..
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
                ..
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

    fn get_left(&self) -> Rc<RefCell<Self>> {
        match self {
            Self::Nil => {
                Rc::new(RefCell::new(Self::Nil))
            },
            Self::Node{ref left, ..}  => {
                left.clone()
            }
        }
    }

    fn get_right(&self) -> Rc<RefCell<Self>> {
        match self {
            Self::Nil => {
                Rc::new(RefCell::new(Self::Nil))
            },
            Self::Node{ref right, ..}  => {
                right.clone()
            }
        }
    }

    fn set_right(&mut self, right: Rc<RefCell<Self>>) {
        if let Self::Node{right: ref mut r, ..} = self {
            *r = right;
        }
    }

    fn get_key(&self) -> Option<T> {
        match self {
            Self::Nil => {
                None
            },
            Self::Node {ref key, ..} => {
                Some(*key)
            }
        }
    }
    fn set_key(&mut self, new_key: T) {
        if let Self::Node {ref mut key, ..} = self {
            *key = new_key;
        }
    }

    fn delete(&mut self, given: T) {
        self.delete_rec(given);
    }

    fn delete_rec(&mut self, key: T) {
        if let s@Self::Node{..} = self.clone() {
            if key == s.get_key().unwrap() {
                let tmp = self.clone();
                self.delete_node(tmp);
            } else if key < s.get_key().unwrap() {
                let l = s.get_left().clone();
                l.borrow_mut().delete(key);
            } else {
                let r = s.get_right().clone();
                r.borrow_mut().delete(key);
            }
        }
    }

    fn delete_node(&mut self, mut node: Self) {
        if let Self::Node{ref mut left, ref mut right, ..} = node {
            match (&*left.borrow_mut(), &*right.borrow_mut()) {
                (Self::Nil, Self::Nil) => {
                    *self = Self::Nil;
                },
                (Self::Node{ref parent, ref left, ref right, ref key}, Self::Nil) | (Self::Nil, Self::Node{ref parent,ref  left,ref  right,ref  key}) => {
                    *self = Self::Node {
                        parent: parent.clone(),
                        left: left.clone(), 
                        right: right.clone(), 
                        key: *key
                    };
                },
                (Self::Node{..}, r@Self::Node{..}) => {
                    let successor = r.get_successor();
                    let new_key = successor.get_key().unwrap();
                    let mut r = r.clone();
                    r.delete(new_key);
                    self.set_right(Rc::new(RefCell::new(r)));
                    self.set_key(new_key);
                }
            }
        }
    }

    fn get_successor(&self) -> Self {
        match self {
            Self::Nil => {
                unreachable!();
            },
            Self::Node {..} => {
                self.get_minimum()
            }
        }
    }

    fn get_minimum(&self) -> Self {
        if let Self::Node {left, ..} = self {
            match *left.borrow() {
                Self::Node{..} => {
                    Self::get_minimum(&left.borrow())
                },
                Self::Nil => {
                    self.clone()
                }
            }
        } else {
            unreachable!();
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
            ..
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
            ..
        } = self {
            print!(" {}", key);
            left.borrow().print_preorder();
            right.borrow().print_preorder();
        }
    }
}
