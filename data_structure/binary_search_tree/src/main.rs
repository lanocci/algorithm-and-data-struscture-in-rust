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

#[derive(PartialEq)]
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

    fn delete(&mut self, given: T) {
        let mut replace_target: Option<Rc<RefCell<BST<T>>>> = None;
        let mut should_delete_self = false;
        if let Self::Node {
            ref mut key,
            ref mut left,
            ref mut right,
            ..
        } = self {
            if &given == key {
                if *left.borrow() == Self::Nil && *right.borrow() == Self::Nil {
                    should_delete_self = true;
                } else if *left.borrow() == Self::Nil {
                    replace_target = Some(Rc::clone(right));
                } else if *right.borrow() == Self::Nil {
                    replace_target = Some(Rc::clone(left));
                } else {
                    let r = Rc::clone(right);
                    let successor = self.get_successor();
                    match *successor.borrow_mut() {
                        s@Self::Node{..} => {
                            let sv = *s.key;
                            s.delete(sv);
                        }
                        Self::Nil => {
                            ();
                        }
                    }
                }
            } else if &given < key {
                let l = left.clone();
                l.borrow_mut().delete(given);
            } else {
                let r = right.clone();
                r.borrow_mut().delete(given);
            }
        }
        if should_delete_self {
            *self = Self::Nil;
        } else if let Some(target) = replace_target {
            if let Self::Node {ref key, ref parent, ref left, ref right} = *target.borrow() {
                *self = Self::Node {
                    key: *key,
                    parent: Rc::clone(parent),
                    left: Rc::clone(left),
                    right: Rc::clone(right),
                }
            }
        }
    }

    fn get_successor(&self) -> Rc<RefCell<Self>> {
        match self {
            Self::Nil => {
                self
            },
            Self::Node { ref right, ..} => {
                match *right.borrow() {
                    Self::Node { ref right: r, .. } => {
                        let r = *right.borrow();
                        r.get_minimum()
                    },
                    Self::Nil => {
                        let mut y = self.parent.clone();
                        loop {
                            let x = y;
                            y = *y.parent.clone();
                            if let Self::Nil = *y.borrow() {
                                if  *y.borrow().left == *x.borrow() {
                                    break;
                                }
                            }
                        }
                        y.clone()
                    }

                }
            }
        }
//        let mut next = None;
//        if let Self::Node {ref left, ref key,..} = self {
//            if let Self::Node {left: ref l,..} = *left.borrow() {
//                next = Some(l.clone());
//            }
//            if let Some(n) = next {
//                let n = Rc::clone(&n);
//                let n = n.borrow();
//                n.get_minimum()
//            } else {
//                Some(*key)
//            }
//        } else {
//            None
//        }
    }

    fn get_minimum(&self) -> Rc<RefCell<Self>> {
        if let Self::Node {ref left, ..} = self {
            if let Self::Node {ref l, ..} = left {
                l.borrow().get_minimum()
            } else {
                left.clone()
            }
        } else {
            Self::Nil
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
