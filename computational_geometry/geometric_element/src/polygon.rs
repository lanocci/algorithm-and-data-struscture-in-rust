use std::fmt::Debug;
use std::collections::VecDeque;
use std::mem;
use std::cmp::Ordering;
use num_traits::{Float, Zero, FromPrimitive};

use crate::point::Point;
use crate::segment;

#[derive(Clone, Debug)]
pub struct Polygon<T>(Vec<Point<T>>) where T: Float + Zero + FromPrimitive;

impl<T> Polygon<T> where T: Float + Zero + FromPrimitive + Debug {
    pub fn new(points: Vec<Point<T>>) -> Self {
        Polygon(points)
    }

    pub fn len(&self) -> usize {
        self.0.len()
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
            if a.cross_product(&b).is_sign_positive() && a.y.is_sign_negative() && b.y.is_sign_positive() {
                is_in = !is_in;
            }
        }
        if is_in {
            PointLocation::In
        } else {
            PointLocation::Out
        }
    }

    /// convex hull implementation with Andrew's algorithm
    /// ```
    /// use geometric_element::polygon::Polygon;
    /// use geometric_element::point::Point;
    /// let polygon = Polygon::new(
    ///     vec![Point::new(2., 1.), Point::new(0., 0.), Point::new(1., 2.), Point::new(2., 2.), Point::new(4., 2.), Point::new(1., 3.), Point::new(3., 3.)]
    /// );
    /// assert_eq!(polygon.convex_hull().len(), 4);
    /// ```
    pub fn convex_hull(&self) -> Self {
        if self.0.len() < 3 { panic!() }
        let mut s = self.0.clone();
        s.sort_by(|a, b| {
            if a.x < b.x { Ordering::Less } 
            else if a.x > b.x { Ordering::Greater } 
            else if a.y < b.y { Ordering::Less } 
            else if a.y > a.y { Ordering::Greater } 
            else { Ordering::Equal }
        });

        for p in Vec::from(s.clone()).iter() {
            println!("S: ({:?}, {:?})", p.x, p.y);
        }

        let mut u = VecDeque::new();
        u.push_back(s[0].clone());
        u.push_back(s[1].clone());
        for p in s.iter().skip(2) {
            for i in (2..(u.len() + 1)).rev() {
                let seg = segment::Segment::new(u[i-2].clone(), u[i-1].clone());
                if seg.clockwise(p) != segment::PointLocation::Clockwise {
                    u.pop_back();
                } else {
                    break;
                }
            }
            u.push_back(p.clone());
        }

        s.reverse();
        let mut l = VecDeque::new();
        l.push_back(s[0].clone());
        l.push_back(s[1].clone());
        for p in s.iter().skip(2) {
            for i in (2..(l.len()+1)).rev() {
                let seg = segment::Segment::new(l[i-2].clone(), l[i-1].clone());
                if seg.clockwise(p) != segment::PointLocation::Clockwise {
                    l.pop_back();
                } else {
                    break;
                }
            }
            l.push_back(p.clone());
        }

        let mut result = Vec::from(l);
        result.reverse();
        for p in u.iter().skip(1).rev().skip(1) {
            result.push(p.clone());
        }
        Polygon::new(result)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum PointLocation {
    In,
    On,
    Out,
}