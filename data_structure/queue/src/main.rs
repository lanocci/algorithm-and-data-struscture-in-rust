use std::io::*;
use std::str::FromStr;
use std::collections::*;
use std::cmp::min;

fn main() {
    std::thread::Builder::new()
        .stack_size(104857600)
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
    let q: usize = sc.read();
    let mut names = Vec::with_capacity(n);
    let mut durations = Vec::with_capacity(n);
    let mut queue = VecDeque::with_capacity(n);
    for i in 0..n {
        let s:String = sc.read();
        names.push(s);
        let t = sc.read();
        durations.push(t);
        queue.push_back(i);
    }

    let mut total_duration = 0;
    while let Some(p) = queue.pop_front() {
        let t = min(durations[p], q);
        durations[p] -= t;
        total_duration += t;
        if durations[p] > 0 {
            queue.push_back(p)
        } else {
            println!("{} {}", names[p], total_duration)
        }
    }
}

struct Scanner<R:Read> {
    reader: R,
    buffer: String,
}

impl<R: Read> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner { reader: reader, buffer: String::new() }
    }

    fn read_buffer<T: FromStr>(&mut self) {
        self.buffer = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
    }

    fn safe_read<T: FromStr>(&mut self) -> Option<T> {
        self.read_buffer::<T>();
        if self.buffer.is_empty() {
            None
        } else {
            self.buffer.parse::<T>().ok()
        }
    }

    fn read<T: FromStr>(&mut self) -> T {
        if let Some(s) = self.safe_read() {
            s
        } else {
            writeln!(stderr(), "Terminated with EOF").unwrap();
            std::process::exit(0);
        }
    }

    //fn vec<T: FromStr>(&mut self, len: usize) -> Vec<T> {
    //    (0..len).map(|_| self.read())
    //        .collect()
    //}

    //fn matrix<T: FromStr>(&mut self, row: usize, col: usize) -> Vec<Vec<T>> {
    //    (0..row).map(|_| self.vec(col))
    //        .collect()
    //}
}

//trait Joinable {
//    fn join(self, sep: &str) -> String;
//}
//
//impl<U: ToString, T: Iterator<Item=U>> Joinable for T {
//    fn join(self, sep: &str) -> String {
//        self.map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
//    }
//}