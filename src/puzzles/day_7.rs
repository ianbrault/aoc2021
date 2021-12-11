/*
** src/puzzles/day_7.rs
** https://adventofcode.com/2021/day/7
*/

use crate::types::{Puzzle, Result, Solution};

const INPUT: &str = include_str!("../../input/7.txt");

pub struct Day7 {
    input: Vec<i64>,
}

impl Day7 {
    pub fn new() -> Self {
        let input = INPUT.split(',').map(|n| n.parse().unwrap()).collect();
        Self { input }
    }
}

impl Puzzle for Day7 {
    // Determine the horizontal position that the crabs can align to using the
    // least fuel possible. How much fuel must they spend to align to that
    // position?
    fn part_1(&self) -> Result<Solution> {
        // the most efficient position is the median of the inputs
        let mut numbers = self.input.clone();
        numbers.sort_unstable();
        let median = numbers[numbers.len() / 2];

        // determine the fuel used to align all crabs at the median
        let fuel = self.input.iter().map(|n| i64::abs(n - median)).sum::<i64>();
        Ok(fuel.into())
    }

    // As each crab moves, moving further becomes more expensive. How much fuel
    // must they spend to align to that position?
    fn part_2(&self) -> Result<Solution> {
        // the most efficient position is the average of the inputs
        let average = self.input.iter().sum::<i64>() as f64 / self.input.len() as f64;
        let average_int = average.floor() as i64;

        // determine the fuel used to align all crabs at the median
        let fuel = self
            .input
            .iter()
            .map(|n| i64::abs(n - average_int))
            .map(|n| (0..=n).sum::<i64>())
            .sum::<i64>();
        Ok(fuel.into())
    }
}
