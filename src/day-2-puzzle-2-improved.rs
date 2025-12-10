#![allow(dead_code)]

mod utils;

use std::str::FromStr;

/// # Day 2 - Puzzle 2: Gift Shop (AI improved by Claude Opus 4.5)
/// 
/// # Changes Overview
/// 
/// | # | Category | Before | After | Why It's Better |
/// |---|----------|--------|-------|-----------------|
/// | 1 | Trait implementation | `pub fn from_str()` method | `impl FromStr for NumberRange` | Enables idiomatic `.parse()` syntax and integrates with Rust's standard parsing ecosystem |
/// | 2 | Struct design | `struct NumberRange(u64, u64)` | `struct NumberRange { start: u64, end: u64 }` | Named fields are self-documenting; avoids cryptic `.0` and `.1` access |
/// | 3 | Control flow | Imperative `for` loop with mutable `sum` | Iterator chain with `.filter_map()`, `.flat_map()`, `.filter()`, `.sum()` | More declarative, eliminates mutation, clearly expresses data transformation pipeline |
/// | 4 | Code organization | Inline pattern-check logic | Extracted `has_repeating_pattern()` function | Improves readability and makes the algorithm self-documenting |
/// | 5 | Type conversion | `i as u128` | `u128::from(n)` | Explicit, safe conversion; compiler-verified to be lossless |
/// | 6 | Self-reference | `Ok(NumberRange { ... })` | `Ok(Self { ... })` | More idiomatic in impl blocks; easier refactoring if type name changes |
/// | 7 | Trait derivation | `#[derive(PartialEq, Debug)]` | `#[derive(PartialEq, Eq, Debug)]` | `Eq` should be derived when possible; enables use in more contexts (e.g., `HashMap` keys) |
/// | 8 | Type annotations | `let mut sum: u128 = 0` | Let compiler infer | Reduces noise; Rust's type inference handles obvious cases |
/// | 9 | String formatting | `format!("{}{}", i_str, i_str)` | `format!("{s}{s}")` | Inline format syntax (Rust 1.58+) is more concise |
/// | 10 | Test syntax | `NumberRange::from_str("11-22")` | `"11-22".parse()` | Leverages `FromStr` trait; more idiomatic and concise |
/// | 11 | Typo fix | `"numbers in the ranger must..."` | `"numbers in the range must..."` | Corrects spelling error in error message |
/// | 12 | Variable naming | `from`, `to` | `start`, `end` | More conventional for range bounds in Rust |
/// 
fn day_2_puzzle_2(input: &str) -> u128 {
    input
        .split(',')
        .filter_map(|s| s.parse::<NumberRange>().ok())
        .flat_map(|range| range.start..=range.end)
        .filter(|&n| has_repeating_pattern(n))
        .map(u128::from)
        .sum()
}

fn has_repeating_pattern(n: u64) -> bool {
    let s = n.to_string();
    let doubled = format!("{s}{s}");
    doubled[1..doubled.len() - 1].contains(&s)
}

#[derive(PartialEq, Eq, Debug)]
struct NumberRange {
    start: u64,
    end: u64,
}

impl FromStr for NumberRange {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (from_str, to_str) = input
            .split_once('-')
            .ok_or("number range must be delimited by a -")?;

        if from_str.starts_with('0') || to_str.starts_with('0') {
            return Err("numbers in the range must not begin with a 0");
        }

        let start = from_str
            .parse()
            .map_err(|_| "numbers in the range must be integers")?;
        let end = to_str
            .parse()
            .map_err(|_| "numbers in the range must be integers")?;

        if end < start {
            return Err("the second number in the range must be larger than the first");
        }

        Ok(Self { start, end })
    }
}

fn main() {
    let lines = utils::lines_from_file("inputs/day-2-puzzle-1.txt");
    let input = lines
        .first()
        .expect("input file must not be empty for day 2 puzzle 1");

    println!("{}", day_2_puzzle_2(input));
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
            assert_eq!(
                "11-22".parse(),
                Ok(NumberRange { start: 11, end: 22 })
            );
        }
    }

    #[test]
    fn test_day_2_puzzle_2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                     1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                     824824821-824824827,2121212118-2121212124";
        assert_eq!(day_2_puzzle_2(input), 4174379265);
    }
}