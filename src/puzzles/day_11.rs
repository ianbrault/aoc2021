/*
** src/puzzles/day_11.rs
** https://adventofcode.com/2021/day/11
*/

use crate::types::{Array2D, Puzzle, PuzzleError, Result, Solution};

use std::cell::RefCell;

const INPUT: &str = include_str!("../../input/11.txt");
const SIZE: usize = 10;

pub struct Day11 {
    // need RefCell for interior mutability
    energy_levels: RefCell<Array2D<u8, SIZE, SIZE>>,
}

impl Day11 {
    fn load_energy_levels(s: &'static str) -> Array2D<u8, SIZE, SIZE> {
        Array2D::from(s)
    }

    pub fn new() -> Self {
        let energy_levels = RefCell::new(Self::load_energy_levels(INPUT));
        Self { energy_levels }
    }

    // returns the number of flashes in the step
    fn run_step(&self) -> u64 {
        let mut flashes = 0;
        // copy out the energy level grid and replace it at the end to avoid borrowing concerns
        let mut grid = self.energy_levels.take();

        // first increment all energy levels by 1
        for row in 0..SIZE {
            for col in 0..SIZE {
                grid.increment(row, col);
            }
        }

        // handle all flashes
        while let Some((i, j)) = grid.find_index(|&x| x > 9) {
            flashes += 1;
            // set the energy level to 0
            grid.set(i, j, 0);
            // increment the energy level of all neighboring octopi
            for (ii, jj) in Array2D::<u8, SIZE, SIZE>::neighbors_with_diagonal(i, j)
                .iter()
                .flatten()
            {
                // note: do not increment if 0
                if grid.get(*ii, *jj) != 0 {
                    grid.increment(*ii, *jj);
                }
            }
        }

        // replace the grid and return
        let _ = self.energy_levels.replace(grid);
        flashes
    }

    // returns the sum of the number of flashes in each step
    fn run_steps(&self, n: usize) -> u64 {
        (0..n).map(|_| self.run_step()).sum()
    }
}

impl Puzzle for Day11 {
    // Given the starting energy levels of the dumbo octopuses in your cavern, simulate 100 steps.
    // How many total flashes are there after 100 steps?
    fn part_1(&self) -> Result<Solution> {
        Ok(self.run_steps(100).into())
    }

    // What is the first step during which all octopuses flash?
    fn part_2(&self) -> Result<Solution> {
        // first reset the grid
        let _ = self.energy_levels.replace(Self::load_energy_levels(INPUT));

        let all_flash = (SIZE * SIZE) as u64;
        for step in 0..u64::MAX {
            let n = self.run_step();
            if n == all_flash {
                // note: solution steps are 1-indexed
                return Ok((step + 1).into());
            }
        }

        Err(PuzzleError::NoSolution.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";

    fn get_day() -> Day11 {
        let energy_levels = RefCell::new(Day11::load_energy_levels(TEST_INPUT));
        Day11 { energy_levels }
    }

    #[test]
    fn test_flashes() {
        let day = get_day();
        // print_grid(&day);
        assert_eq!(day.run_step(), 0);
        // print_grid(&day);
        assert_eq!(day.run_step(), 35);
        // print_grid(&day);
    }

    #[test]
    fn test_flashes_synchronized() {
        let day = get_day();
        // should synchronize on step 195
        let _ = day.run_steps(194);
        assert_eq!(day.run_step(), (SIZE * SIZE) as u64);
        // print_grid(&day);
    }
}
