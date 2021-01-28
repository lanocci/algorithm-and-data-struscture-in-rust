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

    fn get_parent(&self) -> Rc<RefCell<Self>> {
        match self {
            Self::Nil => {
                Rc::new(RefCell::new(Self::Nil))
            },
            Self::Node{ref parent, ..}  => {
                parent.clone()
            }
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
        let n = self.clone();
        self.delete_rec(n, given);
//        let mut replace_target: Option<Rc<RefCell<BST<T>>>> = None;
//        let mut should_delete_self = false;
//        if should_delete_self {
//            *node.borrow_mut() = Self::Nil;
//        } else if let Some(target) = replace_target {
//            if let Self::Node {ref key, ref parent, ref left, ref right} = *target.borrow() {
//                *node.borrow_mut() = Self::Node {
//                    key: *key,
//                    parent: Rc::clone(parent),
//                    left: Rc::clone(left),
//                    right: Rc::clone(right),
//                }
//            }
//        }
    }

    fn delete_rec(&mut self, node: Self, key: T) {
        if let s@Self::Node{..} = self.clone() {
            println!("rec top unwrap");
            if key == s.get_key().unwrap() {
//                if *s.get_left().borrow() == Self::Nil && *s.get_right().borrow() == Self::Nil {
                let tmp = node.clone();
                self.delete_node(tmp);
                //} else if *s.get_left().borrow() == Self::Nil {
                //    replace_target = Some(Rc::clone(&s.get_right()));
                //} else if *s.get_right().borrow() == Self::Nil {
                //    replace_target = Some(Rc::clone(&s.get_left()));
                //} else {
                //    let successor = s.get_successor();
                //    let new_key = if let Self::Node{key: ref k, ..} = *successor.borrow() {
                //        *k
                //    } else {
                //        given
                //    };
                //    Self::delete(successor, new_key);
                //}
            println!("rec else if unwrap");
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
                (Self::Node{..}, mut r@Self::Node{..}) => {
                    let successor = r.get_successor();
                    println!("node unwrap");
                    let new_key = successor.borrow().get_key().unwrap();
                    self.set_key(new_key);
                    let mut r = r.clone();
                    r.delete(new_key);
                }
            }
        }
    }

    //TODO: successor is nil
    fn get_successor(&self) -> Rc<RefCell<Self>> {
        match self {
            Self::Nil => {
                Rc::new(RefCell::new(self.clone()))
            },
            Self::Node {parent, right, ..} => {
                match *right.borrow() {
                    Self::Node {ref key, ..} => {
                        Self::get_minimum(right.clone())
                    },
                    Self::Nil => {
                        let mut y = parent.clone();
                        let mut x = Rc::new(RefCell::new(self.clone()));
                        loop {
                            match (&*y.borrow() , &*x.borrow()) {
                                (Self::Node{left: ref yl, ..}, Self::Node{key: ref xk, ..}) => {
                                    if let Self::Node{key: ref ylk, ..} = *yl.borrow() {
                                        if *ylk == *xk {
                                            break;
                                        }
                                    }
                                },
                                (Self::Nil, _) => {
                                    break;
                                },
                                _ => {
                                    ()
                                }
                            }
                            x = y.clone();
                            let yp = if let Self::Node{parent: ref ypr, ..} = *y.borrow() {
                                ypr.clone()
                            } else {
                                Rc::new(RefCell::new(Self::Nil))
                            };
                            y = yp.clone();
                        }
                        y.clone()
                    }

                }
            }
        }
    }

    fn get_minimum(node: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        if let Self::Node {left, ..} = &*node.borrow() {
            match *left.borrow() {
                Self::Node{..} => {
                    Self::get_minimum(left.clone())
                },
                Self::Nil => {
                    node.clone()
                }
            }
        } else {
            Rc::new(RefCell::new(Self::Nil))
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
