/*
** src/puzzles/day_10.rs
** https://adventofcode.com/2021/day/10
*/

use crate::types::{Puzzle, Result, Solution};
use crate::utils;

const INPUT: &str = include_str!("../../input/10.txt");

pub struct Day10 {
    lines: Vec<&'static str>,
}

impl Day10 {
    pub fn new() -> Self {
        let lines = utils::input_to_lines(INPUT).collect();
        Self { lines }
    }

    fn is_opener(c: char) -> bool {
        matches!(c, '(' | '[' | '{' | '<')
    }

    fn is_closer(c: char) -> bool {
        matches!(c, ')' | ']' | '}' | '>')
    }

    fn get_closer(opener: char) -> char {
        match opener {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unreachable!(),
        }
    }

    fn opener_matches_closer(opener: char, closer: char) -> bool {
        match opener {
            '(' => closer == ')',
            '[' => closer == ']',
            '{' => closer == '}',
            '<' => closer == '>',
            _ => unreachable!(),
        }
    }

    fn score(c: char) -> u64 {
        match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    }

    fn syntax_error_score(c: char) -> u64 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }

    fn first_illegal_character(line: &str) -> Option<char> {
        let mut stack = Vec::new();

        for c in line.chars() {
            if Self::is_opener(c) {
                stack.push(c);
            } else if Self::is_closer(c) {
                // ensure that the top of the stack matches
                let top = stack.pop().unwrap();
                if !Self::opener_matches_closer(top, c) {
                    return Some(c);
                }
            }
        }

        None
    }

    fn complete_with_score(line: &str) -> u64 {
        let mut score = 0;
        let mut stack = Vec::new();

        for c in line.chars() {
            if Self::is_opener(c) {
                stack.push(c);
            } else if Self::is_closer(c) {
                let _ = stack.pop().unwrap();
            }
        }

        // match un-closed openers to complete the line
        while !stack.is_empty() {
            let opener = stack.pop().unwrap();
            let closer = Self::get_closer(opener);
            score = (score * 5) + Self::score(closer);
        }

        score
    }
}

impl Puzzle for Day10 {
    // Find the first illegal character in each corrupted line of the navigation subsystem. What is
    // the total syntax error score for those errors?
    fn part_1(&self) -> Result<Solution> {
        let syntax_err_score = self
            .lines
            .iter()
            .map(|line| Self::first_illegal_character(line))
            .flatten()
            .map(Self::syntax_error_score)
            .sum::<u64>();
        Ok(syntax_err_score.into())
    }

    // Find the completion string for each incomplete line, score the completion strings, and sort
    // the scores. What is the middle score?
    fn part_2(&self) -> Result<Solution> {
        let mut completion_scores = self
            .lines
            .iter()
            .filter(|line| Self::first_illegal_character(line).is_none())
            .map(|line| Self::complete_with_score(line))
            .collect::<Vec<_>>();
        completion_scores.sort_unstable();
        let score = completion_scores[completion_scores.len() / 2];
        Ok(score.into())
    }
}
