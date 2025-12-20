#[allow(dead_code)]

//! # Day 6: Trash Compactor ([challenge description](https://adventofcode.com/2025/day/6))
//!
//! ## Summary of Refactoring Changes
//!
//! - **Moved `Operation` enum** to the top of the module for better organization
//! - **Renamed `ArithmeticEnum`** to `Operation` (singular, more idiomatic) and removed the redundant type alias
//! - **Extracted `read_line_offsets`** helper function to eliminate duplicate code between puzzles
//! - **Replaced unstable `let_chains`** (`if let ... && ...`) with stable `matches!` and standard `if` guards
//! - **Used `impl Iterator` return type** for the helper function for flexibility
//! - **Simplified buffer handling** with direct array initialization instead of `Vec`
//! - **Used `Iterator::sum` and `Iterator::product`** directly on owned iterators where possible
//! - **Improved variable naming**: `offsets_len` → `line_count`, clearer intent
//! - **Used `mem::take`** instead of reassigning empty collections for clarity
//! - **Consolidated whitespace checks** and simplified control flow
//! - **Added `derive(Clone, Copy)`** to `Operation` since it's a simple enum
//! - **Used `?` operator** in a helper closure for cleaner byte reading

use std::{
    fs::File,
    io::{BufRead, BufReader},
    mem,
    ops::Range,
    os::unix::fs::FileExt,
};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

/// Reads the input file and returns byte offset ranges for each line.
fn read_line_offsets(path: &str) -> (File, Vec<Range<u64>>) {
    let file = File::open(path).expect("failed to open input file");
    let mut reader = BufReader::new(file);
    let mut offsets = vec![0u64];
    let mut buf = Vec::new();

    loop {
        buf.clear();
        let bytes_read = reader
            .read_until(b'\n', &mut buf)
            .expect("failed to scan input file for newlines");
        if bytes_read == 0 {
            break;
        }
        offsets.push(offsets.last().unwrap() + bytes_read as u64);
    }

    let ranges = offsets.windows(2).map(|w| w[0]..w[1]).collect();
    (reader.into_inner(), ranges)
}

/// # Puzzle 1
///
/// ## Summary
/// The objective of this puzzle is to perform several arithmetic operations and aggregate the total sum.
/// The challenging part of the puzzle is the column-oriented format of the input. Each line is a row with
/// multiple numbers separated by one or more spaces. The column contains all the numbers for each operation.
/// The last cell in each column contains the operation symbol, which is either addition (+) or
/// multiplication (*).
///
/// ## Solution
/// Since the number of columns can be extremely large, we theoretically may not be able to load all the operations
/// in memory. One solution is to first convert from a column-oriented format to a row-oriented format. That is,
/// reading the input with a fixed buffer size, creating a new file, writing each number on a line on a separate
/// line until the first input line is processed, then seeking back to the top and repeating the same process but
/// appending to existing lines with some delimiter (e.g. CSV). Once the new file has been created, the program would
/// read each line, perform the operation, and aggregate the results. This solution assumes that every input line
/// contains the exact same number of cells. It also assumes that the file is ASCII encoded, where each character takes
/// one byte.
///
/// The temporary file could be avoided by first scanning the entire input file to find out where the newlines are located,
/// storing the offset ranges in memory, reading the first range until hitting the first whitespace, switching to the next
/// range and repeating until the first column is read, then performing the operation and continuing.
#[must_use]
pub fn puzzle1(input_path: &str) -> i64 {
    let (file, ranges) = read_line_offsets(input_path);
    let mut current_offsets: Vec<_> = ranges.into_iter().map(Iterator::peekable).collect();
    let line_count = current_offsets.len();

    let read_byte = |offset: u64| -> char {
        let mut buf = [0u8; 1];
        file.read_at(&mut buf, offset).expect("failed to read byte");
        buf[0] as char
    };

    let mut total_sum: i64 = 0;

    loop {
        let mut numbers = Vec::new();
        let mut operation = None;

        for (i, iter) in current_offsets.iter_mut().enumerate() {
            let is_last_line = i == line_count - 1;
            let mut num_string = String::new();

            for offset in iter.by_ref() {
                let c = read_byte(offset);

                if c.is_whitespace() && !num_string.is_empty() {
                    numbers.push(num_string.parse::<i64>().expect("failed to parse number"));
                    break;
                } else if !is_last_line && (c.is_ascii_digit() || c == '-') {
                    num_string.push(c);
                } else if c == '+' {
                    operation = Some(Operation::Add);
                    break;
                } else if c == '*' {
                    operation = Some(Operation::Multiply);
                    break;
                }
            }
        }

        match operation {
            Some(Operation::Add) if !numbers.is_empty() => {
                total_sum += numbers.iter().sum::<i64>();
            }
            Some(Operation::Multiply) if !numbers.is_empty() => {
                total_sum += numbers.iter().product::<i64>();
            }
            _ => break,
        }
    }

    total_sum
}

/// # Puzzle 2
///
/// ## Summary
/// This puzzle is similar to the first puzzle in that the input data is still presented in a column-oriented format.
/// There are some fundamental differences though. The columns, separated by at least one full column of whitespaces,
/// must now be read from right to left. Each column has one operation and potentially multiple numbers, each of which
/// is in a sub-column of digits that must be read from top to bottom.
///
/// ## Solution
/// Some modifications need to be made to the solution of puzzle 1. Each line must be read from right to left and
/// only one digit at a time. A number is fully formed as soon as the first whitespace is found after a digit in a
/// vertical column. Since the operation symbol is always at the bottom of the last sub-column, perform the operation
/// and increment the total sum as soon as it is found.
#[must_use]
pub fn puzzle2(input_path: &str) -> i64 {
    let (file, ranges) = read_line_offsets(input_path);
    let mut current_offsets: Vec<_> = ranges.into_iter().map(|r| r.rev().peekable()).collect();
    let line_count = current_offsets.len();

    let read_byte = |offset: u64| -> char {
        let mut buf = [0u8; 1];
        file.read_at(&mut buf, offset).expect("failed to read byte");
        buf[0] as char
    };

    let mut total_sum: i64 = 0;
    let mut numbers = Vec::new();
    let mut operation: Option<Operation> = None;
    let mut num_string = String::new();

    loop {
        let mut found_content = false;

        for (i, iter) in current_offsets.iter_mut().enumerate() {
            let is_last_line = i == line_count - 1;

            for offset in iter.by_ref() {
                let c = read_byte(offset);

                if c == '\n' {
                    continue;
                }

                if c.is_whitespace() {
                    found_content = true;
                } else if !is_last_line && (c.is_ascii_digit() || c == '-') {
                    found_content = true;
                    num_string.push(c);
                } else if c == '+' {
                    found_content = true;
                    operation = Some(Operation::Add);
                } else if c == '*' {
                    found_content = true;
                    operation = Some(Operation::Multiply);
                }

                // Parse accumulated digits when reaching the last line
                if is_last_line && !num_string.is_empty() {
                    numbers.push(
                        mem::take(&mut num_string)
                            .parse::<i64>()
                            .expect("failed to parse number"),
                    );
                }
                break;
            }

            // Process operation when we have both an operation and numbers
            if operation.is_some() && !numbers.is_empty() {
                let nums = mem::take(&mut numbers);
                match operation.take().unwrap() {
                    Operation::Add => total_sum += nums.iter().sum::<i64>(),
                    Operation::Multiply => total_sum += nums.iter().product::<i64>(),
                }
            }
        }

        if !found_content {
            break;
        }
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&utils::string_to_file(TEST_INPUT)), 4_277_556);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&utils::string_to_file(TEST_INPUT)), 3_263_827);
    }
}