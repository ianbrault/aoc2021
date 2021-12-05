/*
** src/puzzles/day_4.rs
** https://adventofcode.com/2021/day/4
*/

use crate::types::{Puzzle, PuzzleError, Result, Solution};

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../input/4.txt");
const BINGO_SIZE: usize = 5;

#[derive(Debug)]
struct BingoBoard {
    // stores the numbers on the card
    numbers: HashSet<u8>,
    // stores the positions of the numbers
    positions: HashMap<u8, usize>,
    // stores marked number positions
    marked: HashSet<usize>,
}

impl BingoBoard {
    fn reset(&mut self) {
        self.marked.clear();
    }

    fn mark(&mut self, number: u8) {
        if self.numbers.contains(&number) {
            self.marked.insert(*self.positions.get(&number).unwrap());
        }
    }

    fn contains_row(&self, pos: usize) -> bool {
        pos % 5 == 0
            && self.marked.contains(&(pos + 1))
            && self.marked.contains(&(pos + 2))
            && self.marked.contains(&(pos + 3))
            && self.marked.contains(&(pos + 4))
    }

    fn contains_col(&self, pos: usize) -> bool {
        self.marked.contains(&(pos + BINGO_SIZE))
            && self.marked.contains(&(pos + (BINGO_SIZE * 2)))
            && self.marked.contains(&(pos + (BINGO_SIZE * 3)))
            && self.marked.contains(&(pos + (BINGO_SIZE * 4)))
    }

    fn is_complete(&self) -> bool {
        // check rows/columns with marked positions
        for &pos in self.marked.iter() {
            if self.contains_row(pos) || self.contains_col(pos) {
                return true;
            }
        }
        false
    }

    fn score(&self, final_number: u8) -> u64 {
        let mut sum: u64 = 0;
        for number in self.numbers.iter() {
            let pos = self.positions.get(number).unwrap();
            if !self.marked.contains(&pos) {
                sum += *number as u64;
            }
        }
        sum * final_number as u64
    }
}

impl From<&str> for BingoBoard {
    fn from(s: &str) -> Self {
        let mut numbers = HashSet::new();
        let mut positions = HashMap::new();
        for (pos, num_str) in s.split_whitespace().filter(|ss| !ss.is_empty()).enumerate() {
            let num = num_str.parse().unwrap();
            numbers.insert(num);
            positions.insert(num, pos);
        }

        Self {
            numbers,
            positions,
            marked: HashSet::new(),
        }
    }
}

pub struct Day4 {
    numbers: Vec<u8>,
    // need RefCell for interior mutability
    bingo_boards: Vec<RefCell<BingoBoard>>,
}

impl Day4 {
    pub fn new() -> Self {
        let parts = INPUT.split("\n\n").collect::<Vec<_>>();
        let numbers = parts[0].split(',').map(|n| n.parse().unwrap()).collect();
        let bingo_boards = parts
            .iter()
            .skip(1)
            .map(|&s| RefCell::new(BingoBoard::from(s)))
            .collect();
        Self {
            numbers,
            bingo_boards,
        }
    }

    fn mark_boards(&self, number: u8) {
        for board in self.bingo_boards.iter() {
            board.borrow_mut().mark(number);
        }
    }

    fn reset_boards(&self) {
        for board in self.bingo_boards.iter() {
            board.borrow_mut().reset();
        }
    }
}

impl Puzzle for Day4 {
    // Figure out which board will win first. What will your final score be if
    // you choose that board?
    fn part_1(&self) -> Result<Solution> {
        for &number in self.numbers.iter() {
            // mark each board
            self.mark_boards(number);
            // check if any are complete
            for board in self.bingo_boards.iter() {
                if board.borrow().is_complete() {
                    let score = board.borrow().score(number);
                    // reset the boards before returning
                    self.reset_boards();
                    return Ok(score.into());
                }
            }
        }

        // reset the boards before returning
        self.reset_boards();
        Err(PuzzleError::NoSolution)?
    }

    // Figure out which board will win last. Once it wins, what would its final
    // score be?
    fn part_2(&self) -> Result<Solution> {
        let mut complete_boards = HashSet::new();
        let mut last_board = None;
        for &number in self.numbers.iter() {
            // mark each board
            self.mark_boards(number);
            // check if any are complete
            for (i, board) in self.bingo_boards.iter().enumerate() {
                if board.borrow().is_complete() && !complete_boards.contains(&i) {
                    complete_boards.insert(i);
                    let score = board.borrow().score(number);
                    last_board = Some(score);
                }
            }
        }
        match last_board {
            Some(score) => Ok(score.into()),
            None => Err(PuzzleError::NoSolution)?,
        }
    }
}
