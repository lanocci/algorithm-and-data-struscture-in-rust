use std::fmt::Debug;
use std::mem;
use num_traits::{Float, Zero};

use crate::point::Point;

#[derive(Clone, Debug)]
pub struct Polygon<T>(Vec<Point<T>>) where T: Float + Zero;

impl<T> Polygon<T> where T: Float + Zero + Debug {
    pub fn new(points: Vec<Point<T>>) -> Self {
        Polygon(points)
    }

    /// ```
    /// use geometric_element::polygon::{Polygon, PointLocation};
    /// use geometric_element::point::Point;
    /// 
    /// let polygon = Polygon::new(
    ///   vec![Point::new(0.0, 0.0), Point::new(3.0, 1.0), Point::new(2.0, 3.0), Point::new(0.0, 3.0)]
    /// );
    /// let point = Point::new(0.0, 2.0);
    /// assert_eq!(PointLocation::On, polygon.contains(&point));
    /// let point = Point::new(3.0, 2.0);
    /// assert_eq!(PointLocation::Out, polygon.contains(&point));
    /// let point = Point::new(2.0, 1.0);
    /// assert_eq!(PointLocation::In, polygon.contains(&point));
    /// ```
    pub fn contains(&self, point: &Point<T>) -> PointLocation {
        let mut is_in = false;
        let n = self.0.len();
        for i in 0..n {
            let g = self.0[i].clone();
            let h = self.0[(i + 1) % n].clone();
            let mut a = g - point.clone();
            let mut b = h - point.clone();
            if a.cross_product(&b).is_zero() && a.dot_product(&b).is_sign_negative() {
                return PointLocation::On
            }
            if a.y > b.y {
                mem::swap(&mut a, &mut b);
            }
            if a.cross_product(&b).is_sign_positive() && (a.y.is_sign_negative() || a.y.is_zero()) && (b.y.is_sign_positive() || b.y.is_zero()) {
                is_in = !is_in;
            }
        }
        if is_in {
            PointLocation::In
        } else {
            PointLocation::Out
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum PointLocation {
    In,
    On,
    Out,
}