pub struct CompleteBinaryTree<T> where T: Copy {
    nodes: Vec<T>,
}

impl<T> CompleteBinaryTree<T> where T: Copy{

    pub fn new(nodes: Vec<T>) -> CompleteBinaryTree<T> {
        CompleteBinaryTree { nodes: nodes }
    }

    pub fn get<'a>(&'a self, i: usize) -> Option<&'a T> {
        self.nodes.get(i)
    }

    pub fn left<'a>(&'a self, i: usize) -> Option<&'a T> {
        let l_idx = (i + 1) * 2 - 1;
        self.nodes.get(l_idx)
    }

    pub fn right<'a>(&'a self, i: usize) -> Option<&'a T> {
        let r_idx = (i + 1) * 2;
        self.nodes.get(r_idx)
    }

    pub fn parent<'a>(&'a self, i: usize) -> Option<&'a T> {
        let p_idx = (i + 1) / 2 - 1;
        self.nodes.get(p_idx)
    }

    pub fn max_heapify(&mut self) -> {

    }

    fn swap(&mut self, a: usize, b: usize) {
        self.nodes.swap(a, b);
    }

}