/*
** src/main.rs
*/

mod puzzles;
mod types;
mod utils;

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
    let puzzles = match parse_args() {
        Day::Which(n) => vec![puzzles::all().into_iter().nth(n - 1).unwrap()],
        Day::All => puzzles::all(),
    };

    for (day, puzzle) in puzzles.into_iter().enumerate() {
        // part 1
        match puzzle.part_1() {
            Ok(solution) => println!("day {:02} part 1: {}", day + 1, solution),
            Err(err) => println!("day {:02} part 1: {}", day + 1, err),
        };
        // part 2
        match puzzle.part_2() {
            Ok(solution) => println!("day {:02} part 2: {}", day + 1, solution),
            Err(err) => println!("day {:02} part 1: {}", day + 1, err),
        };
    }
}
