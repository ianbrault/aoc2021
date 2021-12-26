/*
** src/types/mod.rs
*/

mod geometry;

pub use self::geometry::{Line, Point};

use crate::utils;

use num::Integer;

use std::collections::HashMap;
use std::error;
use std::fmt;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// variant to cover various solution types
#[derive(Debug)]
pub enum Solution {
    Int(i64),
    UInt(u64),
    String(String),
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

impl From<String> for Solution {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::UInt(u) => write!(f, "{}", u),
            Self::String(s) => write!(f, "{}", s),
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

pub struct Array2D<T, const W: usize, const H: usize> {
    data: [[T; W]; H],
}

impl<T, const W: usize, const H: usize> Array2D<T, W, H> {
    pub fn new() -> Self
    where
        T: Copy + Default,
    {
        let data = [[T::default(); W]; H];
        Self { data }
    }

    pub const fn left(i: usize, j: usize) -> Option<(usize, usize)> {
        if j > 0 {
            Some((i, j - 1))
        } else {
            None
        }
    }

    pub const fn right(i: usize, j: usize) -> Option<(usize, usize)> {
        if j < W - 1 {
            Some((i, j + 1))
        } else {
            None
        }
    }

    pub const fn up(i: usize, j: usize) -> Option<(usize, usize)> {
        if i > 0 {
            Some((i - 1, j))
        } else {
            None
        }
    }

    pub const fn down(i: usize, j: usize) -> Option<(usize, usize)> {
        if i < H - 1 {
            Some((i + 1, j))
        } else {
            None
        }
    }

    pub const fn up_left(i: usize, j: usize) -> Option<(usize, usize)> {
        if i > 0 && j > 0 {
            Some((i - 1, j - 1))
        } else {
            None
        }
    }

    pub const fn up_right(i: usize, j: usize) -> Option<(usize, usize)> {
        if i > 0 && j < W - 1 {
            Some((i - 1, j + 1))
        } else {
            None
        }
    }

    pub const fn down_left(i: usize, j: usize) -> Option<(usize, usize)> {
        if i < H - 1 && j > 0 {
            Some((i + 1, j - 1))
        } else {
            None
        }
    }

    pub const fn down_right(i: usize, j: usize) -> Option<(usize, usize)> {
        if i < H - 1 && j < W - 1 {
            Some((i + 1, j + 1))
        } else {
            None
        }
    }

    pub const fn neighbors(i: usize, j: usize) -> [Option<(usize, usize)>; 4] {
        [
            Self::left(i, j),
            Self::right(i, j),
            Self::up(i, j),
            Self::down(i, j),
        ]
    }

    pub const fn neighbors_with_diagonal(i: usize, j: usize) -> [Option<(usize, usize)>; 8] {
        [
            Self::left(i, j),
            Self::right(i, j),
            Self::up(i, j),
            Self::down(i, j),
            Self::up_left(i, j),
            Self::up_right(i, j),
            Self::down_left(i, j),
            Self::down_right(i, j),
        ]
    }

    pub fn get(&self, i: usize, j: usize) -> T
    where
        T: Copy,
    {
        self.data[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, val: T) {
        self.data[i][j] = val;
    }

    fn iter_indices(&self) -> impl Iterator<Item = (usize, usize)> {
        itertools::iproduct!(0..H, 0..W)
    }

    pub fn iter_with_indices(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        // need collect().into_iter() in order to avoid lifetime issues with the map closure
        self.iter_indices()
            .map(|(i, j)| (i, j, &self.data[i][j]))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn find_index<P>(&self, predicate: P) -> Option<(usize, usize)>
    where
        P: Fn(&T) -> bool,
    {
        self.iter_with_indices()
            .find(|(_, _, x)| predicate(x))
            .map(|(i, j, _)| (i, j))
    }
}

impl<T, const W: usize, const H: usize> Array2D<T, W, H>
where
    T: Copy + Integer,
{
    pub fn increment(&mut self, i: usize, j: usize) {
        self.data[i][j] = self.data[i][j] + T::one();
    }
}

impl<T, const W: usize, const H: usize> From<&'static str> for Array2D<T, W, H>
where
    T: Copy + Default + FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    fn from(s: &'static str) -> Self {
        let mut arr = Self::new();
        for (i, line) in utils::input_to_lines(s).enumerate() {
            for (j, c) in line.chars().enumerate() {
                arr.data[i][j] = c.to_string().parse().unwrap();
            }
        }
        arr
    }
}

impl<T, const W: usize, const H: usize> Default for Array2D<T, W, H>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

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

pub struct Counter<T> {
    counts: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn insert(&mut self, val: T) {
        let el = self.counts.entry(val).or_insert(0);
        *el += 1;
    }

    pub fn insert_n(&mut self, val: T, count: usize) {
        let el = self.counts.entry(val).or_insert(0);
        *el += count;
    }

    pub fn min(&self) -> Option<usize> {
        self.counts.values().min().copied()
    }

    pub fn max(&self) -> Option<usize> {
        self.counts.values().max().copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.counts.iter()
    }
}

impl<T, I> From<I> for Counter<T>
where
    T: Clone + Eq + Hash,
    I: Iterator<Item = T>,
{
    fn from(it: I) -> Self {
        let mut counts = HashMap::new();
        for el in it {
            let count = counts.entry(el.clone()).or_insert(0);
            *count += 1;
        }
        Self { counts }
    }
}
