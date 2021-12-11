/*
** src/types.rs
*/

use std::error;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// variant to cover various solution types
#[derive(Debug)]
pub enum Solution {
    Int(i64),
    UInt(u64),
}

impl From<i32> for Solution {
    fn from(n: i32) -> Self {
        Self::Int(n as i64)
    }
}

impl From<i64> for Solution {
    fn from(n: i64) -> Self {
        Self::Int(n)
    }
}

impl From<u32> for Solution {
    fn from(n: u32) -> Self {
        Self::UInt(n as u64)
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

#[derive(Debug)]
pub enum PuzzleError {
    NoSolution,
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSolution => write!(f, "no solution found"),
        }
    }
}

impl error::Error for PuzzleError {}

macro_rules! bind_els {
    ($self:expr, $a:ident, $b:ident) => {
        let $a = $self.data[0];
        let $b = $self.data[1];
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident) => {
        let $a = $self.data[0];
        let $b = $self.data[1];
        let $c = $self.data[2];
        let $d = $self.data[3];
    };
}

pub struct Vector2<T> {
    pub data: [T; 2],
}

impl<T> Vector2<T> {
    pub fn new(a: T, b: T) -> Self {
        let data = [a, b];
        Self { data }
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Copy,
    T: Div<T, Output = T>,
{
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Self::Output {
        bind_els!(self, a, b);
        Vector2::new(a / rhs, b / rhs)
    }
}

pub struct Matrix2D<T> {
    data: [T; 4],
}

impl<T> Matrix2D<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Div<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Neg<Output = T>,
    T: Sub<T, Output = T>,
{
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        let data = [a, b, c, d];
        Self { data }
    }

    pub fn determinant(&self) -> T {
        bind_els!(&self, a, b, c, d);
        (a * d) - (b * c)
    }

    pub fn solve_system(m: &Self, v: &Vector2<T>) -> Vector2<T> {
        bind_els!(m, a, b, c, d);
        // note: save the division for last in case of integer division
        let det = m.determinant();
        let m_inv = Self::new(d, -b, -c, a);
        (m_inv * v) / det
    }
}

impl<T> Mul<&Vector2<T>> for Matrix2D<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Vector2<T>;

    fn mul(self, rhs: &Vector2<T>) -> Self::Output {
        bind_els!(self, a, b, c, d);
        bind_els!(rhs, e, f);
        Vector2::new((a * e) + (b * f), (c * e) + (d * f))
    }
}
