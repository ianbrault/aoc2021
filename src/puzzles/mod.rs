/*
** src/puzzles/mod.rs
*/

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use crate::types::Puzzle;

pub fn all() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(day_1::Day1::new()),
        Box::new(day_2::Day2::new()),
        Box::new(day_3::Day3::new()),
        Box::new(day_4::Day4::new()),
        Box::new(day_5::Day5::new()),
        Box::new(day_6::Day6::new()),
        Box::new(day_7::Day7::new()),
        Box::new(day_8::Day8::new()),
        Box::new(day_9::Day9::new()),
        Box::new(day_10::Day10::new()),
        Box::new(day_11::Day11::new()),
        Box::new(day_12::Day12::new()),
        Box::new(day_13::Day13::new()),
        Box::new(day_14::Day14::new()),
        Box::new(day_15::Day15::new()),
    ]
}
