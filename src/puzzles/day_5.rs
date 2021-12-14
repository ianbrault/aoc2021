/*
** src/puzzles/day_5.rs
** https://adventofcode.com/2021/day/5
*/

use crate::types::{Matrix2D, Puzzle, Result, Solution, Vector2};
use crate::utils;

use std::cmp;
use std::collections::HashSet;
use std::fmt;

const INPUT: &str = include_str!("../../input/5.txt");

#[derive(Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    // are the 3 points listed in counter-clockwise order?
    fn ccw(a: &Point, b: &Point, c: &Point) -> bool {
        // if the slope of the line AB is less than the slope of the line AC
        (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
    }

    fn sort_by_x<'a>(pa: &'a Self, pb: &'a Self) -> (&'a Self, &'a Self) {
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
struct Line {
    p0: Point,
    p1: Point,
    slope: Option<i64>,
    y_intercept: Option<i64>,
}

impl Line {
    fn new(p0: Point, p1: Point) -> Self {
        let slope = Self::get_slope(&p0, &p1);
        let y_intercept = Self::get_y_intercept(&p0, &p1);
        Self {
            p0,
            p1,
            slope,
            y_intercept,
        }
    }

    fn is_horizontal(&self) -> bool {
        self.p0.y == self.p1.y
    }

    fn is_vertical(&self) -> bool {
        self.p0.x == self.p1.x
    }

    fn x_min(&self) -> i64 {
        cmp::min(self.p0.x, self.p1.x)
    }

    fn x_max(&self) -> i64 {
        cmp::max(self.p0.x, self.p1.x)
    }

    fn y_min(&self) -> i64 {
        cmp::min(self.p0.y, self.p1.y)
    }

    fn y_max(&self) -> i64 {
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

    fn contains_point(&self, p: &Point) -> bool {
        if self.is_vertical() {
            p.x == self.p0.x && (self.y_min()..=self.y_max()).contains(&p.y)
        } else {
            p.y == (self.slope.unwrap() * p.x) + self.y_intercept.unwrap()
                && (self.x_min()..=self.x_max()).contains(&p.x)
                && (self.y_min()..=self.y_max()).contains(&p.y)
        }
    }

    fn sort_by_x<'a>(line_a: &'a Self, line_b: &'a Self) -> (&'a Self, &'a Self) {
        if cmp::min(line_a.x_min(), line_b.x_min()) == line_a.x_min() {
            (line_a, line_b)
        } else {
            (line_b, line_a)
        }
    }

    fn sort_by_y<'a>(line_a: &'a Self, line_b: &'a Self) -> (&'a Self, &'a Self) {
        if cmp::min(line_a.y_min(), line_b.y_min()) == line_a.y_min() {
            (line_a, line_b)
        } else {
            (line_b, line_a)
        }
    }

    #[allow(clippy::suspicious_operation_groupings)]
    fn verticals_intersect(line_a: &Self, line_b: &Self) -> bool {
        let (bot, top) = Line::sort_by_y(line_a, line_b);
        bot.p0.x == top.p0.x && top.y_min() <= bot.y_max()
    }

    fn has_intersection(line_a: &Self, line_b: &Self) -> bool {
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

    fn intersection(line_a: &Self, line_b: &Self) -> Option<Point> {
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

pub struct Day5 {
    vent_lines: Vec<Line>,
}

impl Day5 {
    pub fn new() -> Self {
        let vent_lines = utils::input_to_lines(INPUT).map(Line::from).collect();
        Self { vent_lines }
    }

    fn intersection_with_vertical(line_a: &Line, line_b: &Line) -> Option<Point> {
        let (vline, other) = if line_a.is_vertical() {
            (line_a, line_b)
        } else {
            (line_b, line_a)
        };

        let vx = vline.p0.x;
        if (other.x_min()..=other.x_max()).contains(&vx) {
            let y = (other.slope.unwrap() * vx) + other.y_intercept.unwrap();
            let p = Point::new(vx, y);
            if vline.contains_point(&p) {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn colinear_intersections(line_a: &Line, line_b: &Line, intersections: &mut HashSet<Point>) {
        // special case for vertical intersections
        if line_a.is_vertical() && line_b.is_vertical() {
            if Line::verticals_intersect(line_a, line_b) {
                let x = line_a.p0.x;
                let isect_start = cmp::max(line_a.y_min(), line_b.y_min());
                let isect_end = cmp::min(line_a.y_max(), line_b.y_max());
                for y in isect_start..=isect_end {
                    intersections.insert(Point::new(x, y));
                }
            }
        } else {
            let slope = line_a.slope.unwrap();
            // sort the lines by x
            let (lline, rline) = Line::sort_by_x(&line_a, &line_b);
            // consider if points on the rightmost line fall along the leftmost
            let (lp, rp) = Point::sort_by_x(&rline.p0, &rline.p1);
            if lline.contains_point(lp) {
                let mut p = lp.clone();
                while p != *rp {
                    if lline.contains_point(&p) {
                        intersections.insert(p.clone());
                    }
                    p.x += 1;
                    p.y += slope;
                }
                // check the endpoint
                if lline.contains_point(&p) {
                    intersections.insert(p);
                }
            }
        }
    }

    fn find_intersections(lines: &[Line]) -> HashSet<Point> {
        let n_lines = lines.len();
        let mut intersections = HashSet::new();

        // check line intersections
        for i in 0..(n_lines - 1) {
            for j in (i + 1)..n_lines {
                let line_i = &lines[i];
                let line_j = &lines[j];
                if line_i.slope == line_j.slope {
                    Self::colinear_intersections(line_i, line_j, &mut intersections);
                } else if line_i.is_vertical() || line_j.is_vertical() {
                    if let Some(p) = Self::intersection_with_vertical(line_i, line_j) {
                        intersections.insert(p);
                    }
                } else if let Some(p) = Line::intersection(line_i, line_j) {
                    intersections.insert(p);
                }
            }
        }

        intersections
    }
}

impl Puzzle for Day5 {
    // Consider only horizontal and vertical lines. At how many points do at
    // least two lines overlap?
    fn part_1(&self) -> Result<Solution> {
        // filter horizontal/vertical lines
        let horizontal_vertical = self
            .vent_lines
            .iter()
            .filter(|l| l.is_horizontal() || l.is_vertical())
            // note: need to dereference
            .cloned()
            .collect::<Vec<_>>();
        let intersections = Self::find_intersections(&horizontal_vertical);
        Ok(intersections.len().into())
    }

    // Consider all of the lines. At how many points do at least two lines
    // overlap?
    fn part_2(&self) -> Result<Solution> {
        let intersections = Self::find_intersections(&self.vent_lines);
        Ok(intersections.len().into())
    }
}
