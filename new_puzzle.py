#!/usr/bin/env python3

import os
import pathlib
import sys

mod_template = """\
/*
** src/puzzles/mod.rs
*/

<M>

use crate::types::Puzzle;

pub fn all() -> Vec<Box<dyn Puzzle>> {
    vec![
<P>
    ]
}
"""

puzzle_template = """\
/*
** src/puzzles/day_<D>.rs
** https://adventofcode.com/2021/day/<D>
*/

use crate::types::{Puzzle, PuzzleError, Result, Solution};

const INPUT: &str = include_str!("../../input/<D>.txt");

pub struct Day<D> {}

impl Day<D> {
    pub fn new() -> Self {
        Self {}
    }
}

impl Puzzle for Day<D> {
    // [QUESTION]
    fn part_1(&self) -> Result<Solution> {
        Err(PuzzleError::NoSolution.into())
    }

    // [QUESTION]
    fn part_2(&self) -> Result<Solution> {
        Err(PuzzleError::NoSolution.into())
    }
}
"""


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit("error: missing argument DAY")

    n = sys.argv[1]
    try:
        n = int(n)
    except ValueError:
        sys.exit(f"error: invalid argument DAY: {n}")

    current_dir = os.path.dirname(os.path.abspath(__file__))
    puzzle_dir = os.path.join(current_dir, "src", "puzzles")
    input_dir = os.path.join(current_dir, "input")

    # write the puzzle source file
    with open(os.path.join(puzzle_dir, f"day_{n}.rs"), "w") as puzzle_file:
        puzzle_file.write(puzzle_template.replace("<D>", str(n)))

    # write the mod.rs file
    with open(os.path.join(puzzle_dir, "mod.rs"), "w") as mod_file:
        mods = "\n".join(f"mod day_{i + 1};" for i in range(n))
        puzzles = "\n".join(
            f"        Box::new(day_{i + 1}::Day{i + 1}::new()),"
            for i in range(n))
        mod_file.write(
            mod_template.replace("<M>", mods).replace("<P>", puzzles))

    # touch the input file to prevent errors
    pathlib.Path(os.path.join(input_dir, f"{n}.txt")).touch()
