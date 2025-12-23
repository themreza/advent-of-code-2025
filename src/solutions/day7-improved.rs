#[allow(dead_code)]

//! # Day 7: Laboratories ([challenge description](https://adventofcode.com/2025/day/7))
//!
//! ## Refactoring Summary
//!
//! - **Removed unused import**: `Seek` trait was imported but never used directly
//! - **Added constants**: Replaced magic characters (`b'S'`, `b'^'`, `b'\n'`) with named constants
//! - **Simplified `RayNode` construction**: Made `new()` return `Self` and added a separate
//!   `into_ref()` method for wrapping in `Rc<RefCell<_>>`
//! - **Cleaner borrowing in DFS**: Used `drop()` placement and scoping more idiomatically
//! - **Fixed unstable `let_chains`**: Replaced `if cond && let Some(x) = ...` with nested
//!   `if let` or `match` expressions for stable Rust compatibility
//! - **Used `Entry` API consistently**: Replaced `or_default().push/extend` patterns with
//!   cleaner entry API usage
//! - **Improved variable names**: `rr`/`ra`/`rps` → `col`/`nodes`, etc.
//! - **Extracted `read_char` helper**: Reduced code duplication for file reads
//! - **Used early returns**: Simplified control flow in grid processing loops
//! - **Removed redundant bounds check**: `c < self.columns` was always true within the loop
//! - **Consistent documentation style**: Aligned doc comments with Rust conventions

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Read, SeekFrom},
    os::unix::fs::FileExt,
    rc::Rc,
};

const START_CHAR: u8 = b'S';
const SPLIT_CHAR: u8 = b'^';
const NEWLINE: u8 = b'\n';

type RayNodeRef = Rc<RefCell<RayNode>>;
type Position = (usize, usize);

#[derive(Debug)]
struct RayNode {
    position: Position,
    children: Vec<RayNodeRef>,
}

impl RayNode {
    fn new(line_index: usize, char_index: usize) -> Self {
        Self {
            position: (line_index, char_index),
            children: Vec::new(),
        }
    }

    fn into_ref(self) -> RayNodeRef {
        Rc::new(RefCell::new(self))
    }

    fn add_child(parent: &RayNodeRef, child: &RayNodeRef) {
        parent.borrow_mut().children.push(Rc::clone(child));
    }

    fn count_unique_paths(root: &RayNodeRef) -> u64 {
        let mut cache = HashMap::new();
        Self::count_paths_memoized(root, &mut cache)
    }

    /// Counts paths using DFS with memoization.
    /// See: <https://www.geeksforgeeks.org/dsa/number-of-paths-from-source-to-destination-in-a-directed-acyclic-graph/>
    fn count_paths_memoized(node: &RayNodeRef, cache: &mut HashMap<Position, u64>) -> u64 {
        let borrowed = node.borrow();
        let key = borrowed.position;

        if let Some(&cached) = cache.get(&key) {
            return cached;
        }

        if borrowed.children.is_empty() {
            return 1;
        }

        let children = borrowed.children.clone();
        drop(borrowed); // Release borrow before recursive calls

        let total: u64 = children
            .iter()
            .map(|child| Self::count_paths_memoized(child, cache))
            .sum();

        cache.insert(key, total);
        total
    }
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    columns: usize,
    file: File,
}

impl TryFrom<&str> for Grid {
    type Error = &'static str;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let file = File::open(path).map_err(|_| "failed to open input file")?;
        let mut reader = BufReader::new(file);

        let line_len = Self::find_line_length(&mut reader)?;
        let file_len = reader.seek_relative(0).map_or_else(
            |_| Err("failed to get file position"),
            |()| {
                use std::io::Seek;
                reader
                    .seek(SeekFrom::End(0))
                    .map_err(|_| "failed to seek to end of file")
            },
        )?;

        if file_len == 0 {
            return Err("invalid zero-length file");
        }

        Ok(Self {
            rows: file_len as usize / line_len,
            columns: line_len,
            file: reader.into_inner(),
        })
    }
}

impl Grid {
    /// Finds the length of the first line (including newline character).
    fn find_line_length(reader: &mut BufReader<File>) -> Result<usize, &'static str> {
        let mut line_len = 0_usize;
        let mut buf = [0u8; 8192];

        loop {
            let n = reader
                .read(&mut buf)
                .map_err(|_| "failed to read line chunk into buffer")?;

            if n == 0 {
                break;
            }

            for &byte in &buf[..n] {
                line_len += 1;
                if byte == NEWLINE {
                    return Ok(line_len);
                }
            }
        }

        Err("invalid input: no newline character found")
    }

    /// Reads a single character at the given (row, col) position.
    fn read_char(&self, row: usize, col: usize) -> u8 {
        let mut buf = [0u8; 1];
        let offset = (row * self.columns + col) as u64;
        self.file
            .read_at(&mut buf, offset)
            .expect("failed to read character");
        buf[0]
    }

    /// Calculates the byte offset for a grid position.
    fn offset(&self, row: usize, col: usize) -> u64 {
        (row * self.columns + col) as u64
    }

    pub fn count_ray_splits(self) -> u64 {
        let mut rays: HashSet<usize> = HashSet::new();
        let mut total_splits = 0_u64;

        for row in (0..self.rows).step_by(2) {
            if row == 0 {
                // Find starting position
                if let Some(col) = (0..self.columns).find(|&c| self.read_char(0, c) == START_CHAR) {
                    rays.insert(col);
                }
                continue;
            }

            let splits: Vec<usize> = rays
                .iter()
                .copied()
                .filter(|&col| self.read_char(row, col) == SPLIT_CHAR)
                .collect();

            total_splits += splits.len() as u64;

            for col in splits {
                rays.remove(&col);
                rays.insert(col - 1);
                rays.insert(col + 1);
            }
        }

        total_splits
    }

    pub fn build_graph(self) -> RayNodeRef {
        let mut ray_map: HashMap<usize, Vec<RayNodeRef>> = HashMap::new();
        let mut root_node: Option<RayNodeRef> = None;

        for row in (0..self.rows).step_by(2) {
            if row == 0 {
                // Find and create root node
                if let Some(col) = (0..self.columns).find(|&c| self.read_char(0, c) == START_CHAR) {
                    let root = RayNode::new(row, col).into_ref();
                    root_node = Some(Rc::clone(&root));
                    ray_map.insert(col, vec![root]);
                }
                continue;
            }

            let mut new_rays: HashMap<usize, Vec<RayNodeRef>> = HashMap::new();
            let split_cols: Vec<usize> = ray_map
                .keys()
                .copied()
                .filter(|&col| self.read_char(row, col) == SPLIT_CHAR)
                .collect();

            for col in split_cols {
                let Some(parent_nodes) = ray_map.remove(&col) else {
                    continue;
                };

                let left_child = RayNode::new(row, col - 1).into_ref();
                let right_child = RayNode::new(row, col + 1).into_ref();

                for parent in &parent_nodes {
                    RayNode::add_child(parent, &left_child);
                    RayNode::add_child(parent, &right_child);
                }

                new_rays
                    .entry(col - 1)
                    .or_default()
                    .push(Rc::clone(&left_child));
                new_rays
                    .entry(col + 1)
                    .or_default()
                    .push(Rc::clone(&right_child));
            }

            for (col, nodes) in new_rays {
                ray_map.entry(col).or_default().extend(nodes);
            }
        }

        root_node.expect("failed to identify the root node")
    }
}

/// # Puzzle 1
///
/// Given a grid starting from `S`, trace vertical rays downward. When a ray hits `^`,
/// it splits into two rays at positions -1 and +1. Count total splits.
#[must_use]
pub fn puzzle1(input_path: &str) -> u64 {
    Grid::try_from(input_path)
        .expect("failed to construct grid from input file")
        .count_ray_splits()
}

/// # Puzzle 2
///
/// Build a DAG from ray splits and count all unique paths from root to leaves.
#[must_use]
pub fn puzzle2(input_path: &str) -> u64 {
    let root = Grid::try_from(input_path)
        .expect("failed to construct grid from input file")
        .build_graph();
    RayNode::count_unique_paths(&root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&utils::string_to_file(TEST_INPUT)), 21);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&utils::string_to_file(TEST_INPUT)), 40);
    }
}