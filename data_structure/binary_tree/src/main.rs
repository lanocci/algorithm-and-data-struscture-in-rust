use std::io::*;
use util::Scanner;
use std::cmp::max;

#[derive(Clone)]
struct Node {
    pub parent: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

fn calc_depth(a: &Vec<Node>, depths: &mut Vec<usize>, idx: usize, depth: usize) {
    depths[idx] = depth;
    if let Some(left) = a[idx].left {
        calc_depth(a, depths, left, depth + 1);
    }
    if let Some(right) = a[idx].right {
        calc_depth(a, depths, right, depth + 1);
    }
}

fn calc_height(a: &Vec<Node>, heights: &mut Vec<usize>, idx: usize) -> usize {
    let mut left_height: Option<usize> = None;
    let mut right_height: Option<usize> = None;
    if let Some(left) = a[idx].left {
        left_height = Some(calc_height(a, heights, left));
    }
    if let Some(right) = a[idx].right {
        right_height = Some(calc_height(a, heights, right));
    }
    let height = match (left_height, right_height) {
        (Some(left), Some(right)) => max(left, right) + 1,
        (None, Some(right)) => right + 1,
        (Some(left), None) => left + 1,
        (None, None) => 0
    };
    heights[idx] = height;
    height
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

    let mut nodes = vec![Node{parent: None, left: None, right: None}; n];
    let mut depths: Vec<usize> = vec![0; n];
    let mut heights: Vec<usize> = vec![0; n];

    for _ in 0..n {
        let id: usize = sc.read();
        let left: i32 = sc.read();
        let right: i32 = sc.read();
        if left >= 0 {
            nodes[id].left = Some(left as usize);
            nodes[left as usize].parent = Some(id);
        }
        if right >= 0 {
            nodes[id].right = Some(right as usize);
            nodes[right as usize].parent = Some(id);
        }
    }
    let mut r: usize = 0;
    for i in 0..n {
        if nodes[i].parent.is_none() {
            r = i;
        }
    }

    //calculate depth
    calc_depth(&nodes, &mut depths, r, 0);

    //calculate height
    calc_height(&nodes, &mut heights, r);

    for i in 0..n {
        let parent = nodes[i].parent.map(|p| p as i32).unwrap_or(-1);
        let sibling = {
            if parent == -1 { 
                -1 
            }
            else if nodes[parent as usize].left == Some(i) { 
                nodes[parent as usize].right
                    .map(|r| r as i32)
                    .unwrap_or(-1) 
            } 
            else {
                nodes[parent as usize].left
                    .map(|l| l as i32)
                    .unwrap_or(-1)
            }
        };
        let degree = match (nodes[i].left, nodes[i].right) {
            (Some(_), Some(_)) => 2,
            (None, None) => 0,
            _ => 1,
        };
        let node_type = {
            if parent == -1 {
                "root"
            } else if degree == 0 {
                "leaf"
            } else {
                "internal node"
            }
        };
        println!("node {}: parent = {}, sibling = {}, degree = {}, depth = {}, height = {}, {}", i, parent, sibling, degree, depths[i], heights[i], node_type);
    }
}
