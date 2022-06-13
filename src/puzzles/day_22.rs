/*
** src/puzzles/day_22.rs
** https://adventofcode.com/2021/day/22
*/

use crate::types::{Puzzle, Result, Solution};
use crate::utils;

use itertools::Itertools;

use std::cmp;
use std::collections::HashSet;
use std::ops::RangeInclusive;

type Cube = (i64, i64, i64);

#[derive(Debug)]
enum Instruction {
    On,
    Off,
}

impl From<&'static str> for Instruction {
    fn from(s: &'static str) -> Self {
        match s {
            "on" => Self::On,
            "off" => Self::Off,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn size(&self) -> i64 {
        self.max - self.min
    }

    fn iter(&self) -> RangeInclusive<i64> {
        self.min..=self.max
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let max_of_mins = cmp::max(self.min, other.min);
        let min_of_maxs = cmp::min(self.max, other.max);

        if max_of_mins > min_of_maxs {
            None
        } else {
            Some(Self {
                min: max_of_mins,
                max: min_of_maxs,
            })
        }
    }

    fn fully_contains(&self, other: &Self) -> bool {
        other.min >= self.min && other.max <= self.max
    }
}

impl From<RangeInclusive<i64>> for Range {
    fn from(range: RangeInclusive<i64>) -> Self {
        let (min, max) = range.into_inner();
        Self { min, max }
    }
}

#[derive(Debug)]
struct Region {
    x: Range,
    y: Range,
    z: Range,
}

impl Region {
    fn new(x: RangeInclusive<i64>, y: RangeInclusive<i64>, z: RangeInclusive<i64>) -> Self {
        Self {
            x: Range::from(x),
            y: Range::from(y),
            z: Range::from(z),
        }
    }

    fn size(&self) -> i64 {
        self.x.size() * self.y.size() * self.z.size()
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if let Some(x) = self.x.intersection(&other.x) {
            if let Some(y) = self.y.intersection(&other.y) {
                if let Some(z) = self.z.intersection(&other.z) {
                    return Some(Self { x, y, z });
                }
            }
        }
        None
    }

    fn fully_contains(&self, other: &Self) -> bool {
        self.x.fully_contains(&other.x)
            && self.y.fully_contains(&other.y)
            && self.z.fully_contains(&other.z)
    }
}

struct PoweredRegion<'a> {
    region: &'a Region,
    deductions: Vec<Region>,
}

impl<'a> PoweredRegion<'a> {
    fn prune_overlaps(regions: Vec<Region>) -> Vec<Region> {
        let fully_contained_regions = regions
            .iter()
            .enumerate()
            .tuple_combinations()
            .filter_map(|((a, region_a), (b, region_b))| {
                if region_a.fully_contains(region_b) {
                    Some(b)
                } else if region_b.fully_contains(region_a) {
                    Some(a)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        regions
            .into_iter()
            .enumerate()
            .filter_map(|(i, region)| {
                if fully_contained_regions.contains(&i) {
                    None
                } else {
                    Some(region)
                }
            })
            .collect()
    }

    fn overlaps(regions: &[Region]) -> Vec<Region> {
        let overlaps = regions
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a.intersection(b))
            .flatten()
            .collect();

        // remove overlaps which are fully subsets of other overlaps
        Self::prune_overlaps(overlaps)
    }

    fn deduction_overlap_size(regions: &[Region]) -> i64 {
        if regions.is_empty() {
            0
        } else if regions.len() == 1 {
            regions[0].size()
        } else {
            // find the overlaps of the regions
            let overlaps = Self::overlaps(regions);
            // calculate the size of the region
            let size = overlaps.iter().map(|region| region.size()).sum::<i64>();
            // then exclude the overlaps of the overlaps
            let overlap_size = Self::deduction_overlap_size(&overlaps);

            size - overlap_size
        }
    }

    fn size(&self) -> i64 {
        // start with the total size of the region
        let full_size = self.region.size();
        // subtract the sizes of the deductions
        let deduction_size = self.deductions.iter().map(Region::size).sum::<i64>();
        // but then account for overlaps in the deductions
        let deduction_overlaps = Self::deduction_overlap_size(&self.deductions);

        full_size - deduction_size + deduction_overlaps
    }

    fn deduct(&mut self, region: &Region) {
        if let Some(overlap) = self.region.intersection(region) {
            self.deductions.push(overlap);
        }
    }
}

impl<'a> From<&'a Region> for PoweredRegion<'a> {
    fn from(region: &'a Region) -> Self {
        Self {
            region,
            deductions: vec![],
        }
    }
}

#[derive(Debug)]
struct Step {
    instr: Instruction,
    region: Region,
}

impl Step {
    fn parse_range(s: &'static str) -> (i64, i64) {
        split_into!(&s[2..s.len()], "..", min_str, max_str);
        let min = min_str.parse().unwrap();
        let max = max_str.parse().unwrap();
        (min, max)
    }
}

impl From<&'static str> for Step {
    fn from(s: &'static str) -> Self {
        split_into!(s, ' ', instr_str, ranges_str);
        split_into!(ranges_str, ',', x_str, y_str, z_str);

        let instr = Instruction::from(instr_str);
        let (x_min, x_max) = Self::parse_range(x_str);
        let (y_min, y_max) = Self::parse_range(y_str);
        let (z_min, z_max) = Self::parse_range(z_str);

        Self {
            instr,
            region: Region::new(x_min..=x_max, y_min..=y_max, z_min..=z_max),
        }
    }
}

pub struct Day22 {
    procedure: Vec<Step>,
}

impl Day22 {
    pub fn new(input: &'static str) -> Self {
        let procedure = utils::input_to_lines(input).map(Step::from).collect();
        Self { procedure }
    }

    fn power_on_cubes_with_boundary(cubes: &mut HashSet<Cube>, region: &Region, boundary: &Region) {
        if let Some(overlap) = region.intersection(boundary) {
            for x in overlap.x.iter() {
                for y in overlap.y.iter() {
                    for z in overlap.z.iter() {
                        cubes.insert((x, y, z));
                    }
                }
            }
        }
    }

    fn power_off_cubes_with_boundary(
        cubes: &mut HashSet<Cube>,
        region: &Region,
        boundary: &Region,
    ) {
        if let Some(overlap) = region.intersection(boundary) {
            for x in overlap.x.iter() {
                for y in overlap.y.iter() {
                    for z in overlap.z.iter() {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    fn execute_procedure_with_boundary(&self, boundary: Region) -> usize {
        let mut cubes = HashSet::new();

        for step in self.procedure.iter() {
            match step.instr {
                Instruction::On => {
                    Self::power_on_cubes_with_boundary(&mut cubes, &step.region, &boundary)
                }
                Instruction::Off => {
                    Self::power_off_cubes_with_boundary(&mut cubes, &step.region, &boundary)
                }
            }
        }

        cubes.len()
    }

    fn execute_procedure(&self) -> i64 {
        // first add all powered on cubes
        let mut powered_regions = self
            .procedure
            .iter()
            .filter(|step| matches!(step.instr, Instruction::On))
            .map(|step| PoweredRegion::from(&step.region))
            .collect::<Vec<_>>();

        // now deduct the powered off regions from the powered on cubes
        for step in self
            .procedure
            .iter()
            .filter(|step| matches!(step.instr, Instruction::Off))
        {
            for powered_region in powered_regions.iter_mut() {
                powered_region.deduct(&step.region);
            }
        }

        // now sum the sizes of the remaining powered on regions
        powered_regions
            .iter()
            .map(|region| region.size())
            .sum::<i64>()
    }
}

impl Puzzle for Day22 {
    // Execute the reboot steps. Afterward, considering only cubes in the region
    // x=-50..50,y=-50..50,z=-50..50, how many cubes are on?
    fn part_1(&self) -> Result<Solution> {
        let boundary = Region::new(-50..=50, -50..=50, -50..=50);
        let n_cubes = self.execute_procedure_with_boundary(boundary);
        Ok(n_cubes.into())
    }

    // Starting again with all cubes off, execute all reboot steps. Afterward,
    // considering all cubes, how many cubes are on?
    fn part_2(&self) -> Result<Solution> {
        let n_cubes = self.execute_procedure();
        Ok(n_cubes.into())
    }
}
