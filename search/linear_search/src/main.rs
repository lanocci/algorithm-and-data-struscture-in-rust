use util::Scanner;

fn main() {
    std::thread::Builder::new()
        .stack_size(1048576)
        .spawn(solve_2)
        .unwrap()
        .join()
        .unwrap();
}

fn solve() {
    let cin = std::io::stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let n = sc.read();
    let s: Vec<i32> = sc.vec(n);
    let q = sc.read();
    let t: Vec<i32> = sc.vec(q);

    let mut cnt = 0;

    for a in s.iter() {
        for b in t.iter() {
            if a == b {cnt += 1};
        }
    }
    println!("{}", cnt);
}

fn solve_2() {
    let cin = std::io::stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let n = sc.read();
    let mut s: Vec<i32> = sc.vec(n);
    let q = sc.read();
    let mut t: Vec<i32> = sc.vec(q);

    s.sort();
    s.dedup();
    t.sort();
    
    let mut cnt = 0;
    let mut nit = s.iter().peekable();

    for &x in t.iter() {
        while let Some(&y) = nit.peek() {
            if y < &x {
                nit.next();
            }
            else {
                break;
            }
        }
        while let Some(&y) = nit.peek() {
            if y == &x {
                cnt += 1;
                nit.next();
            }
            else {
                break;
            }
        }
    }
    println!("{}", cnt);
}
