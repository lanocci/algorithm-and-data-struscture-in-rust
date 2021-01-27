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
        let mut replace_target: Option<Rc<RefCell<BST<T>>>> = None;
        let mut should_delete_self = false;
        if let mut s@Self::Node {..} = self.clone() {
            if given == s.get_key().unwrap() {
                if *s.get_left().borrow() == Self::Nil && *s.get_right().borrow() == Self::Nil {
                    should_delete_self = true;
                } else if *s.get_left().borrow() == Self::Nil {
                    replace_target = Some(Rc::clone(&s.get_right()));
                } else if *s.get_right().borrow() == Self::Nil {
                    replace_target = Some(Rc::clone(&s.get_left()));
                } else {
                    let r = Rc::clone(&s.get_right());
                    let successor = s.get_successor();
                    let mut successor = successor.borrow_mut();
                    let new_key = if let Self::Node{key: ref k, ..} = *successor {
                        *k
                    } else {
                        given
                    };
                    successor.delete(new_key);
                    s.set_key(new_key);
                }
            } else if given < s.get_key().unwrap() {
                let l = s.get_left().clone();
                l.borrow_mut().delete(given);
            } else {
                let r = s.get_right().clone();
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
                Rc::new(RefCell::new(self.clone()))
            },
            Self::Node { ref parent, ref right, ..} => {
                match *right.borrow() {
                    Self::Node { right: ref r, .. } => {
                        let r = r.borrow();
                        r.get_minimum()
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

    fn get_minimum(&self) -> Rc<RefCell<Self>> {
        if let Self::Node {ref left, ..} = self {
            if let Self::Node {left: ref l, ..} = *left.borrow() {
                l.borrow().get_minimum()
            } else {
                left.clone()
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
