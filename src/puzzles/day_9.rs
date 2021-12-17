/*
** src/puzzles/day_9.rs
** https://adventofcode.com/2021/day/9
*/

use crate::types::{Puzzle, Result, Solution};
use crate::utils;

use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;

const INPUT: &str = include_str!("../../input/9.txt");
const WIDTH: usize = 100;
const HEIGHT: usize = 100;

pub struct Day9 {
    heightmap: [[u8; WIDTH]; HEIGHT],
}

impl Day9 {
    pub fn new() -> Self {
        let mut heightmap = [[0; WIDTH]; HEIGHT];
        for (i, line) in utils::input_to_lines(INPUT).enumerate() {
            for (j, c) in line.chars().enumerate() {
                heightmap[i][j] = c.to_digit(10).unwrap() as u8;
            }
        }
        Self { heightmap }
    }

    const fn left(i: usize, j: usize) -> Option<(usize, usize)> {
        if j > 0 {
            Some((i, j - 1))
        } else {
            None
        }
    }

    const fn right(i: usize, j: usize) -> Option<(usize, usize)> {
        if j < WIDTH - 1 {
            Some((i, j + 1))
        } else {
            None
        }
    }

    const fn up(i: usize, j: usize) -> Option<(usize, usize)> {
        if i > 0 {
            Some((i - 1, j))
        } else {
            None
        }
    }

    const fn down(i: usize, j: usize) -> Option<(usize, usize)> {
        if i < HEIGHT - 1 {
            Some((i + 1, j))
        } else {
            None
        }
    }

    fn neighbors(&self, i: usize, j: usize) -> [Option<u8>; 4] {
        let neighbor_coords = [
            Self::left(i, j),
            Self::right(i, j),
            Self::up(i, j),
            Self::down(i, j),
        ];
        neighbor_coords
            .iter()
            .map(|n| n.map(|(i, j)| self.heightmap[i][j]))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn neighbors_with_coords(&self, i: usize, j: usize) -> [Option<(usize, usize, u8)>; 4] {
        let neighbor_coords = [
            Self::left(i, j),
            Self::right(i, j),
            Self::up(i, j),
            Self::down(i, j),
        ];
        neighbor_coords
            .iter()
            .map(|n| n.map(|(i, j)| (i, j, self.heightmap[i][j])))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn is_lowpoint(&self, i: usize, j: usize) -> bool {
        let here = self.heightmap[i][j];
        self.neighbors(i, j)
            .iter()
            .filter_map(|&x| x)
            .all(|x| x > here)
    }

    fn basin_size(&self, i: usize, j: usize) -> usize {
        // points to be explored
        let mut frontier = VecDeque::new();
        // points already explored
        let mut explored = HashSet::new();

        // start with the given point
        frontier.push_back((i, j));

        while !frontier.is_empty() {
            // pop from the front of the frontier
            let (ii, jj) = frontier.pop_front().unwrap();
            // add unexplored neighbors to the frontier
            // note: exclude neighbors at the maximum height (9)
            for (iii, jjj, v) in self.neighbors_with_coords(ii, jj).iter().flatten() {
                if !explored.contains(&(*iii, *jjj)) && *v < 9 {
                    frontier.push_back((*iii, *jjj));
                }
            }
            // add the current point to the explored set
            explored.insert((ii, jj));
        }

        explored.len()
    }
}

impl Puzzle for Day9 {
    // Find all of the low points on your heightmap. What is the sum of the
    // risk levels of all low points on your heightmap?
    fn part_1(&self) -> Result<Solution> {
        let mut sum = 0;

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if self.is_lowpoint(i, j) {
                    sum += 1 + self.heightmap[i][j] as u64;
                }
            }
        }

        Ok(sum.into())
    }

    // What do you get if you multiply together the sizes of the three largest
    // basins?
    fn part_2(&self) -> Result<Solution> {
        // gather all low points and determine the sizes of their corresponding basins
        let mut lowpoints = itertools::iproduct!(0..HEIGHT, 0..WIDTH)
            .filter(|(i, j)| self.is_lowpoint(*i, *j))
            .map(|(i, j)| self.basin_size(i, j))
            .collect::<Vec<_>>();
        // sort and grab the 3 largest basins
        lowpoints.sort_unstable();
        let res = lowpoints.iter().rev().take(3).product::<usize>();

        Ok(res.into())
    }
}
