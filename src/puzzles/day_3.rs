/*
** src/puzzles/day_3.rs
** https://adventofcode.com/2021/day/3
*/

use crate::types::{Puzzle, Result, Solution};
use crate::utils;

const INPUT: &str = include_str!("../../input/3.txt");
const N_BITS: usize = 12;

#[derive(Clone)]
struct Binary {
    digits: [u8; N_BITS],
}

impl Binary {
    fn bit(&self, i: usize) -> u8 {
        self.digits[N_BITS - i - 1]
    }
}

impl From<&str> for Binary {
    fn from(s: &str) -> Self {
        let mut digits = [0; N_BITS];
        for (i, c) in s.chars().enumerate() {
            digits[i] = c.to_digit(10).unwrap() as u8;
        }
        Self { digits }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for &Binary {
    fn into(self) -> u32 {
        let mut n = 0;
        for (i, &x) in self.digits.iter().rev().enumerate() {
            n |= (x as u32) << i;
        }
        n
    }
}

pub struct Day3 {
    numbers: Vec<Binary>,
    bit_counts: [u64; N_BITS],
}

impl Day3 {
    fn count_bits(numbers: &[Binary]) -> [u64; N_BITS] {
        let mut bit_count = [0; N_BITS];

        for number in numbers.iter() {
            for (i, &bit) in number.digits.iter().enumerate() {
                if bit == 1 {
                    bit_count[i] += 1;
                }
            }
        }

        bit_count
    }

    pub fn new() -> Self {
        let numbers = utils::input_to_lines(INPUT).map(Binary::from).collect::<Vec<_>>();
        let bit_counts = Self::count_bits(&numbers);
        Self {
            numbers,
            bit_counts,
        }
    }

    fn most_common(bit_counts: &[u64; N_BITS], n_numbers: usize, bit: usize) -> u8 {
        let pos = N_BITS - bit - 1;
        if bit_counts[pos] >= n_numbers as u64 / 2 {
            1
        } else {
            0
        }
    }

    fn least_common(bit_counts: &[u64; N_BITS], n_numbers: usize, bit: usize) -> u8 {
        let pos = N_BITS - bit - 1;
        if bit_counts[pos] >= n_numbers as u64 / 2 {
            0
        } else {
            1
        }
    }
}

impl Puzzle for Day3 {
    // Use the binary numbers in your diagnostic report to calculate the gamma
    // rate and epsilon rate, then multiply them together. What is the power
    // consumption of the submarine?
    fn part_1(&self) -> Result<Solution> {
        let mut gamma = 0;
        let mut epsilon = 0;

        for i in 0..N_BITS {
            match Self::most_common(&self.bit_counts, self.numbers.len(), i) {
                0 => epsilon |= 1 << i,
                1 => gamma |= 1 << i,
                _ => unreachable!(),
            };
        }

        Ok((gamma * epsilon).into())
    }

    // Use the binary numbers in your diagnostic report to calculate the oxygen
    // generator rating and CO2 scrubber rating, then multiply them together.
    // What is the life support rating of the submarine?
    fn part_2(&self) -> Result<Solution> {
        // determine oxygen generator rating
        let mut oxygen_numbers = self.numbers.clone();
        for i in (0..N_BITS).rev() {
            let bit_counts = Self::count_bits(&oxygen_numbers);
            let bit = Self::most_common(&bit_counts, oxygen_numbers.len(), i);
            oxygen_numbers = oxygen_numbers
                .iter()
                .filter(|n| n.bit(i) == bit)
                .cloned()
                .collect();
            if oxygen_numbers.len() == 1 {
                break;
            }
        }
        let oxygen_rating: u32 = (&oxygen_numbers[0]).into();

        // determine CO2 scrubber rating
        let mut co2_numbers = self.numbers.clone();
        for i in (0..N_BITS).rev() {
            let bit_counts = Self::count_bits(&co2_numbers);
            let bit = Self::least_common(&bit_counts, co2_numbers.len(), i);
            co2_numbers = co2_numbers
                .iter()
                .filter(|n| n.bit(i) == bit)
                .cloned()
                .collect();
            if co2_numbers.len() == 1 {
                break;
            }
        }
        let co2_rating: u32 = (&co2_numbers[0]).into();

        Ok((oxygen_rating * co2_rating).into())
    }
}
