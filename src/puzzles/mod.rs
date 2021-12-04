/*
** src/puzzles/mod.rs
*/

mod day_1;
mod day_2;

use crate::types::Puzzle;

pub fn all() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(day_1::Day1::new()), Box::new(day_2::Day2::new())]
}
