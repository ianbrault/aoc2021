/*
** src/puzzles/day_6.rs
** https://adventofcode.com/2021/day/6
*/

use crate::types::{Puzzle, Result, Solution};

use std::cell::RefCell;

const LIFECYCLE: usize = 6;
const INACTIVE_PERIOD: usize = 2;

pub struct Day6 {
    input: &'static str,
    // count the number of fish with each timer to save space/time
    // need RefCell for interior mutability
    fish: RefCell<[u64; LIFECYCLE + INACTIVE_PERIOD + 1]>,
}

impl Day6 {
    pub fn new(input: &'static str) -> Self {
        // empty initialization then call initialize_fish_array
        // marginally more inefficient but cleaner
        let new = Self {
            input,
            fish: RefCell::new([0; LIFECYCLE + INACTIVE_PERIOD + 1]),
        };
        new.initialize_fish_array();
        new
    }

    fn initialize_fish_array(&self) {
        let mut fish = [0; LIFECYCLE + INACTIVE_PERIOD + 1];
        for n in self.input.split(',') {
            fish[n.parse::<usize>().unwrap()] += 1;
        }
        let _ = self.fish.replace(fish);
    }

    fn simulate_day(&self) {
        // double-buffer for updates
        let mut fish_new = [0; LIFECYCLE + INACTIVE_PERIOD + 1];

        for (i, &n_fish) in self.fish.borrow().iter().enumerate() {
            if i == 0 {
                // fish whose timers have expired are reset
                fish_new[LIFECYCLE] += n_fish;
                // create new fish, including the inactive period
                fish_new[LIFECYCLE + INACTIVE_PERIOD] += n_fish;
            } else {
                // decrease the timer for the fish
                fish_new[i - 1] += n_fish;
            }
        }

        let _ = self.fish.replace(fish_new);
    }
}

impl Puzzle for Day6 {
    // How many lanternfish would there be after 80 days?
    fn part_1(&self) -> Result<Solution> {
        let days = 80;
        for _ in 0..days {
            self.simulate_day();
        }

        Ok(self.fish.borrow().iter().sum::<u64>().into())
    }

    // How many lanternfish would there be after 256 days?
    fn part_2(&self) -> Result<Solution> {
        // note: re-initialize the fish array
        self.initialize_fish_array();

        let days = 256;
        for _ in 0..days {
            self.simulate_day();
        }

        Ok(self.fish.borrow().iter().sum::<u64>().into())
    }
}
