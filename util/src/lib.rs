use std::io::*;
use std::str::FromStr;

pub struct Scanner<R:Read> {
    reader: R,
    buffer: String,
}

impl<R: Read> Scanner<R> {
    pub fn new(reader: R) -> Scanner<R> {
        Scanner { reader: reader, buffer: String::new() }
    }

    pub fn line(&mut self) -> String {
        self.buffer = self.reader
            .by_ref()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|&c| c == '\n' || c == '\r')
            .take_while(|&c| c != '\n' || c != '\r')
            .collect::<String>();
        self.buffer.clone()
    }

    fn read_buffer<T: FromStr>(&mut self) {
        self.buffer = self.reader
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

    pub fn read<T: FromStr>(&mut self) -> T {
        if let Some(s) = self.safe_read() {
            s
        } else {
            writeln!(stderr(), "Terminated with EOF").unwrap();
            std::process::exit(0);
        }
    }


    pub fn vec<T: FromStr>(&mut self, len: usize) -> Vec<T> {
        (0..len).map(|_| self.read())
            .collect()
    }

    pub fn matrix<T: FromStr>(&mut self, row: usize, col: usize) -> Vec<Vec<T>> {
        (0..row).map(|_| self.vec(col))
            .collect()
    }
}

pub trait Joinable {
    fn join(self, sep: &str) -> String;
}

impl<U: ToString, T: Iterator<Item=U>> Joinable for T {
    fn join(self, sep: &str) -> String {
        self.map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
    }
}