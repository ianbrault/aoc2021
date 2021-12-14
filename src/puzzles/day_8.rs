/*
** src/puzzles/day_8.rs
** https://adventofcode.com/2021/day/8
*/

use crate::types::{Puzzle, Result, Solution};
use crate::utils;

use std::collections::HashMap;
use std::convert::TryInto;

const INPUT: &str = include_str!("../../input/8.txt");

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Segment {
    fn from(c: char) -> Self {
        match c {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => panic!("invalid character: {}", c),
        }
    }
}

#[derive(Debug, PartialEq)]
struct SevenSegment {
    segment_inner: [Option<Segment>; 7],
}

impl SevenSegment {
    fn len(&self) -> usize {
        self.segment_inner.iter().filter(|s| s.is_some()).count()
    }

    fn segments<const N: usize>(&self) -> [Segment; N] {
        self.segment_inner
            .iter()
            .take(N)
            .filter_map(|seg| *seg)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn segments_solved<const N: usize>(
        &self,
        solution: &HashMap<Segment, Segment>,
    ) -> [Segment; N] {
        let mut segments = self
            .segment_inner
            .iter()
            .take(N)
            .filter_map(|seg| *seg)
            .map(|seg| *solution.get(&seg).unwrap())
            .collect::<Vec<_>>();
        segments.sort();
        segments.try_into().unwrap()
    }

    fn contains_segment(&self, segment: Segment) -> bool {
        for seg in self.segment_inner.iter() {
            match seg {
                Some(x) if *x == segment => return true,
                _ => (),
            }
        }
        false
    }

    fn contains_segments(&self, segments: &[Segment]) -> bool {
        segments.iter().all(|&seg| self.contains_segment(seg))
    }

    fn contains_one_of(&self, segments: [Segment; 2]) -> bool {
        let contains_0 = self.contains_segment(segments[0]);
        let contains_1 = self.contains_segment(segments[1]);
        (contains_0 || contains_1) && !(contains_0 && contains_1)
    }

    fn select_segment<P>(&self, predicate: P) -> Segment
    where
        P: Fn(&Segment) -> bool,
    {
        self.segment_inner
            .iter()
            .filter_map(|seg| *seg)
            .find(predicate)
            .unwrap()
    }

    fn select_segments<P>(&self, predicate: P) -> Vec<Segment>
    where
        P: Fn(&Segment) -> bool,
    {
        self.segment_inner
            .iter()
            .filter_map(|seg| *seg)
            .filter(predicate)
            .collect()
    }

    fn solve_with(&self, solution: &HashMap<Segment, Segment>) -> u32 {
        match self.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => match self.segments_solved::<5>(&solution) {
                [Segment::A, Segment::C, Segment::D, Segment::E, Segment::G] => 2,
                [Segment::A, Segment::C, Segment::D, Segment::F, Segment::G] => 3,
                [Segment::A, Segment::B, Segment::D, Segment::F, Segment::G] => 5,
                _ => {
                    unreachable!();
                }
            },
            6 => match self.segments_solved::<6>(&solution) {
                [Segment::A, Segment::B, Segment::C, Segment::E, Segment::F, Segment::G] => 0,
                [Segment::A, Segment::B, Segment::D, Segment::E, Segment::F, Segment::G] => 6,
                [Segment::A, Segment::B, Segment::C, Segment::D, Segment::F, Segment::G] => 9,
                _ => {
                    unreachable!();
                }
            },
            7 => 8,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for SevenSegment {
    fn from(s: &str) -> Self {
        let mut segment_inner = [None; 7];

        for (i, c) in s.chars().enumerate() {
            segment_inner[i] = Some(Segment::from(c));
        }

        Self { segment_inner }
    }
}

struct Entry {
    signals: [SevenSegment; 10],
    output: [SevenSegment; 4],
}

impl Entry {
    fn new(signals: [SevenSegment; 10], output: [SevenSegment; 4]) -> Self {
        Self { signals, output }
    }

    fn select_signal(&self, n_segments: usize) -> &SevenSegment {
        self.signals.iter().find(|s| s.len() == n_segments).unwrap()
    }

    fn select_signal_with<P>(&self, n_segments: usize, predicate: P) -> &SevenSegment
    where
        P: Fn(&SevenSegment) -> bool,
    {
        self.signals
            .iter()
            .filter(|s| s.len() == n_segments)
            .find(|&s| predicate(s))
            .unwrap()
    }

    #[allow(clippy::many_single_char_names)]
    fn solve_segments(&self) -> HashMap<Segment, Segment> {
        let mut solution = HashMap::new();

        // select the 2-segment signal
        // this is the number 1
        let sig2 = self.select_signal(2);
        // these 2 segments are C/F options
        let c_f_opts = sig2.segments::<2>();

        // select the 3-segment signal
        // this is the number 7
        let sig3 = self.select_signal(3);
        // A is the segment which is not a C/F option
        let a = sig3.select_segment(|s| !c_f_opts.contains(s));
        solution.insert(a, Segment::A);

        // select a 5-segment signal which includes all the same segments as the 3-segment
        // this is the number 3
        let sig3_segs = sig3.segments::<3>();
        let sig5_3 = self.select_signal_with(5, |sig| sig.contains_segments(&sig3_segs));
        // D and G options are the segments which are not A/C/F
        let d_g_opts: [Segment; 2] = sig5_3
            .select_segments(|s| !c_f_opts.contains(s) && *s != a)
            .try_into()
            .unwrap();

        // select a 5-segment signal which contains A/D/G and one of the C/F options, but NOT both
        // this is either the number 2 or 5
        let a_d_g_segs = [a, d_g_opts[0], d_g_opts[1]];
        let sig5_2_5_a = self.select_signal_with(5, |sig| {
            sig.contains_segments(&a_d_g_segs) && sig.contains_one_of(c_f_opts)
        });
        // one of the B/E options is the odd segment out
        let b_e_opt_1 =
            sig5_2_5_a.select_segment(|s| !a_d_g_segs.contains(s) && !c_f_opts.contains(s));

        // select the other 5-segment signal which contains A/D/G and one of the C/F options
        // this is the other number 2 or 5
        #[allow(clippy::suspicious_operation_groupings)]
        let sig5_2_5_b = self.select_signal_with(5, |sig| {
            sig.contains_segments(&a_d_g_segs) && sig.contains_one_of(c_f_opts) && sig != sig5_2_5_a
        });
        // the the other B/E option is the odd segment out
        let b_e_opt_2 =
            sig5_2_5_b.select_segment(|s| !a_d_g_segs.contains(s) && !c_f_opts.contains(s));
        let b_e_opts = [b_e_opt_1, b_e_opt_2];

        // select the 4-segment signal
        // this is the number 4
        let sig4 = self.select_signal(4);
        // the B/E option which is not contained in the 4-segment is E
        let e = if sig4.contains_segment(b_e_opts[0]) {
            b_e_opts[1]
        } else {
            b_e_opts[0]
        };
        solution.insert(e, Segment::E);
        // the other B/E option is B
        let b = utils::other(b_e_opts, e);
        solution.insert(b, Segment::B);
        // use this new info to determine D from the 4-segment
        let d = sig4.select_segment(|s| !c_f_opts.contains(s) && *s != b);
        solution.insert(d, Segment::D);
        // the other D/G option is G
        let g = utils::other(d_g_opts, d);
        solution.insert(g, Segment::G);

        // select the 6-segment signal with both D/G options and only one of the C/F options
        // this is the number 6
        let sig6 = self.select_signal_with(6, |sig| {
            sig.contains_segments(&d_g_opts) && sig.contains_one_of(c_f_opts)
        });
        // the unknown segment is F
        let f = sig6.select_segment(|s| !solution.contains_key(s));
        solution.insert(f, Segment::F);
        // the other C/F option is C
        let c = utils::other(c_f_opts, f);
        solution.insert(c, Segment::C);

        solution
    }

    fn output_value(&self, solution: &HashMap<Segment, Segment>) -> u32 {
        self.output
            .iter()
            .rev()
            .enumerate()
            .map(|(i, signal)| signal.solve_with(solution) * 10u32.pow(i as u32))
            .sum()
    }
}

impl From<&str> for Entry {
    fn from(s: &str) -> Self {
        match split!(s, " | ") {
            [signals_str, output_str] => {
                let signals = signals_str
                    .split(' ')
                    .map(SevenSegment::from)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                let output = output_str
                    .split(' ')
                    .map(SevenSegment::from)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                Self::new(signals, output)
            }
            _ => unreachable!(),
        }
    }
}

pub struct Day8 {
    entries: Vec<Entry>,
}

impl Day8 {
    pub fn new() -> Self {
        let mut entries = Vec::new();
        for line in utils::input_to_lines(INPUT) {
            entries.push(Entry::from(line));
        }
        Self { entries }
    }
}

impl Puzzle for Day8 {
    // In the output values, how many times do digits 1, 4, 7, or 8 appear?
    fn part_1(&self) -> Result<Solution> {
        let mut count = 0;

        for entry in self.entries.iter() {
            // 1 = 2 segments; 4 = 4 segments; 7 = 3 segments; 8 = 7 segments
            for signal in entry.output.iter() {
                let len = signal.len();
                if len == 2 || len == 3 || len == 4 || len == 7 {
                    count += 1;
                }
            }
        }

        Ok(count.into())
    }

    // For each entry, determine all of the wire/segment connections and decode
    // the four-digit output values. What do you get if you add up all of the
    // output values?
    fn part_2(&self) -> Result<Solution> {
        let mut sum = 0;

        for entry in self.entries.iter() {
            let solution = entry.solve_segments();
            sum += entry.output_value(&solution);
        }

        Ok(sum.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let entry_string = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let entry = Entry::from(entry_string);

        let mut exp = HashMap::new();
        exp.insert(Segment::A, Segment::E);
        exp.insert(Segment::B, Segment::C);
        exp.insert(Segment::C, Segment::D);
        exp.insert(Segment::D, Segment::A);
        exp.insert(Segment::E, Segment::F);
        exp.insert(Segment::F, Segment::G);
        exp.insert(Segment::G, Segment::B);

        let sol = entry.solve_segments();
        assert_eq!(sol, exp);
    }

    #[test]
    fn test_solve_sum() {
        let entry_string =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let entry = Entry::from(entry_string);

        let sol = entry.solve_segments();
        assert_eq!(entry.output_value(&sol), 5353);
    }
}
