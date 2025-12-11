#[allow(dead_code)]

//! # Day 2 - Puzzle 1: Gift Shop ([challenge description](https://adventofcode.com/2025/day/2))
//!
//! ## Improvements Made
//!
//! - **Implemented `FromStr` trait**: Replaced custom `from_str` method with the standard library
//!   trait, enabling idiomatic `.parse()` syntax
//! - **Iterator chains over imperative loops**: Used `filter_map`, `flat_map`, `filter`, and `sum`
//!   for a more functional, declarative style
//! - **Removed redundant type annotations**: Let Rust's type inference work where possible
//! - **Extracted `digit_count` helper**: Reusable function clarifies intent and avoids repetition
//! - **Replaced `is_multiple_of(2)`**: This method is unstable; used `% 2 == 0` instead
//! - **Clearer variable names**: Renamed `oe_half_len` → `half_divisor`
//! - **Consistent formatting**: Used `','` and `'-'` char literals with `split`/`split_once`
//! - **Added `#[must_use]` attributes**: Warns if return values are accidentally ignored
//! - **Used `u128::from` instead of `as`**: Safer, explicit conversion that won't silently truncate
//! - **Simplified string formatting**: Used inline format syntax (`{s}` instead of `{}", s`)

use std::str::FromStr;

/// Returns the number of decimal digits in a positive integer.
///
/// Returns 1 for n = 0.
fn digit_count(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

/// # Puzzle 1
///
/// ## Summary
/// Given a comma-separated list of hyphen-separated consecutive number intervals, determine the
/// sum of numbers within all intervals that are composed of only two repeated sequences of digits.
/// Numbers with leading zeros must be ignored.
///
/// ## Solution
/// We need to sum all numbers in each range with an even length whose first half of digits is
/// equal to its second half, excluding numbers starting with 0. Start by splitting the input string
/// by commas. Then parse the number range into a struct that splits the string by a hyphen, makes
/// sure neither part starts with 0, parses them into integers, and checks that the second number is
/// larger than the first number. If a range could not be parsed, it will be skipped. Finally, for each
/// number in each range, add it to the total sum if the number's length is even ((log₁₀(n)+1) % 2 == 0),
/// and its first half (n / 10^(len/2)) is equal to its second half (n mod 10^(len/2)).
/// This solution avoids constantly converting between string and integer to assess symmetry.
#[must_use]
pub fn puzzle1(input: &str) -> u128 {
    input
        .split(',')
        .filter_map(|s| s.parse::<NumberRange>().ok())
        .filter(|range| {
            let from_len = digit_count(range.0);
            let to_len = digit_count(range.1);
            // Skip ranges where all numbers have the same odd digit count
            from_len != to_len || from_len % 2 == 0
        })
        .flat_map(|range| range.0..=range.1)
        .filter(|&n| {
            let len = digit_count(n);
            if len % 2 != 0 {
                return false;
            }
            let half_divisor = 10_u64.pow(len / 2);
            n / half_divisor == n % half_divisor
        })
        .map(u128::from)
        .sum()
}

/// # Puzzle 2
///
/// ## Summary
/// This is similar to Day 2 - Puzzle 1, but with a twist: Rather than numbers that only consist
/// of a sequence of digits repeated exactly twice, invalid IDs now include those with the digits
/// repeated at least twice.
///
/// ## Solution
/// Let's take a simpler, string-based approach this time. I originally had a different approach
/// similar to the Knuth-Morris-Pratt (KMP) algorithm based on comparing the prefixes at every index,
/// but found a much simpler approach that concatenates the string with itself, trims the first and last
/// digits, and checks if the original string appears in it. For example, given the number 565656,
/// concatenate it to form 565656565656 and then check if 565656 can be found in 6565656565.
#[must_use]
pub fn puzzle2(input: &str) -> u128 {
    input
        .split(',')
        .filter_map(|s| s.parse::<NumberRange>().ok())
        .flat_map(|range| range.0..=range.1)
        .filter(|&n| {
            let s = n.to_string();
            let doubled = format!("{s}{s}");
            doubled[1..doubled.len() - 1].contains(&s)
        })
        .map(u128::from)
        .sum()
}

/// A range of numbers represented as (start, end) inclusive.
#[derive(PartialEq, Eq, Debug)]
struct NumberRange(u64, u64);

impl FromStr for NumberRange {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (from_str, to_str) = input
            .split_once('-')
            .ok_or("number range must be delimited by a hyphen")?;

        if from_str.starts_with('0') || to_str.starts_with('0') {
            return Err("numbers in the range must not begin with 0");
        }

        let from: u64 = from_str
            .parse()
            .map_err(|_| "numbers in the range must be integers")?;
        let to: u64 = to_str
            .parse()
            .map_err(|_| "numbers in the range must be integers")?;

        if to <= from {
            return Err("the second number must be larger than the first");
        }

        Ok(NumberRange(from, to))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod number_range {
        use super::*;

        #[test]
        fn rejects_missing_delimiter() {
            assert!("invalid".parse::<NumberRange>().is_err());
        }

        #[test]
        fn rejects_empty_parts() {
            assert!("-".parse::<NumberRange>().is_err());
            assert!("1-".parse::<NumberRange>().is_err());
            assert!("-1".parse::<NumberRange>().is_err());
        }

        #[test]
        fn rejects_multiple_delimiters() {
            assert!("1-2-3".parse::<NumberRange>().is_err());
        }

        #[test]
        fn rejects_leading_zeros() {
            assert!("01-2".parse::<NumberRange>().is_err());
            assert!("1-02".parse::<NumberRange>().is_err());
        }

        #[test]
        fn rejects_invalid_ordering() {
            assert!("2-1".parse::<NumberRange>().is_err());
        }

        #[test]
        fn accepts_valid_range() {
            assert_eq!("11-22".parse(), Ok(NumberRange(11, 22)));
        }
    }

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
        446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(TEST_INPUT), 1227775554);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(TEST_INPUT), 4174379265);
    }
}