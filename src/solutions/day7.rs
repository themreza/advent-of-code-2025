//! # Day 7: Laboratories ([challenge description](https://adventofcode.com/2025/day/7))

use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufReader, Read, Seek, SeekFrom}, os::unix::fs::FileExt, rc::Rc};

// Due to shared ownership, a Rc<RefCell<T>> needs to be used
type RayNodeRef = Rc<RefCell<RayNode>>;

#[derive(Debug)]
struct RayNode {
    line_index: usize,
    char_index: usize,
    children: Vec<RayNodeRef>,
}

impl RayNode {
    fn new(line_index: usize, char_index: usize) -> RayNodeRef {
        Rc::new(RefCell::new(RayNode{line_index, char_index, children: Vec::new()}))
    }

    fn add_child(parent: &RayNodeRef, child: &RayNodeRef) {
        parent.borrow_mut().children.push(Rc::clone(child));
    }

    fn count_unique_paths(root_node: &RayNodeRef) -> u64 {
        let mut cache = HashMap::new();
        Self::count_unique_paths_dfs_memoized(root_node, &mut cache)
    }

    // Inspired from https://www.geeksforgeeks.org/dsa/number-of-paths-from-source-to-destination-in-a-directed-acyclic-graph/
    // The cache uses the (line_index, char_index) coordinate as the key
    fn count_unique_paths_dfs_memoized(root_node: &RayNodeRef, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
        let rn = root_node.borrow();
        let key = (rn.line_index, rn.char_index);
        if let Some(hit) = cache.get(&key) {
            return *hit
        }
        if rn.children.is_empty() {
            return 1
        }
        let children = rn.children.clone();
        // Because root_node was borrowed above, it must be released
        drop(rn);
        let mut total_paths = 0u64;
        for c in &children {
            total_paths += RayNode::count_unique_paths_dfs_memoized(c, cache);
        }
        cache.insert(key, total_paths);
        total_paths
    }
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    columns: usize,
    file: File,
    // Specifically for Puzzle 1
    rays: HashSet<usize>,
    // Specifically for Puzzle 2
    root_ray_node: Option<RayNodeRef>,
    ray_map: HashMap<usize, Vec<RayNodeRef>>,
}

// Initialize a new Grid from an input file path
impl TryFrom<&str> for Grid {
    type Error = &'static str;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let file = File::open(path).map_err(|_| "failed to open input file")?;
        let mut reader = BufReader::new(file);
        // Get the length of the first line (including newline characters) while scanning with a fixed size buffer
        let mut line_len = 0_usize;
        let mut buf = [0u8; 8192];
        'find_line_length: loop {
            let n = reader.read(&mut buf).map_err(|_| "failed to read line chunk into buffer")?;
            if n == 0 {
                // EOF
                break;
            }
            for &b in &buf[..n] {
                line_len += 1;
                if b == b'\n' {
                    break 'find_line_length;
                }
            }
        }
        if line_len == 0 {
            return Err("invalid input not containing at least one newline character")
        }
        // Get the total length of the file and divide it by the length of first line to get total rows
        let file_len = reader.seek(SeekFrom::End(0)).map_err(|_| "failed to seek to the end of file")?;
        if file_len == 0 {
            return Err("invalid zero-length file");
        }
        let rows = file_len as usize / line_len;
        // Rewind back to reuse the reader
        let _ = reader.seek(SeekFrom::Start(0)).map_err(|_| "failed to seek back to the beginning of file");
        Ok(Grid { 
            rows,
            columns: line_len,
            rays: HashSet::new(),
            file: reader.into_inner(),
            root_ray_node: None,
            ray_map: HashMap::new(),
        })
    }
}

impl Grid {
    pub fn count_ray_splits(mut self) -> u64 {
        let mut total_splits = 0u64;
        let mut char_buf = [0u8; 1];
        for r in (0..self.rows).step_by(2) {
            let mut rays_to_add: Vec<usize> = vec![];
            let mut rays_to_remove: Vec<usize> = vec![];
            for c in 0..self.columns {
                self.file.read_at(&mut char_buf, ((r * self.columns) + c) as u64).expect("failed to read character");
                if r == 0 && char_buf[0] == b'S' {
                    self.rays.insert(c);
                    break;
                }
                if char_buf[0] == b'^' && self.rays.contains(&c) {
                    total_splits += 1;
                    rays_to_remove.push(c);
                    rays_to_add.push(c - 1);
                    if c < self.columns {
                        rays_to_add.push(c + 1);
                    }
                }
            }
            for rr in rays_to_remove {
                self.rays.remove(&rr);
            }
            for ra in rays_to_add {
                self.rays.insert(ra);
            }
        }
        total_splits
    }
    pub fn build_graph(mut self) -> Rc<RefCell<RayNode>> {
        let mut char_buf = [0u8; 1];
        for r in (0..self.rows).step_by(2) {
            let mut rays_to_add: HashMap<usize, Vec<RayNodeRef>> = HashMap::new();
            let mut rays_to_remove: Vec<usize> = vec![];
            for c in 0..self.columns {
                self.file.read_at(&mut char_buf, ((r * self.columns) + c) as u64).expect("failed to read character");
                if r == 0 && char_buf[0] == b'S' {
                    // Set the root node
                    let root_node = RayNode::new(r, c);
                    // Clone just increments the pointer and returns the same allocated data in memory
                    self.root_ray_node = Some(Rc::clone(&root_node));
                    self.ray_map.insert(c, vec![Rc::clone(&root_node)]);
                    break;
                }
                if char_buf[0] == b'^' && let Some(nodes) = self.ray_map.get(&c) {
                    // Create new children for each ray's node
                    rays_to_remove.push(c);
                    let left_child = RayNode::new(r, c-1);
                    let right_child = RayNode::new(r, c+1);     
                    for node in nodes {
                        RayNode::add_child(node, &left_child);
                        if c < self.columns {
                            RayNode::add_child(node, &right_child);
                        }
                    }
                    rays_to_add.entry(c - 1).or_default().push(Rc::clone(&left_child));
                    if c < self.columns {
                        rays_to_add.entry(c + 1).or_default().push(Rc::clone(&right_child));
                    }
                }
            }
            for rr in rays_to_remove {
                self.ray_map.remove(&rr);
            }
            for (ra, rps) in rays_to_add {
                // The new ray positions are mapped to references to new children of existing (root) ray node(s)
                self.ray_map.entry(ra).or_default().extend(rps);
            }
        }
        match self.root_ray_node {
            Some(rn) => rn,
            None => panic!("failed to identify the root node")
        }
    }
}

/// # Puzzle 1
/// 
/// ## Summary
/// Given an input file containing an m (rows) by n (cols) grid of characters and starting from the character S in 
/// the first line, draw an imaginary vertical lines and keep going downward until the line hits a ^ character in 
/// the line(s) below. Then stop that line and start two new lines on either side of the ^ character. Repeat this 
/// process for the new vertical lines and any consequent vertical lines every time a line hits a ^ character. 
/// Once the end of the file is reached, return the count of the total times a vertical line hit a ^ character.
/// 
/// ## Solution
/// Similar to the puzzles in day 6, let's assume that the input file could be arbitrarily large, whereas available
/// memory is limited, that is, there could be several lines and each line could contain hundreds of thousands of
/// characters. Assuming that all lines in the input have the same number of characters, scan the first line to get
/// the width of the grid, then get the total length of the file and divided it by the width of the grid to get the
/// total number of rows in the grid.
/// 
/// Once the grid is ready, start by reading the first line to find the location of the S character. Add the location 
/// of this character in into a hash set called rays. Go two lines down. For every ray in the hash set, check if the 
/// ray's position in the current line contains a ^ character. If it does, mark that index in the hash set and increment 
/// the total count of ray splits. Once all the rays have been checked, remove the marked rays and add two new rays at 
/// -1/+1 relative positions of each marked ray. Overriding rays from two adjacent ^ characters will override each other
/// in the hash set and avoid repeated work. Once all lines are processed, return the total count of ray splits.
/// 
/// There are probably more efficient solutions based on symmetry and pattern analysis of the shape of the Christmas tree
/// (e.g. ^ characters always appear on either side of the growing triangular tree shape), but I'm not getting into that.
/// My focus is learning how to write idiomatic Rust code that is easy to read.
/// 
#[must_use]
pub fn puzzle1(input_path: &str) -> u64 {
    Grid::try_from(input_path)
        .expect("failed to construct a grid from the input file")
        .count_ray_splits()
}

/// # Puzzle 2
/// 
/// ## Summary
/// This puzzle is similar to puzzle 1, but rather than counting the splits, we must count all the unique ray
/// paths from the starting point till the end of the input.
/// 
/// ## Solution
/// Since there is initially just a single ray that will split into two every time it hits a ^ character, we are
/// essentially creating a directed acyclic graph (DAG), where each node represents a ray. Each ray node can optionally
/// point to 0 to many child ray nodes. Once the DAG is prepared, the solution is the total unique paths from the root 
/// node to all leaves. 
/// 
/// Parse and prepare the grid as before. Start with the first line and determine the location of the S character. The root 
/// node is the ray emitted by the S character. As in puzzle 1, keep a hash set of positions to check, but also point them to 
/// existing ray nodes, so that the new rays can be added as their children once splits happens. This bit is important, because
/// in the implementation of puzzle 1, we override converging rays, but here we must preserve each path. Go two lines down and 
/// check for potential splits. Repeat until the end.
/// 
/// Finally, start a depth-first search to reach all leaves from the root node and increment a shared counter every time a leaf
/// is reached. An optimization is to memoize the search by caching the total number of paths at every node and reusing that value
/// when arriving from a different parent node to that node. The count is the answer to the puzzle.
/// 
#[must_use]
pub fn puzzle2(input_path: &str) -> u64 {
    let root_node = Grid::try_from(input_path)
        .expect("failed to construct a grid from the input file")
        .build_graph();
    // {:#?} pretty prints if the object implements Debug
    // println!("{:#?}", root_node);
    RayNode::count_unique_paths(&root_node)
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

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