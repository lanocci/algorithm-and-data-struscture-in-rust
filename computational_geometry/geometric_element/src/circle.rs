use std::fmt::Debug;
use num_traits::{Float, Zero, cast::FromPrimitive};

use crate::point::{Point, Vector};
use crate::segment::Line;

#[derive(Clone, Debug)]
pub struct Circle<T> where T: Float + FromPrimitive + Zero {
    center: Point<T>,
    radius: T,
}

impl<T> Circle<T> where T: Float + FromPrimitive + Zero + Debug {
    pub fn new(x: T, y: T, r: T) -> Self {
        Circle {
            center: Point::new(x, y),
            radius: r,
        }
    }

    pub fn polar(&self, radian: T) -> Vector<T> {
        Vector::new(radian.cos() * self.radius, radian.sin() * self.radius)
    }

    pub fn line_intersect(&self, line: &Line<T>) -> bool {
        line.distance_from_point(&self.center) < self.radius
    }

    /// ```
    /// use geometric_element::point::Point;
    /// use geometric_element::segment::Line;
    /// use geometric_element::circle::Circle;
    /// 
    /// let c = Circle::new(2.0, 1.0, 1.0);
    /// let l = Line::new(Point::new(0.0, 1.0), Point::new(4.0, 1.0));
    /// assert_eq!(c.line_cross_points(&l), Ok((Point{x: 1.0, y: 1.0}, Point{x: 3.0, y:1.0})));
    /// ```
    pub fn line_cross_points(&self, line: &Line<T>) -> Result<(Point<T>, Point<T>), String> {
        if self.line_intersect(&line) {
            let pr = line.projection(&self.center);
            let e = line.base_vector() / line.base_vector().abs();
            let base = (self.radius.clone().powi(2) + (pr.clone() - self.center.clone()).norm()).sqrt();
            Ok((pr.clone() - e.clone() * base.clone(), pr.clone() + e.clone() * base.clone()))
        } else {
            Err("line doesn't intersects the circle".to_string())
        }
    }

    pub fn intersect(&self, other: &Self) -> bool {
        self.center.distance(&other.center) <= self.radius + other.radius
    }

    /// ```
    /// use geometric_element::point::Point;
    /// use geometric_element::circle::Circle;
    /// let c1 = Circle::new(0.0, 0.0, 2.0);
    /// let c2 = Circle::new(2.0, 0.0, 2.0);
    /// if let Ok((point1, point2)) = c1.cross_points(&c2){
    ///     assert!(point1.x - 1.0 < 0.000001 && point1.x - 1.0 > -0.00000001);
    ///     assert!(point1.y - (-1.7320508) < 0.000001 && point1.y - (-1.7320508) > -0.000001);
    ///     assert!(point2.x - 1.0 < 0.000001 && point2.x - 1.0 > -0.00000001);
    ///     assert!(point2.y - 1.7320508 < 0.000001 && point2.y - 1.7320508 > -0.00000001);
    /// } else {
    ///     panic!();
    /// }
    /// ```
    pub fn cross_points(&self, other: &Self) -> Result<(Point<T>, Point<T>), String> {
        if self.intersect(&other) {
            let d = self.center.distance(&other.center);
            let a = ((self.radius.powi(2) + d.powi(2) - other.radius.powi(2)) / (T::from_i8(2).unwrap() * self.radius * d)).acos();
            println!("a: {:?}", a);
            let t = (other.center.clone() - self.center.clone()).declination();
            Ok((self.center.clone() + self.polar(t - a), self.center.clone() + self.polar(t + a)))
        } else {
            Err("no intersection".to_string())
        }
    }

}