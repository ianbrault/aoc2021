/*
** src/puzzles/day_12.rs
** https://adventofcode.com/2021/day/12
*/

use crate::types::{Puzzle, Result, Solution};
use crate::utils;

use std::collections::{HashMap, HashSet};

pub struct Day12 {
    cave_connections: HashMap<&'static str, Vec<&'static str>>,
}

impl Day12 {
    pub fn new(input: &'static str) -> Self {
        let mut cave_connections = HashMap::new();

        for line in utils::input_to_lines(input) {
            match split!(line, "-") {
                [from, to] => {
                    // NOTE: cave connections are bi-directional!
                    let entry_from = cave_connections.entry(*from).or_insert_with(Vec::new);
                    entry_from.push(*to);
                    let entry_to = cave_connections.entry(*to).or_insert_with(Vec::new);
                    entry_to.push(*from);
                }
                _ => unreachable!(),
            }
        }

        Self { cave_connections }
    }

    fn is_start(cave: &str) -> bool {
        cave == "start"
    }

    fn is_end(cave: &str) -> bool {
        cave == "end"
    }

    fn is_small_cave(cave: &str) -> bool {
        cave.chars().all(char::is_lowercase)
    }

    fn find_paths_small_caves_once_rec(
        &self,
        from: &'static str,
        mut visited: HashSet<&str>,
    ) -> Vec<Vec<&str>> {
        let mut paths = vec![];
        // add the current cave to the visited caves if it is a small cave
        if Self::is_small_cave(from) {
            visited.insert(from);
        }

        // recurse on un-visited caves
        if let Some(connected_caves) = self.cave_connections.get(from) {
            for cave in connected_caves.iter() {
                if !visited.contains(cave) {
                    // base case: end
                    if Self::is_end(cave) {
                        paths.push(vec![*cave, from]);
                    } else {
                        let paths_rec = self.find_paths_small_caves_once_rec(cave, visited.clone());
                        // add the current cave to the paths and continue
                        for mut path in paths_rec.into_iter() {
                            path.push(from);
                            paths.push(path);
                        }
                    }
                }
            }
        }

        paths
    }

    fn find_paths_small_caves_once(&self) -> Vec<Vec<&str>> {
        let visited = HashSet::new();
        self.find_paths_small_caves_once_rec("start", visited)
    }

    fn find_paths_small_caves_once_or_twice_rec(
        &self,
        from: &'static str,
        mut visited: HashSet<&str>,
        twice_visited: bool,
    ) -> Vec<Vec<&str>> {
        let mut paths = vec![];
        // add the current cave to the visited caves if it is a small cave
        if Self::is_small_cave(from) {
            visited.insert(from);
        }

        // recurse on un-visited caves
        if let Some(connected_caves) = self.cave_connections.get(from) {
            for cave in connected_caves.iter() {
                // the small cave revisit adds the option for a second branching point
                // if we have already visited a small cave but have not visited any small cave
                // twice, we can (a) skip the cave or (b) continuing on with the cave
                // note: not true for the start cave
                if visited.contains(cave) && !twice_visited && !Self::is_start(cave) {
                    // base case: end
                    if Self::is_end(cave) {
                        paths.push(vec![*cave, from]);
                    } else {
                        let paths_rec = self.find_paths_small_caves_once_or_twice_rec(
                            cave,
                            visited.clone(),
                            true,
                        );
                        // add the current cave to the paths and continue
                        for mut path in paths_rec.into_iter() {
                            path.push(from);
                            paths.push(path);
                        }
                    }
                } else if !visited.contains(cave) {
                    // base case: end
                    if Self::is_end(cave) {
                        paths.push(vec![*cave, from]);
                    } else {
                        let paths_rec = self.find_paths_small_caves_once_or_twice_rec(
                            cave,
                            visited.clone(),
                            twice_visited,
                        );
                        // add the current cave to the paths and continue
                        for mut path in paths_rec.into_iter() {
                            path.push(from);
                            paths.push(path);
                        }
                    }
                }
            }
        }
        paths
    }

    fn find_paths_small_caves_once_or_twice(&self) -> Vec<Vec<&str>> {
        let visited = HashSet::new();
        self.find_paths_small_caves_once_or_twice_rec("start", visited, false)
    }
}

impl Puzzle for Day12 {
    // How many paths through this cave system are there that visit small caves at most once?
    fn part_1(&self) -> Result<Solution> {
        Ok(self.find_paths_small_caves_once().len().into())
    }

    // After reviewing the available paths, you realize you might have time to visit a single small
    // cave twice. Given these new rules, how many paths through this cave system are there?
    fn part_2(&self) -> Result<Solution> {
        Ok(self.find_paths_small_caves_once_or_twice().len().into())
    }
}
