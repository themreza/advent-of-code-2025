#[allow(dead_code)]

//! # Day 5: Cafeteria ([challenge description](https://adventofcode.com/2025/day/5))
//!
//! ## Changes Summary
//!
//! - **Removed unused import**: `HashSet` was imported but never used
//! - **Organized imports**: Grouped related imports together
//! - **Simplified `parse_inclusive_range`**: Replaced `split().collect()` with `split_once()`
//!   for cleaner two-part parsing
//! - **Fixed non-idiomatic reference binding**: Changed `let ref r = ...` to `let r = &...`
//! - **Used `is_empty()`**: Replaced `intervals.len() == 0` with the idiomatic `is_empty()`
//! - **Removed unnecessary `pub` visibility**: Fields on private struct don't need `pub`
//! - **Switched to `TryFrom`**: Replaced panicking `From` impl with fallible `TryFrom`,
//!   propagating the error to callers
//! - **Removed redundant type annotations**: Let type inference work where obvious
//! - **Simplified numeric conversions**: Used `u128::from()` instead of `try_into().expect()`
//!   for infallible `usize` → `u128` conversion
//! - **Cleaner iterator chains**: Removed intermediate variables where a chain is clearer
//! - **Consistent error message casing**: Standardized to lowercase error messages per Rust convention
//! - **Dereferencing clarity**: Used explicit dereferencing patterns for cleaner range comparisons

use std::{collections::VecDeque, ops::RangeInclusive};

/// Parses a string like "3-5" into an inclusive range `3..=5`.
fn parse_inclusive_range(s: &str) -> Result<RangeInclusive<i64>, String> {
    let (start_str, end_str) = s
        .split_once('-')
        .ok_or("interval must contain two integers delimited by a '-'")?;

    let start = start_str
        .trim()
        .parse::<i64>()
        .map_err(|e| format!("invalid interval start value: {e}"))?;

    let end = end_str
        .trim()
        .parse::<i64>()
        .map_err(|e| format!("invalid interval end value: {e}"))?;

    Ok(start..=end)
}

#[derive(Debug)]
struct IntervalNode {
    interval: RangeInclusive<i64>,
    max: i64,
    left: Option<Box<IntervalNode>>,
    right: Option<Box<IntervalNode>>,
}

impl TryFrom<Vec<RangeInclusive<i64>>> for IntervalNode {
    type Error = &'static str;

    fn try_from(mut intervals: Vec<RangeInclusive<i64>>) -> Result<Self, Self::Error> {
        if intervals.is_empty() {
            return Err("there must be at least one interval");
        }
        intervals.sort_by_key(|n| *n.start());
        // SAFETY: We checked non-empty above, so `build_from_intervals` will return `Some`
        Ok(*Self::build_from_intervals(&intervals).unwrap())
    }
}

impl IntervalNode {
    fn build_from_intervals(intervals: &[RangeInclusive<i64>]) -> Option<Box<IntervalNode>> {
        if intervals.is_empty() {
            return None;
        }

        let mid = intervals.len() / 2;
        let r = &intervals[mid];

        let left = Self::build_from_intervals(&intervals[..mid]);
        let right = Self::build_from_intervals(&intervals[mid + 1..]);

        let mut max = *r.end();
        if let Some(ref left_node) = left {
            max = max.max(left_node.max);
        }
        if let Some(ref right_node) = right {
            max = max.max(right_node.max);
        }

        Some(Box::new(IntervalNode {
            interval: r.clone(),
            max,
            left,
            right,
        }))
    }

    fn contains(&self, integer: i64) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            if node.interval.contains(&integer) {
                return true;
            }
            if let Some(ref left) = node.left {
                queue.push_back(left);
            }
            if let Some(ref right) = node.right {
                queue.push_back(right);
            }
        }

        false
    }
}

/// # Puzzle 1
///
/// ## Summary
///
/// The input consists of two sections, separated by an empty line, with each
/// section's values delimited by a newline. The first section contains potentially
/// overlapping, inclusive intervals of integers. The second section contains
/// individual integers. The program must count the number of integers that are
/// contained in at least one of the intervals.
///
/// ## Solution
///
/// First, parse the input to extract the intervals and integers. A trivial but
/// inefficient solution is to then create an array of integer intervals, test each
/// integer to see if at least one interval contains it, and count all the integers
/// that pass the test. A slight optimization is to combine intervals that overlap.
/// This might be an acceptable solution if the number of intervals and integers to
/// check are relatively low. It has a time complexity of O(n * r).
///
/// A more efficient solution is to find a way to create a virtual number line and
/// consolidate all intervals. This could be done by constructing a special balanced
/// binary search tree known as an interval tree, which has a time complexity of
/// O(log n). Each node represents an inclusive interval. The left and right sub-trees
/// point to intervals that have lower and higher starting endpoints respectively.
/// Each node also contains the largest endpoint all of its subtrees contain, which
/// is calculated after the initial tree is set up. The tree is constructed by sorting
/// the intervals by the lower endpoint, choosing the middle interval for the root
/// node, and repeating the process to form the left and right sub-trees.
///
/// This problem is somewhat similar to checking if an IP is in a set of CIDR intervals.
///
/// Biggest lesson: You might be tempted to make things too abstract and complex. Stay simple.
/// <https://corrode.dev/blog/simple/>
///
/// Also see:
/// - <https://tildesites.bowdoin.edu/~ltoma/teaching/cs231/fall07/Lectures/augtrees.pdf>
/// - <https://endler.dev/2017/boxes-and-trees/>
#[must_use]
pub fn puzzle1(input: &str) -> u128 {
    let (intervals_str, integers_str) = input
        .trim()
        .split_once("\n\n")
        .expect("input must contain intervals and integers separated by an empty line");

    let intervals: Vec<_> = intervals_str
        .lines()
        .map(|line| parse_inclusive_range(line).expect("failed to parse interval"))
        .collect();

    let root_node = IntervalNode::try_from(intervals).expect("no intervals provided");

    u128::from(
        integers_str
            .lines()
            .map(|line| line.parse::<i64>().expect("failed to parse integer"))
            .filter(|&i| root_node.contains(i))
            .count() as u32,
    )
}

/// # Puzzle 2
///
/// ## Summary
///
/// Find the unique set of integers obtained from a series of integer intervals.
///
/// ## Solution
///
/// A naive solution for this puzzle is using a hash set. Ignoring the second half
/// of the input, loop through integers in each interval and append unique values
/// to the hash set. Finally, return the count of unique values. However, this is
/// very slow and has a time complexity of O(n * (avg(n_end - n_start + 1))).
///
/// A much more efficient solution is to sort and consolidate overlapping intervals
/// and get the total unique numbers by subtracting each interval's end from its
/// start. This should have a time complexity of O(n log n) mainly for sorting
/// the intervals.
#[must_use]
pub fn puzzle2(input: &str) -> u128 {
    let (intervals_str, _) = input
        .trim()
        .split_once("\n\n")
        .expect("input must contain intervals and integers separated by an empty line");

    let mut intervals: Vec<_> = intervals_str
        .lines()
        .map(|line| parse_inclusive_range(line).expect("failed to parse interval"))
        .collect();

    if intervals.is_empty() {
        return 0;
    }

    intervals.sort_by_key(|r| *r.start());

    let mut total: u128 = 0;
    let mut current_start = *intervals[0].start();
    let mut current_end = *intervals[0].end();

    for interval in intervals.iter().skip(1) {
        if *interval.start() <= current_end + 1 {
            current_end = current_end.max(*interval.end());
        } else {
            total += (current_end - current_start + 1) as u128;
            current_start = *interval.start();
            current_end = *interval.end();
        }
    }

    total + (current_end - current_start + 1) as u128
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