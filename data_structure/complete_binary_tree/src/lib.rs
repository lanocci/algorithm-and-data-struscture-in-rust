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

    fn parent_idx(i: usize) -> usize {
        (i + 1) / 2 - 1
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
        let p_idx = Self::parent_idx(i);
        self.nodes.get(p_idx)
    }

    fn max_heapify(&mut self, idx: usize) {
        let mut lg_idx = match (self.get(idx), self.left(idx)) {
            (Some(s), Some(l)) => {
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

    pub fn insert(&mut self, key: T) {
        self.nodes.push(key);
        let mut i = self.nodes.len() - 1;
        if i < 1 { return; }
        while let (Some(p), Some(s)) = (self.parent(i), self.get(i)) {
            if i < 1 && s < p {
                break;
            }
            if p < s {
                self.nodes.swap(i, Self::parent_idx(i));
            }
            i = Self::parent_idx(i);
            if i < 1 { return; }
        }
    }

    pub fn extract(&mut self) -> T {
        let h = self.nodes.len();
        if h < 1 { panic!("tree is empty") }
        let max = self.nodes[0];
        let last = self.nodes.pop();
        if self.nodes.len() > 0 {
            self.nodes[0] = last.unwrap();
            self.max_heapify(0);
        }
        max
    }

}