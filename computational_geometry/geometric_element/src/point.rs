use std::ops;
use std::cmp::*;
use std::fmt::Debug;
use num_traits::{Float, Zero};

#[derive(Eq, Clone, Debug)]
pub struct Point<T> where T: Float {
    pub x: T,
    pub y: T,
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

impl<T> ops::Mul<T> for Point<T> where T: Float {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> ops::Div<T> for Point<T> where T: Float {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
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

    pub fn norm(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn abs(&self) -> T {
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

    pub fn is_parallel(&self, other: &Self) -> bool {
        self.cross_product(other).is_zero()
    }

    pub fn distance(&self, other: &Self) -> T {
        (self.clone() - other.clone()).abs()
    }

    /// returns gradient of the vector to x axis
    pub fn declination(&self) -> T {
        self.y.atan2(self.x)
    }
}

pub type Vector<T> = Point<T>;
