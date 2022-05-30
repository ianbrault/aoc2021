/*
** src/puzzles/day_19.rs
** https://adventofcode.com/2021/day/19
*/

use crate::types::{Puzzle, PuzzleError, Result, Solution};

use itertools::Itertools;
use nalgebra::{Rotation3, Vector3};

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Identity,
    RotateX,
    RotateY,
    RotateZ,
    RotateXY,
    RotateXZ,
    RotateYZ,
    RotateXYZ,
}

impl Rotation {
    fn has_x_rotation(&self) -> bool {
        matches!(
            self,
            Self::RotateX | Self::RotateXY | Self::RotateXZ | Self::RotateXYZ
        )
    }

    fn has_y_rotation(&self) -> bool {
        matches!(
            self,
            Self::RotateY | Self::RotateXY | Self::RotateYZ | Self::RotateXYZ
        )
    }

    fn has_z_rotation(&self) -> bool {
        matches!(
            self,
            Self::RotateZ | Self::RotateXZ | Self::RotateYZ | Self::RotateXYZ
        )
    }

    fn rotate_inner(&self, v: &Vector3<i64>, angle: f64) -> Vector3<i64> {
        // note: need to convert to floating point first
        let mut u = v.map(|n| n as f64);

        if self.has_x_rotation() {
            let rot = Rotation3::from_axis_angle(&Vector3::x_axis(), angle);
            u = rot * u;
        }
        if self.has_y_rotation() {
            let rot = Rotation3::from_axis_angle(&Vector3::y_axis(), angle);
            u = rot * u;
        }
        if self.has_z_rotation() {
            let rot = Rotation3::from_axis_angle(&Vector3::z_axis(), angle);
            u = rot * u;
        }

        // convert back to integers
        u.map(|n| n as i64)
    }

    fn rotate(&self, v: &Vector3<i64>) -> Vector3<i64> {
        let angle = std::f64::consts::FRAC_PI_2;
        self.rotate_inner(v, angle)
    }

    fn unrotate(&self, v: &Vector3<i64>) -> Vector3<i64> {
        let angle = std::f64::consts::PI + std::f64::consts::FRAC_PI_2;
        self.rotate_inner(v, angle)
    }
}

const ROTATIONS: [Rotation; 8] = [
    Rotation::Identity,
    Rotation::RotateX,
    Rotation::RotateY,
    Rotation::RotateZ,
    Rotation::RotateXY,
    Rotation::RotateXZ,
    Rotation::RotateYZ,
    Rotation::RotateXYZ,
];

#[derive(Debug, Clone, Copy)]
enum Reflection {
    Identity,
    ReflectX,
    ReflectY,
    ReflectZ,
    ReflectXY,
    ReflectXZ,
    ReflectYZ,
    ReflectXYZ,
}

impl Reflection {
    fn combine(a: Self, b: Self) -> Self {
        let a: u8 = a.into();
        let b: u8 = b.into();
        Self::from(a | b)
    }

    fn from_parameters(x: bool, y: bool, z: bool) -> Self {
        let mut reflection = Self::Identity;

        if x {
            reflection = Self::combine(reflection, Self::ReflectX);
        }
        if y {
            reflection = Self::combine(reflection, Self::ReflectY);
        }
        if z {
            reflection = Self::combine(reflection, Self::ReflectZ);
        }

        reflection
    }

    fn solve_for_reflection(a: &Vector3<i64>, b: &Vector3<i64>) -> Option<Self> {
        // compare the absolute values, they must be equal
        let a_abs = a.map(|n| n.abs());
        let b_abs = b.map(|n| n.abs());

        if a_abs == b_abs {
            let reflect_x = a.x != b.x;
            let reflect_y = a.y != b.y;
            let reflect_z = a.z != b.z;
            Some(Self::from_parameters(reflect_x, reflect_y, reflect_z))
        } else {
            None
        }
    }

    fn has_x_reflection(&self) -> bool {
        matches!(
            self,
            Self::ReflectX | Self::ReflectXY | Self::ReflectXZ | Self::ReflectXYZ
        )
    }

    fn has_y_reflection(&self) -> bool {
        matches!(
            self,
            Self::ReflectY | Self::ReflectXY | Self::ReflectYZ | Self::ReflectXYZ
        )
    }

    fn has_z_reflection(&self) -> bool {
        matches!(
            self,
            Self::ReflectZ | Self::ReflectXZ | Self::ReflectYZ | Self::ReflectXYZ
        )
    }

    fn reflect(&self, v: &Vector3<i64>) -> Vector3<i64> {
        let mut x = v.x;
        let mut y = v.y;
        let mut z = v.z;

        if self.has_x_reflection() {
            x = -x;
        }
        if self.has_y_reflection() {
            y = -y;
        }
        if self.has_z_reflection() {
            z = -z;
        }

        Vector3::new(x, y, z)
    }
}

impl Into<u8> for Reflection {
    fn into(self) -> u8 {
        match self {
            Self::Identity => 0x0,
            Self::ReflectX => 0x1,
            Self::ReflectY => 0x2,
            Self::ReflectZ => 0x4,
            Self::ReflectXY => 0x3,
            Self::ReflectXZ => 0x5,
            Self::ReflectYZ => 0x6,
            Self::ReflectXYZ => 0x7,
        }
    }
}

impl From<u8> for Reflection {
    fn from(n: u8) -> Self {
        match n {
            0x0 => Self::Identity,
            0x1 => Self::ReflectX,
            0x2 => Self::ReflectY,
            0x4 => Self::ReflectZ,
            0x3 => Self::ReflectXY,
            0x5 => Self::ReflectXZ,
            0x6 => Self::ReflectYZ,
            0x7 => Self::ReflectXYZ,
            _ => unreachable!(),
        }
    }
}

pub struct Day19 {
    scanner_reports: Vec<Vec<Vector3<i64>>>,
    // note: need RefCell for interior mutability
    scanner_positions: RefCell<HashMap<usize, Vector3<i64>>>,
    scanner_rotations: RefCell<HashMap<usize, Rotation>>,
    scanner_reflections: RefCell<HashMap<usize, Reflection>>,
}

impl Day19 {
    fn parse_vector(input: &'static str) -> Vector3<i64> {
        Vector3::from_iterator(input.split(',').map(|n| n.parse().unwrap()))
    }

    pub fn new(input: &'static str) -> Self {
        let scanner_reports = input
            .split("\n\n")
            .map(|scanner| {
                scanner
                    .split('\n')
                    .skip(1)
                    .map(Self::parse_vector)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            scanner_reports,
            scanner_positions: RefCell::new(HashMap::new()),
            scanner_rotations: RefCell::new(HashMap::new()),
            scanner_reflections: RefCell::new(HashMap::new()),
        }
    }

    fn square_distance(va: &Vector3<i64>, vb: &Vector3<i64>) -> i64 {
        va.zip_map(vb, |x, y| (x - y).pow(2)).fold(0, |x, y| x + y)
    }

    fn manhattan_distance(va: &Vector3<i64>, vb: &Vector3<i64>) -> i64 {
        va.zip_map(vb, |x, y| (x - y).abs()).fold(0, |x, y| x + y)
    }

    fn square_distances(set: &[Vector3<i64>]) -> HashSet<i64> {
        set.iter()
            .tuple_combinations()
            .map(|(ba, bb)| Self::square_distance(ba, bb))
            .collect()
    }

    fn set_scanner_position(&self, scanner: usize, position: Vector3<i64>) {
        self.scanner_positions
            .borrow_mut()
            .insert(scanner, position);
    }

    fn set_scanner_rotation(&self, scanner: usize, rotation: Rotation) {
        self.scanner_rotations
            .borrow_mut()
            .insert(scanner, rotation);
    }

    fn set_scanner_reflection(&self, scanner: usize, reflection: Reflection) {
        self.scanner_reflections
            .borrow_mut()
            .insert(scanner, reflection);
    }

    fn find_matching_beacon_pairs(
        &self,
        scanner_a: usize,
        scanner_b: usize,
    ) -> Result<(
        (&Vector3<i64>, &Vector3<i64>),
        (&Vector3<i64>, &Vector3<i64>),
    )> {
        // find a pair of beacons in each scanner with matching distances
        let mut beacon_a1 = None;
        let mut beacon_a2 = None;
        let mut beacon_b1 = None;
        let mut beacon_b2 = None;

        'outer: for (ba1, ba2) in self.scanner_reports[scanner_a].iter().tuple_combinations() {
            let dist_a = Self::square_distance(ba1, ba2);
            for (bb1, bb2) in self.scanner_reports[scanner_b].iter().tuple_combinations() {
                let dist_b = Self::square_distance(bb1, bb2);
                if dist_a == dist_b {
                    beacon_a1 = Some(ba1);
                    beacon_a2 = Some(ba2);
                    beacon_b1 = Some(bb1);
                    beacon_b2 = Some(bb2);
                    break 'outer;
                }
            }
        }

        let beacon_a1 = beacon_a1.ok_or(PuzzleError::NoSolution)?;
        let beacon_a2 = beacon_a2.ok_or(PuzzleError::NoSolution)?;
        let beacon_b1 = beacon_b1.ok_or(PuzzleError::NoSolution)?;
        let beacon_b2 = beacon_b2.ok_or(PuzzleError::NoSolution)?;
        Ok(((beacon_a1, beacon_a2), (beacon_b1, beacon_b2)))
    }

    fn solve_scanners_for_rotation(
        beacon_a1: &Vector3<i64>,
        beacon_a2: &Vector3<i64>,
        beacon_b1: &Vector3<i64>,
        beacon_b2: &Vector3<i64>,
    ) -> Result<Option<(Vector3<i64>, Reflection)>> {
        // check if the rotation is correct but a reflection is needed
        let a = beacon_a2 - beacon_a1;
        let b = beacon_b2 - beacon_b1;
        if let Some(reflection) = Reflection::solve_for_reflection(&a, &b) {
            // apply the reflection to the B beacons
            let beacon_b1 = reflection.reflect(beacon_b1);
            let beacon_b2 = reflection.reflect(beacon_b2);

            // compare differences between the points, accounting for different endpoints
            let loc_a = beacon_a1 - beacon_b1;
            let loc_b = beacon_a2 - beacon_b2;
            let loc_c = beacon_a1 - beacon_b2;
            let loc_d = beacon_a2 - beacon_b1;
            if loc_a == loc_b {
                Ok(Some((loc_a, reflection)))
            } else if loc_c == loc_d {
                Ok(Some((loc_c, reflection)))
            } else {
                // nothing found, incorrect rotation
                // NOTE: this should probably never be hit...
                Ok(None)
            }
        } else {
            // incorrect rotation
            Ok(None)
        }
    }

    fn solve_scanners(&self, scanner_a: usize, scanner_b: usize) -> Result<()> {
        // figure out which scanner has already been solved
        // sa is solved, sb is unknown
        let (sa, sb) = if self.scanner_positions.borrow().contains_key(&scanner_a) {
            (scanner_a, scanner_b)
        } else {
            (scanner_b, scanner_a)
        };
        println!("DEBUG: solving scanner {} using scanner {}", sb, sa);
        // grab the position/rotation/reflection of the already-solved scanner
        let a_pos = self.scanner_positions.borrow()[&sa];
        let a_rot = self.scanner_rotations.borrow()[&sa];
        let a_rfl = self.scanner_reflections.borrow()[&sa];
        println!("DEBUG: scanner {} position is {:?}", sa, a_pos);
        println!("DEBUG: scanner {} rotation is {:?}", sa, a_rot);
        println!("DEBUG: scanner {} reflection is {:?}", sa, a_rfl);

        // find a pair of beacons in each scanner with matching distances
        let ((ba1, ba2), (bb1, bb2)) = self.find_matching_beacon_pairs(sa, sb)?;

        // try all rotations to find a working orientation
        for rot in ROTATIONS.iter() {
            println!("DEBUG: using rotation {:?}", rot);
            let rot_bb1 = rot.rotate(bb1);
            let rot_bb2 = rot.rotate(bb2);
            if let Some((pos, rfl)) =
                Self::solve_scanners_for_rotation(ba1, ba2, &rot_bb1, &rot_bb2)?
            {
                println!(
                    "DEBUG: initial solve at {:?} using rotation {:?} and reflection {:?}",
                    pos, rot, rfl
                );
                // apply the position/rotation/reflection from the previously-solved scanner
                // NOTE: need to UN-rotate relative to the solved scanner
                let pos = a_pos + a_rfl.reflect(&a_rot.unrotate(&pos));
                println!(
                    "DEBUG: solved scanner {} at {:?} with rotation {:?} and reflection {:?}",
                    sb, pos, rot, rfl
                );
                self.set_scanner_position(sb, pos);
                // TODO: might need to do some composition of rotations
                self.set_scanner_rotation(sb, *rot);
                self.set_scanner_reflection(sb, rfl);
                break;
            }
        }

        Ok(())
    }

    fn combine_beacons(&self) -> HashSet<Vector3<i64>> {
        let mut beacons = HashSet::new();

        for (i, scanner_beacons) in self.scanner_reports.iter().enumerate() {
            let pos = self.scanner_positions.borrow()[&i];
            let rot = self.scanner_rotations.borrow()[&i];
            let rfl = self.scanner_reflections.borrow()[&i];

            for beacon in scanner_beacons.iter() {
                let b_pos = beacon + rfl.reflect(&rot.unrotate(&pos));
                beacons.insert(b_pos);
            }
        }

        beacons
    }
}

impl Puzzle for Day19 {
    // Assemble the full map of beacons. How many beacons are there?
    fn part_1(&self) -> Result<Solution> {
        // get the squared distances between all beacons for each scanner report
        let square_distances = self
            .scanner_reports
            .iter()
            .map(|scanner| Self::square_distances(scanner.as_slice()))
            .collect::<Vec<_>>();

        // find scanners that can see the same beacons
        // treat the beacons as a complete graph so need n * (n - 1) / 2 overlaps
        // for n=12 this is 66
        let n_common = 66;
        let overlaps = square_distances
            .iter()
            .enumerate()
            .tuple_combinations()
            .filter(|((_, dists_a), (_, dists_b))| {
                dists_a.intersection(dists_b).count() >= n_common
            })
            .map(|((i, _), (j, _))| (i, j))
            .collect::<Vec<_>>();

        // use the first scanner as the base reference
        self.set_scanner_position(0, Vector3::from_element(0));
        self.set_scanner_rotation(0, Rotation::Identity);
        self.set_scanner_reflection(0, Reflection::Identity);

        // solve remaining scanners
        for (scanner_a, scanner_b) in overlaps.iter() {
            self.solve_scanners(*scanner_a, *scanner_b)?;
        }

        // combine the beacons using the scanner solutions
        let beacons = self.combine_beacons();
        // FIXME: currently broken...
        Ok(beacons.len().into())
    }

    // What is the largest Manhattan distance between any two scanners?
    fn part_2(&self) -> Result<Solution> {
        let largest = self
            .scanner_positions
            .borrow()
            .iter()
            .enumerate()
            .tuple_combinations()
            .map(|((_, (_, pos_a)), (_, (_, pos_b)))| Self::manhattan_distance(pos_a, pos_b))
            .max()
            .unwrap();
        Ok(largest.into())
    }
}
