/*
** src/puzzles/day_2.rs
** https://adventofcode.com/2021/day/2
*/

use crate::types::{Puzzle, Result, Solution};
use crate::utils;

enum Direction {
    Forward,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "forward" => Self::Forward,
            "up" => Self::Up,
            "down" => Self::Down,
            _ => panic!("invalid direction: {}", s),
        }
    }
}

struct Command {
    direction: Direction,
    unit: u64,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match split!(s, ' ') {
            [dir_str, unit_str] => {
                let direction = Direction::from(*dir_str);
                let unit = unit_str.parse().unwrap();
                Self { direction, unit }
            }
            _ => panic!("invalid command: {}", s),
        }
    }
}

pub struct Navigator {
    position: i64,
    depth: i64,
    aim: i64,
}

impl Navigator {
    fn new() -> Self {
        Self {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn handle_command(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => self.position += command.unit as i64,
            Direction::Up => self.depth -= command.unit as i64,
            Direction::Down => self.depth += command.unit as i64,
        }
    }

    fn handle_command_with_aim(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => {
                self.position += command.unit as i64;
                self.depth += self.aim * command.unit as i64;
            }
            Direction::Up => self.aim -= command.unit as i64,
            Direction::Down => self.aim += command.unit as i64,
        }
    }
}

pub struct Day2 {
    commands: Vec<Command>,
}

impl Day2 {
    pub fn new(input: &'static str) -> Self {
        let commands = utils::input_to_lines(input).map(Command::from).collect();

        Self { commands }
    }
}

impl Puzzle for Day2 {
    // What do you get if you multiply your final horizontal position by your
    // final depth?
    fn part_1(&self) -> Result<Solution> {
        let mut navigator = Navigator::new();

        for command in self.commands.iter() {
            navigator.handle_command(command);
        }

        Ok((navigator.position * navigator.depth).into())
    }

    // Using this new interpretation of the commands, calculate the horizontal
    // position and depth you would have after following the planned course.
    // What do you get if you multiply your final horizontal position by your
    // final depth?
    fn part_2(&self) -> Result<Solution> {
        let mut navigator = Navigator::new();

        for command in self.commands.iter() {
            navigator.handle_command_with_aim(command);
        }

        Ok((navigator.position * navigator.depth).into())
    }
}
