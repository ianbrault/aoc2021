/*
** src/puzzles/day_15.rs
** https://adventofcode.com/2021/day/15
*/

use crate::types::{Array2D, Puzzle, Result, Solution};

use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../../input/15.txt");

const SIZE: usize = 100;
const FULL_SIZE: usize = SIZE * 5;

type Coord = (usize, usize);

// tracks the path distances
#[derive(Clone, Copy, Eq, PartialEq)]
struct CoordDistance {
    coord: Coord,
    distance: u64,
}

impl CoordDistance {
    fn new(coord: Coord, distance: u64) -> Self {
        Self { coord, distance }
    }
}

impl Ord for CoordDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for CoordDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day15 {
    cave: Array2D<u8, SIZE, SIZE>,
    cave_full: Array2D<u8, FULL_SIZE, FULL_SIZE>,
}

impl Day15 {
    pub fn new() -> Self {
        let cave = Array2D::from(INPUT);
        let mut cave_full = Array2D::new();
        Self::build_full_cave(&cave, &mut cave_full);
        Self { cave, cave_full }
    }

    fn build_full_cave(
        cave: &Array2D<u8, SIZE, SIZE>,
        full_cave: &mut Array2D<u8, FULL_SIZE, FULL_SIZE>,
    ) {
        for row in 0..5 {
            let row_offset = row * SIZE;
            for col in 0..5 {
                let col_offset = col * SIZE;
                for i in 0..SIZE {
                    for j in 0..SIZE {
                        let original = cave.get(i, j);
                        let new = original + row as u8 + col as u8;
                        if new > 9 {
                            full_cave.set(row_offset + i, col_offset + j, new % 9);
                        } else {
                            full_cave.set(row_offset + i, col_offset + j, new);
                        }
                    }
                }
            }
        }
    }

    // implementation of Djikstra's algorithm to find the lowest-risk (i.e. shortest) path between
    // the start and endpoint of the cave
    fn lowest_risk_path<const N: usize>(&self, cave: &Array2D<u8, N, N>) -> u64 {
        let size = N;
        let total_size = size * size;

        let origin = (0, 0);
        let index = |(i, j)| (i * size) + j;

        // assign distance 0 for the origin and infinity for all other nodes
        let mut distances = (0..total_size).map(|_| u64::MAX).collect::<Vec<_>>();
        distances[0] = 0;

        // easily select the next node
        let mut distance_heap = BinaryHeap::new();
        distance_heap.push(CoordDistance::new(origin, 0));

        while let Some(CoordDistance { coord, distance }) = distance_heap.pop() {
            // skip if we have already found a shorter distance to this coordinate
            if distance <= distances[index(coord)] {
                // consider all neighbors
                for neighbor in Array2D::<u8, N, N>::neighbors(coord.0, coord.1)
                    .iter()
                    .filter_map(|coord| *coord)
                {
                    let tmp_distance = distance + cave.get(neighbor.0, neighbor.1) as u64;
                    if tmp_distance < distances[index(neighbor)] {
                        distance_heap.push(CoordDistance::new(neighbor, tmp_distance));
                        distances[index(neighbor)] = tmp_distance;
                    }
                }
            }
        }

        distances[total_size - 1]
    }
}

impl Puzzle for Day15 {
    // What is the lowest total risk of any path from the top left to the bottom right?
    fn part_1(&self) -> Result<Solution> {
        Ok(self.lowest_risk_path(&self.cave).into())
    }

    // Using the full map, what is the lowest total risk of any path from the top left to the
    // bottom right?
    fn part_2(&self) -> Result<Solution> {
        Ok(self.lowest_risk_path(&self.cave_full).into())
    }
}
