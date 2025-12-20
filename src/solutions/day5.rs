#![allow(clippy::all)]

//! # Day 5: Cafeteria ([challenge description](https://adventofcode.com/2025/day/5))

use std::{collections::VecDeque, ops::RangeInclusive};

/// # Puzzle 1
/// 
/// ## Summary
/// The input consists of two sections, separated by an empty line, with each section's values delimited by a newline.
/// The first section contains potentially overlapping, inclusive intervals of integers. The second section contains 
/// individual integers. The program must count the number of integers that are contained in at least one of the intervals.
/// 
/// ## Solution
/// First, parse the input to extract the intervals and integers. A trivial but inefficient solution is to then create an 
/// array of integer intervals, test each integer to see if at least one interval contains it, and count all the integers 
/// that pass the test. A slight optimization is to combine intervals that overlap. This might be an acceptable solution if
/// the number of intervals and integers to check are relatively low. It has a time complexity of O(n * r).
/// 
/// A more efficient solution is to find a way to create a virtual number line and consolidate all intervals. This could be
/// done by constructing a special balanced binary search tree known as an interval tree, which has a time complexity of O(log n).
/// Each node represents an inclusive interval. The left and right sub-trees point to intervals that have lower and higher starting 
/// endpoints respectively. Each node also contains the largest endpoint all of its subtrees contain, which is calculated after the 
/// initial tree is set up. The tree is constructed by sorting the intervals by the lower endpoint, choosing the middle interval for
/// the root node, and repeating the process to form the left and right sub-trees.
/// 
/// This problem is somewhat similar to checking if an IP is in a set of CIDR intervals.
/// 
/// Biggest lesson: You might be tempted to make things too abstract and complex. Stay simple.
/// https://corrode.dev/blog/simple/
/// 
/// Also see:
/// https://tildesites.bowdoin.edu/~ltoma/teaching/cs231/fall07/Lectures/augtrees.pdf
/// https://endler.dev/2017/boxes-and-trees/
/// 
#[must_use]
pub fn puzzle1(input: &str) -> u128 {
    let (intervals_str, integers_str) = input
        .trim()
        .split_once("\n\n")
        .expect("input must contain intervals and integers separated by an empty line");
    let intervals: Vec<std::ops::RangeInclusive<i64>> = intervals_str
        .split('\n')
        .map(|i| parse_inclusive_range(i).unwrap())
        .collect();
    let root_node = IntervalNode::from(intervals);
    integers_str
        .split('\n')
        .map(|i| i.parse::<i64>().expect("failed to parse 64-bit integer"))
        .filter(|i| root_node.contains(*i))
        .count()
        .try_into().expect("failed to convert to u128")
}

// Would've been nice if RangeInclusive<i64> would've had a built-in FromStr
fn parse_inclusive_range(s: &str) -> Result<RangeInclusive<i64>, String> {
    let parts: Vec<&str> = s
        .split('-')
        .collect();
    if parts.len() != 2 {
        return Err("interval must contain two integers delimited by a -".to_string());
    }
    let start = parts[0]
        .trim()
        .parse::<i64>()
        .map_err(|e| format!("invalid interval start value: {}", e))?;
    let end = parts[1]
        .trim()
        .parse::<i64>()
        .map_err(|e| format!("Invalid interval end value: {}", e))?;
    Ok(start..=end)
}


#[derive(Debug)]
struct IntervalNode {
    pub interval: RangeInclusive<i64>,
    pub max: i64,
    pub left: Option<Box<IntervalNode>>,
    pub right: Option<Box<IntervalNode>>,
}

impl From<Vec<RangeInclusive<i64>>> for IntervalNode {
    fn from(mut intervals: Vec<RangeInclusive<i64>>) -> Self {
        // TODO: Any way to return Option and avoid panic?
        assert_ne!(intervals.len(), 0, "there must be at least one interval");
        intervals.sort_by_key(|n| *n.start());
        *Self::build_from_intervals(&intervals).expect("no interval provided")
    }
}

impl IntervalNode {
    fn build_from_intervals(intervals: &[RangeInclusive<i64>]) -> Option<Box<IntervalNode>> {
        // TODO: Any way to avoid recursion?
        if intervals.is_empty() {
            return None;
        }
        let mid = intervals.len()/2;
        let ref r = intervals[mid];
        let mut node = Box::new(IntervalNode { interval: r.clone(), max: *r.end(), left: None, right: None });
        node.left = Self::build_from_intervals(&intervals[..mid]);
        node.right = Self::build_from_intervals(&intervals[mid + 1..]);
        node.max = *node.interval.end();
        if let Some(ref left) = node.left {
            node.max = node.max.max(left.max);
        }
        if let Some(ref right) = node.right {
            node.max = node.max.max(right.max);
        }
        Some(node)
    }
    fn contains(&self, integer: i64) -> bool {
        // A double-ended queue is better than recursion ;)
        let mut queue = VecDeque::new();
        queue.push_back(self);
        while let Some(n) = queue.pop_front() {
            if n.interval.contains(&integer) {
                return true;
            }
            if let Some(ref ln) = n.left {
                queue.push_back(ln);
            }
            if let Some(ref rn) = n.right {
                queue.push_back(rn);
            }
        }
        false
    }
}

/// # Puzzle 2
/// 
/// ## Summary
/// Find the unique set of integers obtained from a series of integer intervals.
/// 
/// ## Solution
/// A naive solution for this puzzle is using a hash set. Ignoring the second half of the input,
/// loop through integers in each interval and append unique values to the hash set. Finally, return the count of 
/// unique values. However, this is very slow and has a time complexity of O(n * (avg(n_end-n_start+1))).
/// 
/// A much more efficient solution is to sort and consolidate overlapping intervals and get the total unique numbers
/// by subtracting each interval's end from its start. This should have a time complexity of O(n log n) mainly for
/// sorting the intervals.
/// 
#[must_use]
pub fn puzzle2(input: &str) -> u128 {
    let (intervals_str, _) = input
        .trim()
        .split_once("\n\n")
        .expect("input must contain intervals and integers separated by an empty line");
    let mut intervals: Vec<RangeInclusive<i64>> = intervals_str
        .split('\n')
        .map(|i| parse_inclusive_range(i).unwrap())
        .collect();
    if intervals.len() == 0 {
        return 0;
    }
    intervals.sort_by_key(|n| *n.start());
    let mut total: u128 = 0;
    let mut current_start = intervals[0].start();
    let mut current_end = intervals[0].end();
    for i in intervals.iter().skip(1) {
        if *i.start() <= current_end + 1 {
            current_end = current_end.max(i.end());
        } else {
            total += (current_end - current_start + 1) as u128;
            current_start = i.start();
            current_end = i.end();
        }
    }
    total += (current_end - current_start + 1) as u128;
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(TEST_INPUT), 3);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(TEST_INPUT), 14);
    }
}