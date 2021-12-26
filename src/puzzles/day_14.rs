/*
** src/puzzles/day_14.rs
** https://adventofcode.com/2021/day/14
*/

use crate::types::{Counter, Puzzle, Result, Solution};

use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/14.txt");

#[derive(Clone, Eq, Hash, PartialEq)]
struct Pair(char, char);

impl Pair {
    fn new(c1: char, c2: char) -> Self {
        Self(c1, c2)
    }
}

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        if s.len() != 2 {
            unreachable!()
        }
        Self(s.chars().next().unwrap(), s.chars().nth(1).unwrap())
    }
}

type PairCounter = Counter<Pair>;

impl PairCounter {
    fn parse(s: &str) -> Self {
        let mut counter = Counter::new();
        for (c1, c2) in s.chars().zip(s.chars().skip(1)) {
            counter.insert(Pair::new(c1, c2));
        }
        counter
    }
}

pub struct Day14 {
    template: &'static str,
    rules: HashMap<Pair, char>,
}

impl Day14 {
    pub fn new() -> Self {
        match split!(INPUT, "\n\n") {
            [template, rules_str] => {
                let rules = rules_str.split('\n').map(Self::parse_rule).collect();
                Self { template, rules }
            }
            _ => unreachable!(),
        }
    }

    fn parse_rule(s: &str) -> (Pair, char) {
        match split!(s, " -> ") {
            [pair, sub] => (Pair::from(*pair), sub.chars().next().unwrap()),
            _ => unreachable!(),
        }
    }

    fn matches_rule(&self, pair: &Pair) -> Option<(Pair, Pair)> {
        if let Some(&c) = self.rules.get(pair) {
            let pa = Pair::new(pair.0, c);
            let pb = Pair::new(c, pair.1);
            Some((pa, pb))
        } else {
            None
        }
    }

    fn apply_pair_insertion(&self, input: PairCounter) -> PairCounter {
        let mut output = Counter::new();

        for (pair, &count) in input.iter() {
            if let Some((new_pair_a, new_pair_b)) = self.matches_rule(pair) {
                output.insert_n(new_pair_a, count);
                output.insert_n(new_pair_b, count);
            } else {
                output.insert_n(pair.clone(), count);
            }
        }

        output
    }

    fn pair_counter_to_char_counter(pair_counts: PairCounter) -> Counter<char> {
        let mut char_counts = Counter::new();
        for (pair, &count) in pair_counts.iter() {
            char_counts.insert_n(pair.0, count);
            char_counts.insert_n(pair.1, count);
        }

        let mut output = Counter::new();
        for (&c, &count) in char_counts.iter() {
            output.insert_n(c, (count + 1) / 2);
        }
        output
    }
}

impl Puzzle for Day14 {
    // Apply 10 steps of pair insertion to the polymer template and find the most and least common
    // elements in the result. What do you get if you take the quantity of the most common element
    // and subtract the quantity of the least common element?
    fn part_1(&self) -> Result<Solution> {
        let mut input = PairCounter::parse(self.template);
        for _ in 0..10 {
            input = self.apply_pair_insertion(input);
        }
        let counts = Self::pair_counter_to_char_counter(input);
        let min = counts.min().unwrap();
        let max = counts.max().unwrap();
        Ok((max - min).into())
    }

    // Apply 40 steps of pair insertion to the polymer template and find the most and least common
    // elements in the result. What do you get if you take the quantity of the most common element
    // and subtract the quantity of the least common element?
    fn part_2(&self) -> Result<Solution> {
        let mut input = PairCounter::parse(self.template);
        for _ in 0..40 {
            input = self.apply_pair_insertion(input);
        }
        let counts = Self::pair_counter_to_char_counter(input);
        let min = counts.min().unwrap();
        let max = counts.max().unwrap();
        Ok((max - min).into())
    }
}
