use common::{trace, UserStringInput, handle_string_input};

fn extract_num(s: &str) -> i32 {
    s[1..2].parse::<i32>().unwrap()
}

fn bubble_sort(a: &Vec<String>) -> Vec<String> {
    let mut cloned = a.clone();
    for i in 0..(cloned.len()) {
        for j in ((i + 1)..(cloned.len())).rev() {
            if extract_num(&cloned[j]) < extract_num(&cloned[j - 1]) {
                let right = &cloned[j].clone();
                let left = &cloned[j - 1].clone();
                cloned[j] = left.to_string();
                cloned[j - 1] = right.to_string();
            }
        }
        trace(&cloned);
    }
    cloned
}

fn selection_sort(a: &Vec<String>) -> Vec<String> {
    let mut cloned = a.clone();
    for i in 0..(cloned.len()) {
        let mut minj = i;
        for j in i..(cloned.len()) {
            if extract_num(&cloned[j]) < extract_num(&cloned[minj]) {
                minj = j;
            }
        }
        let min = &cloned[minj].clone();
        let index_position = &cloned[i].clone();
        cloned[minj] = index_position.to_string();
        cloned[i] = min.to_string();
        trace(&cloned)
    }
    cloned
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

fn main() {
    let input = handle_string_input();
    let bubble_sorted = bubble_sort(& input.vec);
    println!("bubble sort is stable: {}", is_stable(&input.vec, &bubble_sorted));
    let selection_sorted = selection_sort(& input.vec);
    println!("selection sort is stable: {}", is_stable(&input.vec, &selection_sorted));
}
