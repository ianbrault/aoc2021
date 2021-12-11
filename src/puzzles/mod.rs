/*
** src/puzzles/mod.rs
*/

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

use crate::types::Puzzle;

pub fn all() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(day_1::Day1::new()),
        Box::new(day_2::Day2::new()),
        Box::new(day_3::Day3::new()),
        Box::new(day_4::Day4::new()),
        Box::new(day_5::Day5::new()),
        Box::new(day_6::Day6::new()),
    ]
}
