#[allow(dead_code)]

//! # Day 3: Lobby ([challenge description](https://adventofcode.com/2025/day/3))
//!
//! ## Summary of Changes
//!
//! ### Efficiency
//!
//! - Replaced `format!`/`parse` with arithmetic for number construction
//! - Used `fold` to build the 12-digit number arithmetically in puzzle2
//!
//! ### Idiomatic Rust
//!
//! - Removed unnecessary explicit type annotations (leveraging Rust's type inference)
//! - Used `enumerate()` instead of manual index tracking with `0..=last_i`
//! - Used underscore separators in numeric literals for readability
//! - Moved constant to top of module for visibility
//! - Simplified loop bounds (`0..BATTERY_BANK_SIZE` instead of `0..=last_j`)
//! - Removed redundant `.map(u128::from)` (filter_map already yields `u128`)
//!
//! ### Readability
//!
//! - Renamed variables for clarity (`arr` → `result`, `s` → `line`, `d1_max_i` → `max_first_idx`)
//! - Simplified bounds calculation in puzzle2 with early return guard
//!
//! ### Robustness
//!
//! - Added early return for lines with insufficient digits (prevents potential underflow)
//! - Removed error printing that returned `Some(0)` — invalid lines are now skipped

const BATTERY_BANK_SIZE: usize = 12;

/// # Puzzle 1
///
/// ## Summary
/// Given an input file with a sequence of digits on every line, determine the largest number that can be formed
/// in each line by taking any two digits from that sequence in the same order, and calculate the sum of numbers
/// formed from all lines. For example, if a line contains the digits 234234234234278, the largest number formable
/// for this line is 78 (87 is not possible since that would break the order of digits).
///
/// ## Solution
/// Convert every line into an array of integers. Create two 0-valued integer variables for each digit of the number
/// to be formed. Loop through each integer. If a number (excluding the last number) larger than the current first digit
/// is found, replace it with the first digit and set the second digit to 0. Otherwise, replace the number with the current
/// second digit if it's larger. Finally, increment the total sum with the number formed for that line.
#[must_use]
pub fn puzzle1(input: &str) -> u128 {
    input
        .lines()
        .filter_map(|line| {
            let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            if digits.len() < 2 {
                return None;
            }

            let mut result = [0_u32; 2];
            let max_first_idx = digits.len() - 2;

            for (i, &digit) in digits.iter().enumerate() {
                if digit > result[0] && i <= max_first_idx {
                    result[0] = digit;
                    result[1] = 0;
                } else if digit > result[1] {
                    result[1] = digit;
                }
            }

            Some(u128::from(result[0]) * 10 + u128::from(result[1]))
        })
        .sum()
}

/// # Puzzle 2
///
/// ## Summary
/// The number of batteries to turn on has increased from 2 to 12. The rules and the objective are still the same: Find the
/// largest number that can be formed by taking 12 digits from left to right in the same order. If there are more than 12
/// digits in a line, some may be skipped, leaving digits that result in the largest possible number.
///
/// ## Solution
/// We need to generalize the answer to the first puzzle by creating an array of final digits equal to the maximum battery
/// count in each bank. Each digit of the largest number for a line may only be replaced up to a certain index in the line,
/// such that there would be at least (battery_bank_size - digit_index) more integers left to complete the digits from the
/// point of replacement. For example, if the battery bank size is 12 and the line has 18 numbers in it, the first digit
/// of the 12-digit number to be formed can only be set up to the 6th line digit, so that there would still be 12 more digits
/// left to finish the number.
#[must_use]
pub fn puzzle2(input: &str) -> u128 {
    input
        .lines()
        .filter_map(|line| {
            let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            if digits.len() < BATTERY_BANK_SIZE {
                return None;
            }

            let mut result = [0_u32; BATTERY_BANK_SIZE];
            let max_indices: [usize; BATTERY_BANK_SIZE] =
                std::array::from_fn(|i| digits.len() - (BATTERY_BANK_SIZE - i));

            'outer: for (i, &digit) in digits.iter().enumerate() {
                for j in 0..BATTERY_BANK_SIZE {
                    if digit > result[j] && i <= max_indices[j] {
                        result[j] = digit;
                        if j < BATTERY_BANK_SIZE - 1 {
                            result[j + 1] = 0;
                        }
                        continue 'outer;
                    }
                }
            }

            Some(
                result
                    .iter()
                    .fold(0_u128, |acc, &d| acc * 10 + u128::from(d)),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(TEST_INPUT), 357);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(TEST_INPUT), 3_121_910_778_619);
    }
}