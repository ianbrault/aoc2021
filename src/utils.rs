/*
** src/utils.rs
*/

use std::iter::Peekable;
use std::str::FromStr;

// a macro for a split-and-match pattern which is used frequently
macro_rules! split {
    ($string:ident, $splitter:expr) => {
        $string.split($splitter).collect::<Vec<&str>>().as_slice()
    };
}

// similar to the split! macro above, but binds the provided identifiers
macro_rules! split_into {
    ($string:ident, $splitter:expr, $($var:ident),+) => {
        let ($($var),+) = match split!($string, $splitter) {
            [$($var),+] => ($(*$var),+),
            _ => unreachable!(),
        };
    };
}

// bind a variable to each Vec member when the vector is a known size
macro_rules! bind_vec_deref {
    ($vec:expr, $($var:ident),+) => {
        let ($($var),+) = match $vec.as_slice() {
            [$($var),+] => ($(*$var),+),
            _ => unreachable!(),
        };
    };
}

// splits input into non-empty lines
pub fn input_to_lines(input: &'static str) -> impl Iterator<Item = &str> {
    input.split('\n').filter(|s| !s.is_empty())
}

// splits input into non-empty lines, and parses a type from each line
pub fn input_to_parsed_lines<T>(input: &'static str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input_to_lines(input).map(|s| s.parse::<T>().unwrap())
}

// takes an iterator and transforms it into a new iterator which combines the
// current and next elements using the provided function
pub struct PairWithIter<I, F>
where
    I: Iterator,
{
    inner: Peekable<I>,
    combinator: F,
}

impl<'a, I, N, F> PairWithIter<I, F>
where
    N: 'a,
    I: Iterator<Item = &'a N>,
{
    pub fn new(iter: I, combinator: F) -> Self {
        Self {
            inner: iter.peekable(),
            combinator,
        }
    }
}

impl<'a, I, N, T, F> Iterator for PairWithIter<I, F>
where
    N: 'a,
    T: 'a,
    I: Iterator<Item = &'a N>,
    F: Fn(&'a N, &'a N) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // get the next item
        if let Some(curr) = self.inner.next() {
            // peek the following item
            if let Some(after) = self.inner.peek() {
                Some((self.combinator)(curr, after))
            } else {
                None
            }
        } else {
            None
        }
    }
}

// iterator extension for PairWithIter
pub trait PairWith<'a, N, T, F>: Iterator<Item = &'a N>
where
    Self: Sized,
    N: 'a,
    T: 'a,
    F: Fn(&'a N, &'a N) -> T,
{
    fn pair_with(self, combinator: F) -> PairWithIter<Self, F> {
        PairWithIter::new(self, combinator)
    }
}

impl<'a, N, T, F, I> PairWith<'a, N, T, F> for I
where
    N: 'a,
    T: 'a,
    I: Iterator<Item = &'a N>,
    F: Fn(&'a N, &'a N) -> T,
{
}

// selects the other element in a 2-wide array
pub fn other<T>(array: [T; 2], val: T) -> T
where
    T: PartialEq,
    T: Copy,
{
    if array[0] == val {
        array[1]
    } else {
        array[0]
    }
}
