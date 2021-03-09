use util::{Scanner, Joinable};

fn bubble_sort(a: &Vec<usize>) -> Vec<usize> {
    let mut result = a.clone();
    let mut flag: bool = true;
    let n: usize = a.len();
    while flag {
        flag = false;
        for j in (1..n).rev() {
            if result[j - 1] > result[j] {
                let tmp = result[j];
                result[j] = result[j - 1];
                result[j - 1] = tmp;
                flag = true;
            }
        }
    }
    result
}

fn main() {
    let cin = std::io::stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n = sc.read();
    let input: Vec<usize> = sc.vec(n);
    let result = bubble_sort(&input);
    println!("{}", result.iter().join(" "));
}
