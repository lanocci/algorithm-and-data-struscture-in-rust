use std::ops;
use std::cmp::*;
use num_traits::Float;

#[derive(Eq)]
struct Point<T> where T: Float {
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

impl<T> Point<T> where T: Float {
    fn norm(self) -> T {
        self.x * self.x + self.y * self.y
    }

    fn abs(self) -> T {
        self.norm().sqrt()
    }

    fn dot_product(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    fn cross_product(&self, other: &Self) -> T {
        self.x * other.y + self.y * other.x
    }
}

type Vector<T> = Point<T>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
