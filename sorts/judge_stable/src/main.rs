use util::{Scanner, Joinable};

fn extract_num(s: &str) -> i32 {
    s[1..2].parse::<i32>().unwrap()
}

fn bubble_sort(a: &Vec<String>) -> Vec<String> {
    let mut result = a.clone();
    for i in 0..(result.len()) {
        for j in ((i + 1)..(result.len())).rev() {
            if extract_num(&result[j]) < extract_num(&result[j - 1]) {
                let right = &result[j].clone();
                let left = &result[j - 1].clone();
                result[j] = left.to_string();
                result[j - 1] = right.to_string();
            }
        }
    }
    result
}

fn selection_sort(a: &Vec<String>) -> Vec<String> {
    let mut result = a.clone();
    for i in 0..(result.len()) {
        let mut minj = i;
        for j in i..(result.len()) {
            if extract_num(&result[j]) < extract_num(&result[minj]) {
                minj = j;
            }
        }
        let min = &result[minj].clone();
        let index_position = &result[i].clone();
        result[minj] = index_position.to_string();
        result[i] = min.to_string();
    }
    result
}

fn is_stable(org: &Vec<String>, sorted: &Vec<String>) -> bool {
    for i in 0..(org.len() - 1) {
        for j in (i + 1)..(org.len() - 1) {
            for a in 0..(org.len() - 1) {
                for b in (a + 1)..(org.len() - 1) {
                    if extract_num(&org[i]) == extract_num(&org[j]) && org[i] == sorted[b] && org[j] == sorted[a] {
                        return false
                    }
                }
            }
        }
    }
    return true
}

/// sort out an array of cards in bubble sort and selection sort.
/// reports sorted array and if the sort was stable or not.
/// 
/// example input: 
/// ```
/// 5 # number of cards
/// H4 C9 S4 D2 C3 # array of cards represented with initial letter of the suits and numbers
/// ```
/// example output
/// ```
/// D2 C3 H4 S4 C9
/// Stable
/// D2 C3 S4 H4 C9
/// Not stable
/// ```
fn main() {
    let cin = std::io::stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n = sc.read();
    let vec: Vec<String> = sc.vec(n);
    let bubble_sorted = bubble_sort(&vec);
    println!("{}", bubble_sorted.iter().join(" "));
    if is_stable(&vec, &bubble_sorted) { println!("Stable"); }
    else { println!("Not stable"); }
    let selection_sorted = selection_sort(&vec);
    println!("{}", selection_sorted.iter().join(" "));
    if is_stable(&vec, &selection_sorted) { println!("Stable"); }
    else { println!("Not stable"); }
}
