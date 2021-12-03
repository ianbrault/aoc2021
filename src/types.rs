/*
** src/types.rs
*/

use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// variant to cover various solution types
#[derive(Debug)]
pub enum Solution {
    Int(i64),
    UInt(u64),
}

impl From<i64> for Solution {
    fn from(n: i64) -> Self {
        Self::Int(n)
    }
}

impl From<u64> for Solution {
    fn from(n: u64) -> Self {
        Self::UInt(n)
    }
}

impl From<usize> for Solution {
    fn from(n: usize) -> Self {
        Self::UInt(n as u64)
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::UInt(u) => write!(f, "{}", u),
        }
    }
}
// puzzles are trait objects which conform to the following interface
pub trait Puzzle {
    fn part_1(&self) -> Result<Solution>;
    fn part_2(&self) -> Result<Solution>;
}
