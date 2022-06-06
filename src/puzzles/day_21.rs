/*
** src/puzzles/day_21.rs
** https://adventofcode.com/2021/day/21
*/

use crate::types::{Puzzle, Result, Solution};

use std::cmp;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Player {
    Player1,
    Player2,
}

impl Player {
    fn next(&self) -> Self {
        match self {
            Self::Player1 => Self::Player2,
            Self::Player2 => Self::Player1,
        }
    }
}

#[derive(Debug)]
struct DeterministicDice {
    counter: u32,
    rolls: u32,
}

impl DeterministicDice {
    fn new() -> Self {
        Self {
            counter: 1,
            rolls: 0,
        }
    }

    fn roll_single(&mut self) -> u32 {
        let output = self.counter;
        // advance the counter, rolling over to 1 at 100
        self.counter += 1;
        if self.counter > 100 {
            self.counter = 1;
        }
        self.rolls += 1;
        output
    }

    fn roll(&mut self) -> u32 {
        // roll the dice 3 times
        self.roll_single() + self.roll_single() + self.roll_single()
    }
}

#[derive(Debug, Clone)]
struct DiracDiceGame {
    p1_pos: u32,
    p2_pos: u32,
    p1_score: u32,
    p2_score: u32,
    win_score: u32,
    curr_player: Player,
}

impl DiracDiceGame {
    fn new(p1_pos: u32, p2_pos: u32, win_score: u32) -> Self {
        Self {
            p1_pos,
            p2_pos,
            p1_score: 0,
            p2_score: 0,
            win_score,
            curr_player: Player::Player1,
        }
    }

    fn losing_player_score(&self) -> u32 {
        cmp::min(self.p1_score, self.p2_score)
    }

    fn check_for_winner(&self) -> Option<Player> {
        if self.p1_score >= self.win_score {
            Some(Player::Player1)
        } else if self.p2_score >= self.win_score {
            Some(Player::Player2)
        } else {
            None
        }
    }

    fn advance_player(&mut self, n: u32) {
        match self.curr_player {
            Player::Player1 => {
                // advance the position, rolling over at 10
                self.p1_pos += n;
                if self.p1_pos > 10 {
                    self.p1_pos %= 10;
                    if self.p1_pos == 0 {
                        self.p1_pos = 10;
                    }
                }
                // add the new position to the score
                self.p1_score += self.p1_pos;
            }
            Player::Player2 => {
                // advance the position, rolling over at 10
                self.p2_pos += n;
                if self.p2_pos > 10 {
                    self.p2_pos %= 10;
                    if self.p2_pos == 0 {
                        self.p2_pos = 10;
                    }
                }
                // add the new position to the score
                self.p2_score += self.p2_pos;
            }
        };
    }

    fn play_round(&mut self, roll: u32) {
        self.advance_player(roll);
        self.curr_player = self.curr_player.next();
    }
}

pub struct Day21 {
    p1_start_pos: u32,
    p2_start_pos: u32,
    dirac_moveset: HashMap<u32, u64>,
}

impl Day21 {
    fn parse_start_position(line: &'static str) -> u32 {
        // just grab the last character in each line
        line.chars().rev().next().unwrap().to_digit(10).unwrap()
    }

    pub fn new(input: &'static str) -> Self {
        let p1_start_line = input.split('\n').next().unwrap();
        let p1_start_pos = Self::parse_start_position(p1_start_line);

        let p2_start_line = input.split('\n').nth(1).unwrap();
        let p2_start_pos = Self::parse_start_position(p2_start_line);

        // generate the moveset for part 2; reduces branching by combining
        // dice roll permutations whose sums are equal
        let mut dirac_moveset = HashMap::new();
        for (i, j, k) in itertools::iproduct!(1..=3, 1..=3, 1..=3) {
            let entry = dirac_moveset.entry(i + j + k).or_insert(0);
            *entry += 1;
        }

        Self {
            p1_start_pos,
            p2_start_pos,
            dirac_moveset,
        }
    }

    fn play_game_deterministic(&self) -> u64 {
        let mut game = DiracDiceGame::new(self.p1_start_pos, self.p2_start_pos, 1000);
        let mut dice = DeterministicDice::new();

        while game.check_for_winner().is_none() {
            game.play_round(dice.roll());
        }

        game.losing_player_score() as u64 * dice.rolls as u64
    }

    fn play_game_dirac_rec(
        &self,
        p1_wins: &mut u64,
        p2_wins: &mut u64,
        mut game: DiracDiceGame,
        roll: u32,
        n_games: u64,
    ) {
        game.play_round(roll);

        // check for a winner; otherwise, recurse
        if let Some(player) = game.check_for_winner() {
            match player {
                Player::Player1 => *p1_wins += n_games,
                Player::Player2 => *p2_wins += n_games,
            };
        } else {
            for (roll, n) in self.dirac_moveset.iter() {
                self.play_game_dirac_rec(p1_wins, p2_wins, game.clone(), *roll, n_games * n);
            }
        }
    }

    fn play_game_dirac(&self) -> u64 {
        let game = DiracDiceGame::new(self.p1_start_pos, self.p2_start_pos, 21);
        let mut p1_wins = 0;
        let mut p2_wins = 0;

        // recurse on each possible die roll
        for (roll, n_games) in self.dirac_moveset.iter() {
            self.play_game_dirac_rec(&mut p1_wins, &mut p2_wins, game.clone(), *roll, *n_games);
        }

        cmp::max(p1_wins, p2_wins)
    }
}

impl Puzzle for Day21 {
    // Play a practice game using the deterministic 100-sided die. The moment
    // either player wins, what do you get if you multiply the score of the
    // losing player by the number of times the die was rolled during the game?
    fn part_1(&self) -> Result<Solution> {
        Ok(self.play_game_deterministic().into())
    }

    // Using your given starting positions, determine every possible outcome.
    // Find the player that wins in more universes; in how many universes does
    // that player win?
    fn part_2(&self) -> Result<Solution> {
        Ok(self.play_game_dirac().into())
    }
}
