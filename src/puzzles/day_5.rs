/*
** src/puzzles/day_5.rs
** https://adventofcode.com/2021/day/5
*/

use crate::types::{Line, Point, Puzzle, Result, Solution};
use crate::utils;

use std::cmp;
use std::collections::HashSet;

pub struct Day5 {
    vent_lines: Vec<Line>,
}

impl Day5 {
    pub fn new(input: &'static str) -> Self {
        let vent_lines = utils::input_to_lines(input).map(Line::from).collect();
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
            let (lline, rline) = Line::sort_by_x(line_a, line_b);
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
