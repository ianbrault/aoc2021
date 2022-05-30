/*
** src/types/mod.rs
*/

mod geometry;
mod math;

pub use self::geometry::{Line, Point};
pub use self::math::{FMatrix2x2, FVector2};

use crate::utils;

use num::Integer;

use std::collections::HashMap;
use std::error;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// variant to cover various solution types
#[derive(Debug)]
pub enum Solution {
    Int(i64),
    UInt(u64),
    String(String),
}

impl From<i32> for Solution {
    fn from(n: i32) -> Self {
        Self::Int(n as i64)
    }
}

impl From<i64> for Solution {
    fn from(n: i64) -> Self {
        Self::Int(n)
    }
}

impl From<u32> for Solution {
    fn from(n: u32) -> Self {
        Self::UInt(n as u64)
    }
}

impl From<u64> for Solution {
    fn from(n: u64) -> Self {
        Self::UInt(n)
    }
}

impl From<usize> for Solution {
    fn from(n: usize) -> Self {
        Self::UInt(n as u64)
    }
}

impl From<String> for Solution {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::UInt(u) => write!(f, "{}", u),
            Self::String(s) => write!(f, "{}", s),
        }
    }
}
// puzzles are trait objects which conform to the following interface
pub trait Puzzle {
    fn part_1(&self) -> Result<Solution>;
    fn part_2(&self) -> Result<Solution>;
}

#[derive(Debug)]
pub enum PuzzleError {
    NoSolution,
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSolution => write!(f, "no solution found"),
        }
    }
}

impl error::Error for PuzzleError {}

pub struct Array2D<T, const W: usize, const H: usize> {
    data: [[T; W]; H],
}

impl<T, const W: usize, const H: usize> Array2D<T, W, H> {
    pub fn new() -> Self
    where
        T: Copy + Default,
    {
        let data = [[T::default(); W]; H];
        Self { data }
    }

    pub const fn left(i: usize, j: usize) -> Option<(usize, usize)> {
        if j > 0 {
            Some((i, j - 1))
        } else {
            None
        }
    }

    pub const fn right(i: usize, j: usize) -> Option<(usize, usize)> {
        if j < W - 1 {
            Some((i, j + 1))
        } else {
            None
        }
    }

    pub const fn up(i: usize, j: usize) -> Option<(usize, usize)> {
        if i > 0 {
            Some((i - 1, j))
        } else {
            None
        }
    }

    pub const fn down(i: usize, j: usize) -> Option<(usize, usize)> {
        if i < H - 1 {
            Some((i + 1, j))
        } else {
            None
        }
    }

    pub const fn up_left(i: usize, j: usize) -> Option<(usize, usize)> {
        if i > 0 && j > 0 {
            Some((i - 1, j - 1))
        } else {
            None
        }
    }

    pub const fn up_right(i: usize, j: usize) -> Option<(usize, usize)> {
        if i > 0 && j < W - 1 {
            Some((i - 1, j + 1))
        } else {
            None
        }
    }

    pub const fn down_left(i: usize, j: usize) -> Option<(usize, usize)> {
        if i < H - 1 && j > 0 {
            Some((i + 1, j - 1))
        } else {
            None
        }
    }

    pub const fn down_right(i: usize, j: usize) -> Option<(usize, usize)> {
        if i < H - 1 && j < W - 1 {
            Some((i + 1, j + 1))
        } else {
            None
        }
    }

    pub const fn neighbors(i: usize, j: usize) -> [Option<(usize, usize)>; 4] {
        [
            Self::left(i, j),
            Self::right(i, j),
            Self::up(i, j),
            Self::down(i, j),
        ]
    }

    pub const fn neighbors_with_diagonal(i: usize, j: usize) -> [Option<(usize, usize)>; 8] {
        [
            Self::left(i, j),
            Self::right(i, j),
            Self::up(i, j),
            Self::down(i, j),
            Self::up_left(i, j),
            Self::up_right(i, j),
            Self::down_left(i, j),
            Self::down_right(i, j),
        ]
    }

    pub fn get(&self, i: usize, j: usize) -> T
    where
        T: Copy,
    {
        self.data[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, val: T) {
        self.data[i][j] = val;
    }

    fn iter_indices(&self) -> impl Iterator<Item = (usize, usize)> {
        itertools::iproduct!(0..H, 0..W)
    }

    pub fn iter_with_indices(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        // need collect().into_iter() in order to avoid lifetime issues with the map closure
        self.iter_indices()
            .map(|(i, j)| (i, j, &self.data[i][j]))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn find_index<P>(&self, predicate: P) -> Option<(usize, usize)>
    where
        P: Fn(&T) -> bool,
    {
        self.iter_with_indices()
            .find(|(_, _, x)| predicate(x))
            .map(|(i, j, _)| (i, j))
    }
}

impl<T, const W: usize, const H: usize> Array2D<T, W, H>
where
    T: Copy + Integer,
{
    pub fn increment(&mut self, i: usize, j: usize) {
        self.data[i][j] = self.data[i][j] + T::one();
    }
}

impl<T, const W: usize, const H: usize> From<&'static str> for Array2D<T, W, H>
where
    T: Copy + Default + FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    fn from(s: &'static str) -> Self {
        let mut arr = Self::new();
        for (i, line) in utils::input_to_lines(s).enumerate() {
            for (j, c) in line.chars().enumerate() {
                arr.data[i][j] = c.to_string().parse().unwrap();
            }
        }
        arr
    }
}

impl<T, const W: usize, const H: usize> Default for Array2D<T, W, H>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct Counter<T> {
    counts: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn insert(&mut self, val: T) {
        let el = self.counts.entry(val).or_insert(0);
        *el += 1;
    }

    pub fn insert_n(&mut self, val: T, count: usize) {
        let el = self.counts.entry(val).or_insert(0);
        *el += count;
    }

    pub fn min(&self) -> Option<usize> {
        self.counts.values().min().copied()
    }

    pub fn max(&self) -> Option<usize> {
        self.counts.values().max().copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.counts.iter()
    }
}

impl<T, I> From<I> for Counter<T>
where
    T: Clone + Eq + Hash,
    I: Iterator<Item = T>,
{
    fn from(it: I) -> Self {
        let mut counts = HashMap::new();
        for el in it {
            let count = counts.entry(el.clone()).or_insert(0);
            *count += 1;
        }
        Self { counts }
    }
}

pub struct TreeNode<T> {
    _id: u64,
    pub data: T,
    // ID of parent node
    parent: Option<u64>,
    // IDs of children nodes
    pub children: Vec<u64>,
}

impl<T> TreeNode<T> {
    fn new(_id: u64, data: T, parent: Option<u64>) -> Self {
        let children = vec![];
        Self {
            _id,
            data,
            parent,
            children,
        }
    }

    fn find_child(&self, node_id: u64) -> Option<usize> {
        self.children
            .iter()
            .enumerate()
            .find(|(_, &child_id)| child_id == node_id)
            .map(|(i, _)| i)
    }
}

pub struct Tree<T> {
    pub root: Option<u64>,
    nodes: Vec<Option<TreeNode<T>>>,
    // maps node IDs to their position in the nodes array
    node_positions: HashMap<u64, usize>,
    id_tracker: u64,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        let nodes = (0..64).map(|_| None).collect();
        let node_positions = HashMap::new();
        Self {
            root: None,
            nodes,
            node_positions,
            id_tracker: 0,
        }
    }

    pub fn node(&self, id: u64) -> Option<&TreeNode<T>> {
        let pos = self.node_positions[&id];
        self.nodes[pos].as_ref()
    }

    pub fn node_data(&self, id: u64) -> Option<&T> {
        self.node(id).map(|node| &node.data)
    }

    pub fn node_mut(&mut self, id: u64) -> Option<&mut TreeNode<T>> {
        let pos = self.node_positions[&id];
        self.nodes[pos].as_mut()
    }

    fn find_first_open_slot(&mut self) -> usize {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.is_none() {
                return i;
            }
        }

        // no slot found, resize
        let size = self.nodes.len();
        self.nodes.resize_with(size * 2, Default::default);
        size
    }

    pub fn insert(&mut self, data: T, parent: Option<u64>) -> u64 {
        let id = self.id_tracker;
        let node = TreeNode::new(id, data, parent);

        // add and track the new node
        let pos = self.find_first_open_slot();
        self.nodes[pos] = Some(node);
        self.node_positions.insert(id, pos);
        self.id_tracker += 1;

        // if provided, hook the node up to its parent
        if let Some(parent_id) = parent {
            let parent_node = self.node_mut(parent_id).unwrap();
            parent_node.children.push(id);
        } else {
            self.root = Some(id);
        }

        id
    }

    pub fn remove(&mut self, node_id: u64) {
        if let Some(node) = self.node(node_id) {
            // unhook from the parent
            if let Some(parent_id) = node.parent {
                let parent = self.node_mut(parent_id).unwrap();
                let i = parent.find_child(node_id).unwrap();
                parent.children.remove(i);
            }

            // remove from the nodes and node position structures
            let pos = self.node_positions.remove(&node_id).unwrap();
            self.nodes[pos] = None;
        }
    }

    pub fn left_neighbor_node(&self, node_id: u64) -> Option<u64> {
        if let Some(node) = self.node(node_id) {
            // grab the parent
            if let Some(parent_id) = node.parent {
                let parent = self.node(parent_id).unwrap();
                // check the parent's children for a left neighbor
                let i = parent.find_child(node_id).unwrap();
                if i > 0 {
                    Some(parent.children[i - 1])
                } else {
                    // otherwise need to check the parent's parent
                    self.left_neighbor_node(parent_id)
                }
            } else {
                // otherwise there is no neighbor
                None
            }
        } else {
            None
        }
    }

    pub fn left_neighbor_leaf(&self, node_id: u64) -> Option<u64> {
        if let Some(neighbor_id) = self.left_neighbor_node(node_id) {
            let mut id = neighbor_id;
            let mut node = self.node(neighbor_id).unwrap();
            while !node.children.is_empty() {
                // check the rightmost child
                id = node.children[node.children.len() - 1];
                node = self.node(id).unwrap();
            }
            Some(id)
        } else {
            None
        }
    }

    pub fn right_neighbor_node(&self, node_id: u64) -> Option<u64> {
        if let Some(node) = self.node(node_id) {
            // grab the parent
            if let Some(parent_id) = node.parent {
                let parent = self.node(parent_id).unwrap();
                // check the parent's children for a right neighbor
                let i = parent.find_child(node_id).unwrap();
                if i < parent.children.len() - 1 {
                    Some(parent.children[i + 1])
                } else {
                    // otherwise need to check the parent's parent
                    self.right_neighbor_node(parent_id)
                }
            } else {
                // otherwise there is no neighbor
                None
            }
        } else {
            None
        }
    }

    pub fn right_neighbor_leaf(&self, node_id: u64) -> Option<u64> {
        if let Some(neighbor_id) = self.right_neighbor_node(node_id) {
            let mut id = neighbor_id;
            let mut node = self.node(neighbor_id).unwrap();
            while !node.children.is_empty() {
                // check the leftmost child
                id = node.children[0];
                node = self.node(id).unwrap();
            }
            Some(id)
        } else {
            None
        }
    }

    fn consume_tree(&mut self, tree: &Self, from_node: u64, into_node: u64)
    where
        T: Clone,
    {
        if let Some(node) = tree.node(from_node) {
            let new_id = self.insert(node.data.clone(), Some(into_node));
            for child_id in node.children.iter() {
                self.consume_tree(tree, *child_id, new_id);
            }
        }
    }

    // combines two trees under a single root
    pub fn combine_trees(tree_a: &Self, tree_b: &Self, root: T) -> Self
    where
        T: Clone,
    {
        let mut tree = Self::new();
        let root_id = tree.insert(root, None);
        // add the left and right trees
        if let Some(left_root) = tree_a.root {
            tree.consume_tree(tree_a, left_root, root_id);
        }
        if let Some(right_root) = tree_b.root {
            tree.consume_tree(tree_b, right_root, root_id);
        }

        tree
    }
}
