//! # Day 6: Trash Compactor ([challenge description](https://adventofcode.com/2025/day/6))

use std::{fs::File, io::{BufRead, BufReader}, os::unix::fs::FileExt};

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
/// 
#[must_use]
pub fn puzzle1(input_path: &str) -> i64 {
    let mut input_file = File::open(input_path).expect("failed to read input for day 6");
    let mut reader = BufReader::new(input_file);
    let mut newline_offsets: Vec<u64> = vec![0];
    let mut buf = Vec::new();
    loop {
        buf.clear();
        let bytes_read = reader.read_until(b'\n', &mut buf).expect("failed to scan input file for newlines");
        if bytes_read == 0 {
            break;
        }
        newline_offsets.push(newline_offsets.last().unwrap() + bytes_read as u64);
    }
    let mut current_offsets: Vec<_> = newline_offsets
        .windows(2)
        .map(|w| w[0]..w[1])
        .collect();
    let offsets_len = current_offsets.len();
    input_file = reader.into_inner();
    let mut total_sum: i64 = 0;
    // Keep looping until no new column can be found
    loop {
        let mut numbers: Vec<i64> = vec![];
        let mut operation: Option<Operations> = None;
        // Scan for the next value in each line
        for (i, iter) in &mut current_offsets.iter_mut().enumerate() {
            let mut num_string = String::new();
            let is_last_offset = i == offsets_len - 1;
            // Keep reading the line until either one operation character or 
            // one or more digits (potentially negative) followed by a space are found
            for char_offset in iter.by_ref() {
                let mut buf = [0u8; 1];
                input_file.read_at(&mut buf, char_offset).expect("failed to read the next byte");
                let c = buf[0] as char;
                if c.is_whitespace() && !num_string.is_empty() {
                    numbers.push(num_string.parse::<i64>().expect("failed to parse number found in a cell"));
                    break;
                } else if !is_last_offset && (c.is_ascii_digit() || c == '-') {
                    num_string.push(c);
                } else if c == '+' {
                    operation = Some(Operations::Add);
                    break;
                } else if  c == '*' {
                    operation = Some(Operations::Multiply);
                    break;
                }
            }
        }
        if let Some(o) = operation && !numbers.is_empty() {
            match o {
                Operations::Add => {
                    let res: i64 = numbers.iter().sum();
                    total_sum += res;
                },
                Operations::Multiply => {
                    let res: i64 = numbers.iter().product();
                    total_sum += res;
                }
            }
        } else {
            // A column was fully scanned but no operation was found
            break;
        }
    }
    total_sum
}

#[derive(Debug)]
enum ArithmeticEnum {
    Add,
    Multiply,
}

type Operations = ArithmeticEnum;

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
/// 
#[must_use]
pub fn puzzle2(input_path: &str) -> i64 {
    let mut input_file = File::open(input_path).expect("failed to read input for day 6");
    let mut reader = BufReader::new(input_file);
    let mut newline_offsets: Vec<u64> = vec![0];
    let mut buf = Vec::new();
    loop {
        buf.clear();
        let bytes_read = reader.read_until(b'\n', &mut buf).expect("failed to scan input file for newlines");
        if bytes_read == 0 {
            break;
        }
        newline_offsets.push(newline_offsets.last().unwrap() + bytes_read as u64);
    }
    let mut current_offsets: Vec<_> = newline_offsets
        .windows(2)
        // Iterate in reverse order
        .map(|w| (w[0]..w[1]).rev())
        .collect();
    let offsets_len = current_offsets.len();
    input_file = reader.into_inner();
    let mut total_sum: i64 = 0;
    let mut keep_processing = true;
    let mut numbers: Vec<i64> = vec![];
    let mut operation: Option<Operations> = None;
    let mut num_string = String::new();
    // Keep processing columns and sub-columns until no new character is left to be parsed
    while keep_processing {
        keep_processing = false;
        // Read every line in a vertical fashion, one character at a time
        for (i, iter) in &mut current_offsets.iter_mut().enumerate() {
            let is_last_offset = i == offsets_len - 1;
            // Process the next character in a line, but skip newline characters
            for char_offset in iter.by_ref() {
                let mut buf = [0u8; 1];
                input_file.read_at(&mut buf, char_offset).expect("failed to read the next byte");
                let c = buf[0] as char;
                if c == '\n' {
                    continue;
                } else if c.is_whitespace() {
                    keep_processing = true;
                } else if !is_last_offset && (c.is_ascii_digit() || c == '-') {
                    keep_processing = true;
                    num_string.push(c);
                } else if c == '+' {
                    keep_processing = true;
                    operation = Some(Operations::Add);
                } else if  c == '*' {
                    keep_processing = true;
                    operation = Some(Operations::Multiply);
                }
                // Once the end of a sub-column is reached, parse the string into a number if possible
                if is_last_offset && !num_string.is_empty() {
                    numbers.push(num_string.parse::<i64>().expect("failed to parse number found in a cell"));
                    num_string = String::new();
                }
                break;
            }
            // An operation symbol is only available once all sub-columns have been processed,
            // at which point all the numbers will be added or multiplied together
            if let Some(ref o) = operation && !numbers.is_empty() {
                match o {
                    Operations::Add => {
                        let res: i64 = numbers.iter().sum();
                        total_sum += res;
                    },
                    Operations::Multiply => {
                        let res: i64 = numbers.iter().product();
                        total_sum += res;
                    }
                }
                numbers = vec![];
                operation = None;
            }
        }
        if !keep_processing {
            break;
        }
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&utils::string_to_file(TEST_INPUT)), 4277556);
    }

    #[test]
    fn test_puzzle2() {
       assert_eq!(puzzle2(&utils::string_to_file(TEST_INPUT)), 3263827);
    }
}