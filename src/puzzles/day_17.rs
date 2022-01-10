/*
** src/puzzles/day_17.rs
** https://adventofcode.com/2021/day/17
*/

use crate::types::{Puzzle, Result, Solution};

use std::cmp;
use std::ops::Range;

pub struct Day17 {
    x_range: Range<i64>,
    y_range: Range<i64>,
}

impl Day17 {
    pub fn new(input: &'static str) -> Self {
        split_into!(input, ": ", _x, ranges);
        split_into!(ranges, ", ", x, y);
        let x_range = Self::parse_range(x);
        let y_range = Self::parse_range(y);
        Self { x_range, y_range }
    }

    fn parse_range(s: &str) -> Range<i64> {
        let s = &s[2..s.len()];
        split_into!(s, "..", start, end);
        start.parse().unwrap()..(end.parse::<i64>().unwrap() + 1)
    }

    // does the probe, when launched at the given velocity, land within the target area?
    fn launch_probe(&self, vx: i64, vy: i64) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut vx = vx;
        let mut vy = vy;

        while x <= self.x_range.end && y >= self.y_range.end {
            x += vx;
            y += vy;

            if vx > 0 {
                vx -= 1;
            } else {
                // extra check to terminate early
                if x == 0 && !self.x_range.contains(&x) {
                    return false;
                }
            }
            vy -= 1;
        }

        self.x_range.contains(&x) && self.y_range.contains(&y)
    }

    fn max_y(&self, vx: i64, vy: i64) -> i64 {
        let mut x = 0;
        let mut y = 0;
        let mut vx = vx;
        let mut vy = vy;

        let mut max_y = 0;
        while x <= self.x_range.end && y >= self.y_range.end {
            x += vx;
            y += vy;
            max_y = cmp::max(y, max_y);

            if vx > 0 {
                vx -= 1;
            }
            vy -= 1;
        }

        max_y
    }
}

impl Puzzle for Day17 {
    // Find the initial velocity that causes the probe to reach the highest y position and still
    // eventually be within the target area after any step. What is the highest y position it
    // reaches on this trajectory?
    fn part_1(&self) -> Result<Solution> {
        // note: just brute-force it
        // initial vx and vy must be positive
        let mut y_max = 0;
        for vx in 1..=self.x_range.end {
            for vy in 1..=1000 {
                if self.launch_probe(vx, vy) {
                    y_max = cmp::max(y_max, self.max_y(vx, vy));
                }
            }
        }
        Ok(y_max.into())
    }

    // How many distinct initial velocity values cause the probe to be within the target area after
    // any step?
    fn part_2(&self) -> Result<Solution> {
        // note: just brute-force it
        // initial vx must be positive
        let mut count = 0;
        for vx in 1..=self.x_range.end {
            for vy in self.y_range.start..=1000 {
                if self.launch_probe(vx, vy) {
                    count += 1
                }
            }
        }
        Ok(count.into())
    }
}
