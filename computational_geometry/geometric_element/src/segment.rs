use std::cmp::*;
use std::fmt::Debug;
use num_traits::{Float, Zero, cast::FromPrimitive};

use crate::point::{Point, Vector};

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

    pub fn distance_from_point(&self, point: &Point<T>) -> T {
        if (self.p2.clone() - self.p1.clone()).dot_product(&(point.clone() - self.p1.clone())) < T::from_f32(0.0).unwrap() {
            return (point.clone() - self.p1.clone()).abs()
        }
        if (self.p1.clone() - self.p2.clone()).dot_product(&(point.clone() - self.p2.clone())) < T::from_f32(0.0).unwrap() {
            return (point.clone() - self.p2.clone()).abs()
        }
        (self.p2.clone() - self.p1.clone()).cross_product(&(point.clone() - self.p1.clone())) / (self.p2.clone() - self.p1.clone()).abs()
    }

    /// calculate distance from a segment to another
    /// ```
    /// use geometric_element::point::Point;
    /// use geometric_element::segment::Segment;
    /// let segment1 = Segment::new(Point::new(1.0, 0.0), Point::new(1.0, 0.0));
    /// let segment2 = Segment::new(Point::new(0.0, 1.0), Point::new(1.0, 1.0));
    /// assert_eq!(segment1.distance(&segment2), 1.0);
    /// ```
    pub fn distance(&self, other: &Self) -> T {
        if self.intersection(other) { T::from_f32(0.0).unwrap() }
        else {
            let min_from_s1 = self.distance_from_point(&other.p1).min(self.distance_from_point(&other.p2));
            let min_from_s2 = other.distance_from_point(&self.p1).min(other.distance_from_point(&self.p2));
            min_from_s1.min(min_from_s2).abs()
        }
    }

    /// ```
    /// use geometric_element::point::Point;
    /// use geometric_element::segment::{Segment, PointLocation};
    /// let seg = Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 0.0));
    /// let p1 = Point::new(-1.0, 1.0);
    /// let p2 = Point::new(-1.0, -1.0);
    /// let p3 = Point::new(-1.0, 0.0);
    /// let p4 = Point::new(0.0, 0.0);
    /// let p5 = Point::new(3.0, 0.0);
    /// assert_eq!(seg.clockwise(&p1), PointLocation::CounterClockwise);
    /// assert_eq!(seg.clockwise(&p2), PointLocation::Clockwise);
    /// assert_eq!(seg.clockwise(&p3), PointLocation::OnlineBack);
    /// assert_eq!(seg.clockwise(&p4), PointLocation::OnSegment);
    /// assert_eq!(seg.clockwise(&p5), PointLocation::OnlineFront);
    /// ```
    pub fn clockwise(&self, point: &Point<T>) -> PointLocation {
        let base = self.base_vector();
        let hypo = point.clone() - self.p1.clone();
        if base.cross_product(&hypo) > T::from_f32(0.0).unwrap() {
            PointLocation::CounterClockwise
        } else if base.cross_product(&hypo) < T::from_f32(0.0).unwrap() {
            PointLocation::Clockwise
        } else {
            if base.dot_product(&hypo) < T::from_f32(0.0).unwrap() {
                PointLocation::OnlineBack
            } else if hypo.abs() > base.abs() {
                PointLocation::OnlineFront
            } else {
                PointLocation::OnSegment
            }
        }
    }

    /// ```
    /// use geometric_element::segment::Segment;
    /// use geometric_element::point::Point;
    /// 
    /// let segment1 = Segment::new(Point::new(0.0, 0.0), Point::new(3.0, 0.0));
    /// let segment2 = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, -1.0));
    /// assert!(segment1.intersection(&segment2));
    /// let segment2 = Segment::new(Point::new(3.0, 1.0), Point::new(3.0, -1.0));
    /// assert!(segment1.intersection(&segment2));
    /// let segment2 = Segment::new(Point::new(3.0, -2.0), Point::new(5.0, 0.0));
    /// assert!(!segment1.intersection(&segment2));
    /// ```
    pub fn intersection(&self, other: &Self) -> bool {
        let s1p3 = self.clockwise(&other.p1);
        let s1p4 = self.clockwise(&other.p2);
        let s2p1 = other.clockwise(&self.p1);
        let s2p2 = other.clockwise(&self.p2);
        s1p3.intersection(&s1p4) && s2p1.intersection(&s2p2)
    }

    /// ```
    /// use geometric_element::point::Point;
    /// use geometric_element::segment::Segment;
    /// 
    /// let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 0.0));
    /// let s2 = Segment::new(Point::new(1.0, 1.0), Point::new(1.0, -1.0));
    /// assert_eq!(s1.cross_point(&s2), Point{x: 1.0,y:0.0 });
    /// 
    /// let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
    /// let s2 = Segment::new(Point::new(0.0, 1.0), Point::new(1.0, 0.0));
    /// assert_eq!(s1.cross_point(&s2), Point{x: 0.5,y: 0.5 });
    /// ```
    pub fn cross_point(&self, other: &Self) -> Point<T> {
        let hypo1 = other.p2.clone() - self.p1.clone();
        let hypo2 = self.p2.clone() - other.p1.clone();
        let base = other.base_vector();
        //let d1 = hypo1.cross_product(&base).abs()/base.clone().abs();
        //let d2 = hypo2.cross_product(&base).abs()/base.clone().abs();
        // t を計算するときに約分で自動的に消えるので, baseでの除算は省略しても結果が変わらない
        let d1 = hypo1.cross_product(&base).abs();
        let d2 = hypo2.cross_product(&base).abs();
        let t = d1 / (d1 + d2);
        self.p1.clone() + (self.p2.clone() - self.p1.clone())*t
    }

}

pub type Line<T> = Segment<T>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PointLocation {
    Clockwise,
    CounterClockwise,
    OnlineBack,
    OnlineFront,
    OnSegment,
}

impl PointLocation {
    pub fn intersection(&self, other: &Self) -> bool {
        match self {
            Self::Clockwise => {
                other.clone() == Self::CounterClockwise || other.clone() == Self::OnSegment
            },
            Self::CounterClockwise => {
                other.clone() == Self::Clockwise || other.clone() == Self::OnSegment
            },
            Self::OnSegment => {
                other.clone() == Self::Clockwise || other.clone() == Self::OnSegment || other.clone() == Self::CounterClockwise
            },
            Self::OnlineBack => {
                false
            },
            Self::OnlineFront => {
                false
            }
        }
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