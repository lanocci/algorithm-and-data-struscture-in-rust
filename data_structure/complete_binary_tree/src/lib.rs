use std::fmt::Display;

pub struct CompleteBinaryTree<T> where T: Copy + Ord + Display{
    pub nodes: Vec<T>,
}

impl<T> CompleteBinaryTree<T> where T: Copy + Ord + Display {

    pub fn new(nodes: Vec<T>) -> CompleteBinaryTree<T> {
        CompleteBinaryTree { nodes: nodes }
    }

    fn left_idx(i: usize) -> usize {
        (i + 1) * 2 - 1
    }

    fn right_idx(i: usize) -> usize {
        (i + 1) * 2
    }

    pub fn get<'a>(&'a self, i: usize) -> Option<&'a T> {
        self.nodes.get(i)
    }

    pub fn left<'a>(&'a self, i: usize) -> Option<&'a T> {
        let l_idx = Self::left_idx(i);
        self.nodes.get(l_idx)
    }

    pub fn right<'a>(&'a self, i: usize) -> Option<&'a T> {
        let r_idx = Self::right_idx(i);
        self.nodes.get(r_idx)
    }

    pub fn parent<'a>(&'a self, i: usize) -> Option<&'a T> {
        let p_idx = (i + 1) / 2 - 1;
        self.nodes.get(p_idx)
    }

    fn max_heapify(&mut self, idx: usize) {
        let mut lg_idx = match (self.get(idx), self.left(idx)) {
            (Some(s), Some(l)) => {
                println!("comparing self and left");
                println!("self: {}, left: {}", s, l);
                if l > s {
                    Self::left_idx(idx)
                } else {
                    idx
                }
            }
            (Some(_), None) => {
                idx
            }
            _ => {
                unreachable!()
            }
        };
        lg_idx = match (self.get(lg_idx), self.right(idx)) {
            (Some(lg), Some(r)) => {
                println!("comparing to right");
                println!("lg: {}, right: {}", lg, r);
                if r > lg {
                    Self::right_idx(idx)
                } else {
                    lg_idx
                }
            },
            (Some(_), None) => {
                lg_idx
            }
            _ => unreachable!()
        };
        if lg_idx != idx {
            println!("swapped!");
            self.nodes.swap(lg_idx, idx);
            self.max_heapify(lg_idx);
        }
    }

    pub fn build_max_heap(&mut self) {
        let h = self.nodes.len() / 2;
        for i in (0..h).rev() {
            self.max_heapify(i);
        }
    }

}