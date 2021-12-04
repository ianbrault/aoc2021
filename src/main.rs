/*
** src/main.rs
*/

#[macro_use]
mod utils;

mod puzzles;
mod types;

use std::env;

enum Day {
    Which(usize),
    All,
}

fn parse_args() -> Day {
    match env::args().nth(1) {
        Some(n) => Day::Which(n.parse().unwrap()),
        None => Day::All,
    }
}

fn main() {
    // determine which puzzle to run
    let which_puzzle = parse_args();
    let puzzles = match which_puzzle {
        Day::Which(n) => vec![puzzles::all().into_iter().nth(n - 1).unwrap()],
        Day::All => puzzles::all(),
    };

    for (day, puzzle) in puzzles.into_iter().enumerate() {
        let day = match which_puzzle {
            Day::Which(n) => n,
            Day::All => day + 1,
        };
        // part 1
        match puzzle.part_1() {
            Ok(solution) => println!("day {:02} part 1: {}", day, solution),
            Err(err) => println!("day {:02} part 1: {}", day, err),
        };
        // part 2
        match puzzle.part_2() {
            Ok(solution) => println!("day {:02} part 2: {}", day, solution),
            Err(err) => println!("day {:02} part 1: {}", day, err),
        };
    }
}
