use util::{Joinable, Scanner};

fn selection_sort(a: &Vec<usize>) -> (Vec<usize>, usize) {
    let mut sorted = a.clone();
    let mut count = 0;
    for i in 0..sorted.len() {
        let mut minj = i;
        for j in i..sorted.len() {
            if sorted[j] < sorted[minj] {
                minj = j;
            }
        }
        let tmp = sorted[minj];
        sorted[minj] = sorted[i];
        sorted[i] = tmp;
        if i != minj {count += 1;}
    }
    (sorted, count)
}

/// sorts natural numbers with selection sort
/// inputs are given in standard input
/// example input: 
/// ```
/// 6 # length of the array
/// 5 6 4 2 1 3
/// ```
/// example output:
/// ```
/// 1 2 3 4 5 6
/// ```
fn main() {
    let cin = std::io::stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n = sc.read();
    let vec = sc.vec(n);
    let (sorted, swap_count) = selection_sort(&vec);
    println!("{}", sorted.iter().join(" "));
    println!("{}", swap_count);
}
