/*
** src/puzzles/mod.rs
*/

mod day_1;

use crate::types::Puzzle;

pub fn all() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(day_1::Day1::new()),
    ]
}
