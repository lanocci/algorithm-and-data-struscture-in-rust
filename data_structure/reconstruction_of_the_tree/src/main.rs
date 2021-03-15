use std::io::*;
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

    let n = sc.read();
    let preorder: Vec<usize> = sc.vec(n);
    let inorder: Vec<usize> = sc.vec(n);

    let mut postorder: Vec<usize> = Vec::new();
    let mut position: usize = 0;
    reconstruction(&mut postorder, &preorder, &inorder, &mut position, 0, n);

    println!("{}", postorder.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

fn reconstruction(postorder: &mut Vec<usize>, preorder: &Vec<usize>, inorder: &Vec<usize>, pos: &mut usize, l: usize, r: usize) {
    if l >= r { return; }
    // root node content of the partial tree
    let root = preorder[*pos];
    // index of the root in the inorder traversal
    // nodes that has smaller index than m belongs to the left child tree
    // the other nodes belongs to the right child tree
    let m = inorder.iter().enumerate().find(|(_, x)| x == &&root).unwrap().0;
    *pos += 1;
    reconstruction(postorder, preorder, inorder, pos, l, m);
    reconstruction(postorder, preorder, inorder, pos, m + 1, r);
    postorder.push(root);
}
