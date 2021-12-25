/*
** src/puzzles/day_13.rs
** https://adventofcode.com/2021/day/13
*/

use crate::types::{Point, Puzzle, Result, Solution};

use std::cell::RefCell;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/13.txt");

#[derive(Debug)]
enum Fold {
    X(i64),
    Y(i64),
}

impl Fold {
    fn reflect_point(&self, point: &Point) -> Point {
        match self {
            Self::X(x) => point.reflect_x(*x),
            Self::Y(y) => point.reflect_y(*y),
        }
    }
}

impl From<&str> for Fold {
    fn from(s: &str) -> Self {
        let line = s.split(' ').last().unwrap();
        split_into!(line, '=', axis, point);
        match axis {
            "x" => Fold::X(point.parse().unwrap()),
            "y" => Fold::Y(point.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

pub struct Day13 {
    // need RefCell for interior mutability
    points: RefCell<HashSet<Point>>,
    folds: Vec<Fold>,
}

impl Day13 {
    pub fn new() -> Self {
        match split!(INPUT, "\n\n") {
            [point_strings, fold_strings] => {
                let points = RefCell::new(point_strings.split('\n').map(Point::from).collect());
                let folds = fold_strings.split('\n').map(Fold::from).collect();
                Self { points, folds }
            }
            _ => unreachable!(),
        }
    }

    fn point_eligible_for_fold(point: &Point, fold: &Fold) -> bool {
        match fold {
            Fold::X(x) => point.x > *x,
            Fold::Y(y) => point.y > *y,
        }
    }

    fn perform_fold(&self, fold: &Fold) {
        let mut new_points = HashSet::new();

        for point in self.points.borrow().iter() {
            new_points.insert(if Self::point_eligible_for_fold(point, fold) {
                fold.reflect_point(point)
            } else {
                point.clone()
            });
        }

        let _ = self.points.replace(new_points);
    }

    fn print_grid(&self) -> String {
        let mut grid = vec![String::new()];
        let x_max = self.points.borrow().iter().map(|p| p.x).max().unwrap();
        let y_max = self.points.borrow().iter().map(|p| p.y).max().unwrap();
        for y in 0..=y_max {
            let mut s = String::with_capacity(x_max as usize);
            let px = self.points.borrow().iter().filter(|p| p.y == y).map(|p| p.x).collect::<HashSet<_>>();
            for x in 0..=x_max {
                s += if px.contains(&x) {
                    "#"
                } else {
                    " "
                };
            }
            grid.push(s);
        }
        grid.join("\n")
    }
}

impl Puzzle for Day13 {
    // How many dots are visible after completing just the first fold instruction on your
    // transparent paper?
    fn part_1(&self) -> Result<Solution> {
        self.perform_fold(&self.folds[0]);
        Ok(self.points.borrow().len().into())
    }

    // Finish folding the transparent paper according to the instructions. The manual says the code
    // is always eight capital letters. What code do you use to activate the infrared thermal
    // imaging camera system?
    fn part_2(&self) -> Result<Solution> {
        for fold in self.folds.iter().skip(1) {
            self.perform_fold(fold);
        }
        Ok(self.print_grid().into())
    }
}
