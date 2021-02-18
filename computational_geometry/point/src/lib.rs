use std::ops;
use std::cmp::*;
use num_traits::{Float, Zero};

#[derive(Eq, Clone)]
pub struct Point<T> where T: Float {
    x: T,
    y: T,
}

impl<T> ops::Add for Point<T> where T: Float  {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x, 
            y: self.y + rhs.y
        }
    }
}

impl<T> ops::Sub for Point<T> where T: Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> ops::Mul for Point<T> where T: Float {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> ops::Div for Point<T> where T: Float {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T> PartialEq for Point<T> where T: Float {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> PartialOrd for Point<T> where T: Float + Eq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Point<T> where T: Float + Eq {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x == other.x {
            if self.y == other.y {
                Ordering::Equal
            } else if self.y < other.y {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else if self.x < other.x{
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl<T> Point<T> where T: Float + Zero {
    pub fn new(x: T, y: T) -> Self {
        Point {x, y}
    }

    pub fn norm(self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn abs(self) -> T {
        self.norm().sqrt()
    }

    pub fn dot_product(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn cross_product(&self, other: &Self) -> T {
        self.x * other.y + self.y * other.x
    }

    pub fn is_orthogonal(&self, other: &Self) -> bool {
        self.dot_product(other).is_zero()
    }
}

type Vector<T> = Point<T>;

pub struct Segment<T> where T: Float + Zero {
    p1: Point<T>,
    p2: Point<T>,
}

impl<T> Segment<T> where T: Float + Zero {
    pub fn new(p1: Point<T>, p2: Point<T>) -> Self {
        Segment {p1, p2}
    }

    pub fn is_orthogonal(&self, other: &Self) -> bool {
        let vec1 = self.p2.clone() - self.p1.clone();
        let vec2 = other.p2.clone() - other.p1.clone();
        vec1.is_orthogonal(&vec2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
