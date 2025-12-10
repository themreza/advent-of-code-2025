mod utils;

/// # Day 2 - Puzzle 1: Gift Shop ([challenge description](https://adventofcode.com/2025/day/2))
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
/// number in each range, add it to the total sum if the number's length is even ((log_10(n)+1) % 2 == 0),
/// and its first half (n/1e<len_n/2>) is equal to its second half (n mod 1e<len_n/2>).
/// This solution avoids constantly converting between string and integer to assess symmetry.
/// Some edge case number ranges to immediately ignore:
/// - those with a from and to number of odd and equal length (e.g. 111-999 and 11111-99999)
/// 
fn day_2_puzzle_1(input: &str) -> u128 {
    let mut sum: u128 = 0;
    let ranges: Vec<&str> = input.split(",").collect();
    for range_str in ranges {
        let Ok(range) = NumberRange::from_str(range_str) else {
            //println!("skipping {}", range_str);
            continue;
        };
        // Weird syntax: .. doesn't include the last number, but .= does :/
        for i in range.0..=range.1 {
            let len: u32  = i.checked_ilog10().unwrap_or(0) + 1;
            if !len.is_multiple_of(2) {
                //println!("skipping {}", i);
                continue;
            }
            let oe_half_len = 10_u64.pow(len/2);
            let first_half: u64 = i / oe_half_len;
            let second_half: u64 = i % oe_half_len;
            if first_half != second_half {
                //println!("skipping {}", i);
                continue;
            }
            //println!("invalid ID found: {}", i);
            sum += i as u128;
        }
    }
    sum
}

#[derive(PartialEq, Debug)]
struct NumberRange(u64, u64);

impl NumberRange {
    pub fn from_str(input: &str) -> Result<Self, &str> {
        // .ok_or() defines what error to return.
        // Adding ? to the end means return immediately in case of error, similar to if err != nil { ... } in Go
        let (from_str, to_str) = input.split_once("-").ok_or("number range must be delimited by a -")?;
        // Here, we can't use .ok_or(), because .parse() returns a Result<F, F::Err> whereas .split_once() returns an Option<&'_str,&'_str>.
        // The alternative approach is to use .map_err() with a Rust closure (anonymous function) that returns a static error.
        // I tried finding a quick way to retain the original message, but couldn't, so I just went with a static message.
        // I'm slowly starting to see how Rust's rather complex syntax can get in the way, but I suppose it's just part of the learning curve.
        if from_str.starts_with("0") || to_str.starts_with("0") {
            return Err("numbers in the ranger must not begin with a 0");
        }
        let from: u64 = from_str.parse().map_err(|_| "numbers in the range must be integers")?;
        let to: u64 = to_str.parse().map_err(|_| "numbers in the range must be integers")?;
        if to <= from {
            return Err("the second number in the range must be larger than the first");
        }
        let from_len: u32 = from.checked_ilog10().unwrap_or(0) + 1;
        let to_len: u32 = to.checked_ilog10().unwrap_or(0) + 1;
        if from_len == to_len && !from_len.is_multiple_of(2) {
            return Err("both numbers in the range must not be of the same odd length");
        }
        Ok(NumberRange(from, to))
    }
}

fn main() {
    let lines: Vec<String> = utils::lines_from_file("inputs/day-2-puzzle-1.txt");
    println!(
        "{}",
        day_2_puzzle_1(
            lines
                .first()
                .expect("input file must not be empty for day 2 puzzle 1")
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_puzzle_1_number_range_from_str() {
        assert_eq!(NumberRange::from_str("invalid"), Err("number range must be delimited by a -"), "an invalid input without a - must throw a parsing error");
        assert_eq!(NumberRange::from_str("-"), Err("numbers in the range must be integers"), "an invalid input without any numbers must throw a parsing error");
        assert_eq!(NumberRange::from_str("1-2-"), Err("numbers in the range must be integers"), "an invalid input with more than one - must throw a parsing error");
        assert_eq!(NumberRange::from_str("1-"), Err("numbers in the range must be integers"), "an invalid input with only the first number being valid must throw a parsing error");
        assert_eq!(NumberRange::from_str("-1"), Err("numbers in the range must be integers"), "an invalid input with only the second number being valid must throw a parsing error");
        assert_eq!(NumberRange::from_str("01-2"), Err("numbers in the ranger must not begin with a 0"), "an invalid input with the first number starting with a 0 must throw a parsing error");
        assert_eq!(NumberRange::from_str("1-02"), Err("numbers in the ranger must not begin with a 0"), "an invalid input with the second number starting with a 0 must throw a parsing error");
        assert_eq!(NumberRange::from_str("2-1"), Err("the second number in the range must be larger than the first"), "an invalid input with the second number larger than the first must throw a parsing error");
        assert_eq!(NumberRange::from_str("2-1"), Err("the second number in the range must be larger than the first"), "an invalid input with the second number larger than the first must throw a parsing error");
        assert_eq!(NumberRange::from_str("1-2"), Err("both numbers in the range must not be of the same odd length"), "an invalid input with both numbers being of the same odd length must throw a parsing error");
        assert_eq!(NumberRange::from_str("11-22"), Ok(NumberRange(11,22)), "a valid input must be parsed into NumberRange");      
    }

    #[test]
    fn test_day_2_puzzle_1() {
        assert_eq!(
            day_2_puzzle_1(
                &"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
        446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            1227775554
        );
    }
}
