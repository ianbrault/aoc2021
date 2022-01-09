/*
** src/puzzles/day_18.rs
** https://adventofcode.com/2021/day/18
*/

use crate::types::{Puzzle, Result, Solution, Tree};
use crate::utils;

use std::cmp;
use std::fmt;
use std::ops::Add;

const INPUT: &str = include_str!("../../input/18.txt");

#[derive(Clone, Debug, PartialEq)]
enum NumberType {
    Number(u8),
    Nested,
}

impl NumberType {
    fn number(&self) -> u8 {
        match self {
            Self::Number(n) => *n,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for NumberType {
    fn from(n: u8) -> Self {
        Self::Number(n)
    }
}

impl From<u32> for NumberType {
    fn from(n: u32) -> Self {
        Self::Number(n as u8)
    }
}

type NumberTree = Tree<NumberType>;

struct SnailfishNumber {
    tree: NumberTree,
}

impl SnailfishNumber {
    fn parse_number(tree: &mut NumberTree, s: &str, node_id: u64, pos: &mut usize) {
        // skip the leading bracket
        *pos += 1;

        while *pos < s.len() {
            let c = s.chars().nth(*pos).unwrap();
            if c == ',' {
                // continue, not relevant for parsing
                *pos += 1;
            } else if c == ']' {
                // terminate
                *pos += 1;
                break;
            } else if c.is_ascii_digit() {
                // leaf node of the tree, insert and continue
                tree.insert(c.to_digit(10).unwrap().into(), Some(node_id));
                *pos += 1;
            } else if c == '[' {
                // add a branch point and recurse down another level
                let new_node = tree.insert(NumberType::Nested, Some(node_id));
                Self::parse_number(tree, s, new_node, pos);
            }
        }
    }

    fn find_nested_pair_rec(&self, depth: usize, node_id: u64) -> Option<u64> {
        let node = self.tree.node(node_id).unwrap();
        for child_id in node.children.iter() {
            let child_node = self.tree.node(*child_id).unwrap();
            if child_node.data == NumberType::Nested {
                if depth == 4 {
                    return Some(*child_id);
                } else if let Some(id) = self.find_nested_pair_rec(depth + 1, *child_id) {
                    return Some(id);
                }
            }
        }

        None
    }

    fn magnitude_rec(&self, node_id: u64) -> u64 {
        let node = self.tree.node(node_id).unwrap();
        match node.data {
            NumberType::Number(n) => n as u64,
            NumberType::Nested => {
                bind_vec_deref!(node.children, left_id, right_id);
                (3 * self.magnitude_rec(left_id)) + (2 * self.magnitude_rec(right_id))
            }
        }
    }

    fn magnitude(&self) -> u64 {
        match self.tree.root {
            Some(root_id) => {
                let node = self.tree.node(root_id).unwrap();
                bind_vec_deref!(node.children, left_id, right_id);
                (3 * self.magnitude_rec(left_id)) + (2 * self.magnitude_rec(right_id))
            }
            _ => unreachable!(),
        }
    }

    // finds the leftmost pair nested inside 4 pairs
    fn find_nested_pair(&self) -> Option<u64> {
        if let Some(root) = self.tree.root {
            self.find_nested_pair_rec(1, root)
        } else {
            None
        }
    }

    fn explode(mut self, node_id: u64) -> Self {
        let node = self.tree.node(node_id).unwrap();

        // grab the left and right elements of the nested pair
        bind_vec_deref!(node.children, left_id, right_id);
        let left = self.tree.node_data(left_id).unwrap().number();
        let right = self.tree.node_data(right_id).unwrap().number();

        // check for a left neighbor and add the left element to it, if found
        if let Some(left_neighbor_id) = self.tree.left_neighbor_leaf(left_id) {
            let mut node = self.tree.node_mut(left_neighbor_id).unwrap();
            // note: assumes that this is a number and not a nested pair
            node.data = (node.data.number() + left).into();
        }
        // check for a right neighbor and add the right element to it, if found
        if let Some(right_neighbor_id) = self.tree.right_neighbor_leaf(right_id) {
            let mut node = self.tree.node_mut(right_neighbor_id).unwrap();
            // note: assumes that this is a number and not a nested pair
            node.data = (node.data.number() + right).into();
        }

        // first remove the children
        self.tree.remove(left_id);
        self.tree.remove(right_id);
        // then replace the nested pair with 0
        // note: need to borrow mutably here separate from immutable borrows above
        self.tree.node_mut(node_id).unwrap().data = 0u8.into();

        self
    }

    fn find_big_pair_rec(&self, node_id: u64) -> Option<u64> {
        let node = self.tree.node(node_id).unwrap();
        match node.data {
            NumberType::Number(n) => {
                if n > 9 {
                    Some(node_id)
                } else {
                    None
                }
            }
            NumberType::Nested => {
                for child_id in node.children.iter() {
                    if let Some(id) = self.find_big_pair_rec(*child_id) {
                        return Some(id);
                    }
                }
                None
            }
        }
    }

    // finds a number greater than or equal to 10
    fn find_big_pair(&self) -> Option<u64> {
        if let Some(root) = self.tree.root {
            self.find_big_pair_rec(root)
        } else {
            None
        }
    }

    fn split(mut self, node_id: u64) -> Self {
        let mut node = self.tree.node_mut(node_id).unwrap();
        let n = match node.data {
            NumberType::Number(n) => n,
            _ => unreachable!(),
        };

        node.data = NumberType::Nested;
        self.tree.insert((n / 2).into(), Some(node_id));
        self.tree.insert(((n + 1) / 2).into(), Some(node_id));

        self
    }

    fn reduce_number(mut self) -> Self {
        let mut continue_reduction = true;
        while continue_reduction {
            continue_reduction = false;

            // first check for explode then check for split
            // either being found returns to the top of the loop
            if let Some(node_id) = self.find_nested_pair() {
                self = self.explode(node_id);
                continue_reduction = true;
            } else if let Some(node_id) = self.find_big_pair() {
                self = self.split(node_id);
                continue_reduction = true;
            }
        }

        self
    }

    fn to_string(&self, node_id: u64) -> String {
        if let Some(node) = self.tree.node(node_id) {
            match node.data {
                NumberType::Number(n) => n.to_string(),
                NumberType::Nested => {
                    let children = node
                        .children
                        .iter()
                        .map(|&child_id| self.to_string(child_id))
                        .collect::<Vec<_>>();
                    format!("[{}]", children.join(","))
                }
            }
        } else {
            String::new()
        }
    }
}

impl From<&str> for SnailfishNumber {
    fn from(s: &str) -> Self {
        // build up a tree representation
        let mut tree = Tree::new();
        let node_id = tree.insert(NumberType::Nested, None);

        Self::parse_number(&mut tree, s, node_id, &mut 0);
        Self { tree }
    }
}

impl Add<Self> for &SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let tree = Tree::combine_trees(&self.tree, &rhs.tree, NumberType::Nested);
        let output = SnailfishNumber { tree };
        output.reduce_number()
    }
}

impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(root_id) = self.tree.root {
            write!(f, "{}", self.to_string(root_id))
        } else {
            write!(f, "")
        }
    }
}

pub struct Day18 {
    numbers: Vec<SnailfishNumber>,
}

impl Day18 {
    pub fn new() -> Self {
        let numbers = utils::input_to_lines(INPUT)
            .map(SnailfishNumber::from)
            .collect();
        Self { numbers }
    }
}

impl Puzzle for Day18 {
    // Add up all of the snailfish numbers from the homework assignment in the order they appear.
    // What is the magnitude of the final sum?
    fn part_1(&self) -> Result<Solution> {
        let mut sum = &self.numbers[0] + &self.numbers[1];
        for number in self.numbers.iter().skip(2) {
            sum = &sum + number;
        }
        Ok(sum.magnitude().into())
    }

    // What is the largest magnitude of any sum of two different snailfish numbers from the
    // homework assignment?
    fn part_2(&self) -> Result<Solution> {
        let mut max_magnitude = 0;
        for i in 0..(self.numbers.len() - 1) {
            for j in (i + 1)..self.numbers.len() {
                let a = &self.numbers[i];
                let b = &self.numbers[j];
                let c = a + b;
                let d = b + a;
                max_magnitude = cmp::max(max_magnitude, cmp::max(c.magnitude(), d.magnitude()));
            }
        }

        Ok(max_magnitude.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_snailfish_number_simple() {
        let number = SnailfishNumber::from("[1,2]");

        let root = number.tree.root;
        assert!(root.is_some());

        let root_node = number.tree.node(root.unwrap()).unwrap();
        assert_eq!(root_node.children.len(), 2);
        for (node_id, exp) in root_node.children.iter().zip([1, 2]) {
            let node = number.tree.node(*node_id).unwrap();
            assert_eq!(node.data, NumberType::Number(exp));
            assert_eq!(node.children.len(), 0);
        }
    }

    #[test]
    fn test_parse_snailfish_number_nested() {
        let number = SnailfishNumber::from("[[[[[9,8],1],2],3],4]");

        let root = number.tree.root;
        assert!(root.is_some());
        let root_node = number.tree.node(root.unwrap()).unwrap();
        assert_eq!(root_node.children.len(), 2);

        let left_id = root_node.children[0];
        let left_node = number.tree.node(left_id).unwrap();
        assert_eq!(left_node.data, NumberType::Nested);
        assert_eq!(left_node.children.len(), 2);

        let right_id = root_node.children[1];
        let right_node = number.tree.node(right_id).unwrap();
        assert_eq!(right_node.data, 4u8.into());
        assert_eq!(right_node.children.len(), 0);
    }

    #[test]
    fn test_snailfish_number_nested_pair() {
        let number = SnailfishNumber::from("[[[[[9,8],1],2],3],4]");
        assert_eq!(number.find_nested_pair(), Some(4));
    }

    #[test]
    fn test_add_snailfish_numbers() {
        let a = SnailfishNumber::from("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = SnailfishNumber::from("[1,1]");
        let c = &a + &b;
        let res = String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(format!("{}", c), res);
    }

    #[test]
    fn test_snailfish_number_magnitude() {
        let a = SnailfishNumber::from("[[1,2],[[3,4],5]]");
        assert_eq!(a.magnitude(), 143);

        let b = SnailfishNumber::from("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(b.magnitude(), 445);

        let c = SnailfishNumber::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(c.magnitude(), 3488);
    }
}
