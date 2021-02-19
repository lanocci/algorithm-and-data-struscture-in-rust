use std::ops;
use std::cmp::*;
use num_traits::{Float, Zero, cast::FromPrimitive};

#[derive(Eq, Clone)]
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
}

type Vector<T> = Point<T>;

pub struct Segment<T> where T: Float + Zero + FromPrimitive {
    p1: Point<T>,
    p2: Point<T>,
}

impl<T> Segment<T> where T: Float + Zero + FromPrimitive {
    pub fn new(p1: Point<T>, p2: Point<T>) -> Self {
        Segment {p1, p2}
    }

    pub fn base_vector(&self) -> Vector<T> {
        self.p2.clone() - self.p1.clone()
    }

    pub fn is_orthogonal(&self, other: &Self) -> bool {
        let vec1 = self.base_vector();
        let vec2 = other.base_vector();
        vec1.is_orthogonal(&vec2)
    }

    pub fn is_parallel(&self, other: &Self) -> bool {
        let vec1 = self.base_vector();
        let vec2 = other.base_vector();
        vec1.is_parallel(&vec2)
    }
    
    /// calculate projection point (get projection point in form of vector and add original point back)
    pub fn projection(&self, point: &Point<T>) -> Point<T> {
        // convert self into vector (set p1 to the Origin)
        let base = self.base_vector();
        // vector of given point from p1
        let hypo = point.clone() - self.p1.clone();
        // ratio of t to base, where t is the vector from the Origin to projetion point
        let r = hypo.dot_product(&base) / base.norm();
        self.p1.clone() + base * r
    }

    /// calculate reflection point (get projection point and double the vector from p to projection point)
    pub fn reflection(&self, point: &Point<T>) -> Point<T> {
        point.clone() + (self.projection(point) - point.clone()) * T::from_usize(2).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_test() {
        let parallel_segment1 = Segment::new(Point::new(0.0, 0.0), Point::new(3.0, 0.0));
        let parallel_segment2 = Segment::new(Point::new(0.0, 2.0), Point::new(3.0, 2.0));

        let orthogonal_segment1 = Segment::new(Point::new(0.0, 1.0), Point::new(4.0, 0.0));
        let orthogonal_segment2 = Segment::new(Point::new(0.0, 1.0), Point::new(-1.0, -3.0));

        assert!(orthogonal_segment1.is_orthogonal(&orthogonal_segment2));
        assert!(!parallel_segment1.is_orthogonal(&parallel_segment2));

        assert!(parallel_segment1.is_parallel(&parallel_segment2));
        assert!(!orthogonal_segment1.is_parallel(&orthogonal_segment2));
    }
}
