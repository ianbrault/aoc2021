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
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use crate::types::Puzzle;

const INPUTS: [&str; 20] = [
    include_str!("../../input/1.txt"),
    include_str!("../../input/2.txt"),
    include_str!("../../input/3.txt"),
    include_str!("../../input/4.txt"),
    include_str!("../../input/5.txt"),
    include_str!("../../input/6.txt"),
    include_str!("../../input/7.txt"),
    include_str!("../../input/8.txt"),
    include_str!("../../input/9.txt"),
    include_str!("../../input/10.txt"),
    include_str!("../../input/11.txt"),
    include_str!("../../input/12.txt"),
    include_str!("../../input/13.txt"),
    include_str!("../../input/14.txt"),
    include_str!("../../input/15.txt"),
    include_str!("../../input/16.txt"),
    include_str!("../../input/17.txt"),
    include_str!("../../input/18.txt"),
    include_str!("../../input/19.txt"),
    include_str!("../../input/20.txt"),
];

pub fn all() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(day_1::Day1::new(INPUTS[0])),
        Box::new(day_2::Day2::new(INPUTS[1])),
        Box::new(day_3::Day3::new(INPUTS[2])),
        Box::new(day_4::Day4::new(INPUTS[3])),
        Box::new(day_5::Day5::new(INPUTS[4])),
        Box::new(day_6::Day6::new(INPUTS[5])),
        Box::new(day_7::Day7::new(INPUTS[6])),
        Box::new(day_8::Day8::new(INPUTS[7])),
        Box::new(day_9::Day9::new(INPUTS[8])),
        Box::new(day_10::Day10::new(INPUTS[9])),
        Box::new(day_11::Day11::new(INPUTS[10])),
        Box::new(day_12::Day12::new(INPUTS[11])),
        Box::new(day_13::Day13::new(INPUTS[12])),
        Box::new(day_14::Day14::new(INPUTS[13])),
        Box::new(day_15::Day15::new(INPUTS[14])),
        Box::new(day_16::Day16::new(INPUTS[15])),
        Box::new(day_17::Day17::new(INPUTS[16])),
        Box::new(day_18::Day18::new(INPUTS[17])),
        Box::new(day_19::Day19::new(INPUTS[18])),
        Box::new(day_20::Day20::new(INPUTS[19])),
    ]
}
