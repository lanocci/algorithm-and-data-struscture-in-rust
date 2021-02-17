use std::ops;
use num_traits::Float;

struct Point<T> where T: Float {
    x: T,
    y: T,
}

impl<T> ops::Add for Point<T> where T: Float  {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x, 
            y: self.y + rhs.y
        }
    }
}

impl<T> Point<T> where T: Float {
    fn norm(self) -> T {
        self.x * self.x + self.y * self.y
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
