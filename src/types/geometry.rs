/*
** src/types/geometry.rs
*/

use crate::types::{Matrix2D, Vector2};

use std::cmp;
use std::fmt;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn reflect_x(&self, x: i64) -> Self {
        let dx = self.x - x;
        Self::new(x - dx, self.y)
    }

    pub fn reflect_y(&self, y: i64) -> Self {
        let dy = self.y - y;
        Self::new(self.x, y - dy)
    }

    // are the 3 points listed in counter-clockwise order?
    pub fn ccw(a: &Point, b: &Point, c: &Point) -> bool {
        // if the slope of the line AB is less than the slope of the line AC
        (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
    }

    pub fn sort_by_x<'a>(pa: &'a Self, pb: &'a Self) -> (&'a Self, &'a Self) {
        if cmp::min(pa.x, pb.x) == pa.x {
            (pa, pb)
        } else {
            (pb, pa)
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        // format: x,y
        let split = s.find(',').unwrap();
        let x = s[0..split].parse().unwrap();
        let y = s[(split + 1)..s.len()].parse().unwrap();
        Self { x, y }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{},{}", self.x, self.y))
    }
}

#[derive(Clone)]
pub struct Line {
    pub p0: Point,
    pub p1: Point,
    pub slope: Option<i64>,
    pub y_intercept: Option<i64>,
}

impl Line {
    pub fn new(p0: Point, p1: Point) -> Self {
        let slope = Self::get_slope(&p0, &p1);
        let y_intercept = Self::get_y_intercept(&p0, &p1);
        Self {
            p0,
            p1,
            slope,
            y_intercept,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.p0.y == self.p1.y
    }

    pub fn is_vertical(&self) -> bool {
        self.p0.x == self.p1.x
    }

    pub fn x_min(&self) -> i64 {
        cmp::min(self.p0.x, self.p1.x)
    }

    pub fn x_max(&self) -> i64 {
        cmp::max(self.p0.x, self.p1.x)
    }

    pub fn y_min(&self) -> i64 {
        cmp::min(self.p0.y, self.p1.y)
    }

    pub fn y_max(&self) -> i64 {
        cmp::max(self.p0.y, self.p1.y)
    }

    fn get_slope(p0: &Point, p1: &Point) -> Option<i64> {
        if p0.x == p1.x {
            None
        } else {
            let (lp, rp) = Point::sort_by_x(p0, p1);
            Some((rp.y - lp.y) / (rp.x - lp.x))
        }
    }

    fn get_y_intercept(p0: &Point, p1: &Point) -> Option<i64> {
        let slope = Self::get_slope(p0, p1);
        if p0.x == p1.x {
            None
        } else {
            // solve using p0
            Some(p0.y - (p0.x * slope.unwrap()))
        }
    }

    pub fn contains_point(&self, p: &Point) -> bool {
        if self.is_vertical() {
            p.x == self.p0.x && (self.y_min()..=self.y_max()).contains(&p.y)
        } else {
            p.y == (self.slope.unwrap() * p.x) + self.y_intercept.unwrap()
                && (self.x_min()..=self.x_max()).contains(&p.x)
                && (self.y_min()..=self.y_max()).contains(&p.y)
        }
    }

    pub fn sort_by_x<'a>(line_a: &'a Self, line_b: &'a Self) -> (&'a Self, &'a Self) {
        if cmp::min(line_a.x_min(), line_b.x_min()) == line_a.x_min() {
            (line_a, line_b)
        } else {
            (line_b, line_a)
        }
    }

    pub fn sort_by_y<'a>(line_a: &'a Self, line_b: &'a Self) -> (&'a Self, &'a Self) {
        if cmp::min(line_a.y_min(), line_b.y_min()) == line_a.y_min() {
            (line_a, line_b)
        } else {
            (line_b, line_a)
        }
    }

    #[allow(clippy::suspicious_operation_groupings)]
    pub fn verticals_intersect(line_a: &Self, line_b: &Self) -> bool {
        let (bot, top) = Line::sort_by_y(line_a, line_b);
        bot.p0.x == top.p0.x && top.y_min() <= bot.y_max()
    }

    pub fn has_intersection(line_a: &Self, line_b: &Self) -> bool {
        // note: the below does not cover scenarios when an endpoint is the intersection
        line_a.contains_point(&line_b.p0) || line_a.contains_point(&line_b.p1)
            || line_b.contains_point(&line_a.p0) || line_b.contains_point(&line_a.p1)
        // see https://bryceboe.com/2006/10/23/line-segment-intersection-algorithm/
        // lines A and B intersect if and only if points A0 and A1 are separated by segment B0-B1
        // and points B0 and B1 are separated by segment A0-A1 then: if A0 and A1 are separated by
        // segment B0-B1 then A0-B0-B1 and A1-B0-B1 should have opposite orientation; i.e. either
        // A0-B0-B1 or A1-B0-B1 is counter-clockwise but NOT both
            || Point::ccw(&line_a.p0, &line_b.p0, &line_b.p1)
            != Point::ccw(&line_a.p1, &line_b.p0, &line_b.p1)
            && Point::ccw(&line_a.p0, &line_a.p1, &line_b.p0)
                != Point::ccw(&line_a.p0, &line_a.p1, &line_b.p1)
    }

    pub fn intersection(line_a: &Self, line_b: &Self) -> Option<Point> {
        if Self::has_intersection(line_a, line_b) {
            // solve the system of equations
            // NOTE: start with numbers as floating point
            let ma = line_a.slope.unwrap() as f64;
            let mb = line_b.slope.unwrap() as f64;
            let mat = Matrix2D::new(ma, -1.0, mb, -1.0);
            let vec = Vector2::new(
                -line_a.y_intercept.unwrap() as f64,
                -line_b.y_intercept.unwrap() as f64,
            );
            let sol = Matrix2D::solve_system(&mat, &vec);
            let x = sol.data[0];
            let y = sol.data[1];
            // ensure that the intersection is a whole number
            if x.fract() == 0.0 && y.fract() == 0.0 {
                Some(Point::new(x as i64, y as i64))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        // format: x0,y0 -> x1,y1
        match split!(s, " -> ") {
            [sp0, sp1] => {
                let p0 = Point::from(*sp0);
                let p1 = Point::from(*sp1);
                Self::new(p0, p1)
            }
            _ => panic!("invalid line: {}", s),
        }
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}->{:?}", self.p0, self.p1))
    }
}
