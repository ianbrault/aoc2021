/*
** src/puzzles/day_1.rs
** https://adventofcode.com/2021/day/1
*/

use crate::types::{Puzzle, Result, Solution};
use crate::utils::{self, PairWith};

const INPUT: &str = include_str!("../../input/1.txt");

pub struct Day1 {
    sonar_depths: Vec<u64>,
}

impl Day1 {
    pub fn new() -> Self {
        let sonar_depths = utils::input_to_parsed_lines::<u64>(INPUT).collect();
        Self { sonar_depths }
    }
}

impl Puzzle for Day1 {
    // How many measurements are larger than the previous measurement?
    fn part_1(&self) -> Result<Solution> {
        let n = self.sonar_depths
            .iter()
            .pair_with(|x, y| *y as i64 - *x as i64)
            .filter(|&n| n > 0)
            .count();

        Ok(n.into())
    }

    // Consider sums of a three-measurement sliding window. How many sums are
    // larger than the previous sum?
    fn part_2(&self) -> Result<Solution> {
        // generate the three-sums
        let three_sums = self.sonar_depths
            .iter()
            .pair_with(|x, y| y + x)
            .zip(self.sonar_depths.iter().skip(2))
            .map(|(s, n)| s + n)
            .collect::<Vec<_>>();

        let n = three_sums
            .iter()
            .pair_with(|x, y| *y as i64 - *x as i64)
            .filter(|&n| n > 0)
            .count();

        Ok(n.into())
    }
}
