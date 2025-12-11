//! # Day 2: Gift Shop ([challenge description](https://adventofcode.com/2025/day/2))

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
/// number in each range, add it to the total sum if the number's length is even ((log_10(n)+1) % 2 == 0),
/// and its first half (n/1e<len_n/2>) is equal to its second half (n mod 1e<len_n/2>).
/// This solution avoids constantly converting between string and integer to assess symmetry.
/// 
pub fn puzzle1(input: &str) -> u128 {
    let mut sum: u128 = 0;
    let ranges: Vec<&str> = input.split(",").collect();
    for range_str in ranges {
        let Ok(range) = NumberRange::from_str(range_str) else {
            //println!("skipping {}", range_str);
            continue;
        };
        let from_len: u32 = range.0.checked_ilog10().unwrap_or(0) + 1;
        let to_len: u32 = range.1.checked_ilog10().unwrap_or(0) + 1;
        if from_len == to_len && !from_len.is_multiple_of(2) {
            continue;
        }
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
/// but found a much simpler approach that concatenates the string with itself, trim the first and last
/// digits, and check if the original string appears in it. For example, given the number 565656, 
/// concatenate it to form 565656565656 and then check if 565656 can be found in 6565656565.
/// 
pub fn puzzle2(input: &str) -> u128 {
     let mut sum: u128 = 0;
    let ranges: Vec<&str> = input.split(',').collect();
    for range_str in ranges {
        let Ok(range) = NumberRange::from_str(range_str) else {
            continue;
        };
        for i in range.0..=range.1 {
            let i_str: String = i.to_string();
            let concat_str = format!("{}{}", i_str, i_str);
            if concat_str[1..concat_str.len()-1].contains(&i_str) {
                sum += i as u128;
            }
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
        // "" should be used for strings and '' for literal characters
        if from_str.starts_with('0') || to_str.starts_with('0') {
            return Err("numbers in the ranger must not begin with a 0");
        }
        let from: u64 = from_str.parse().map_err(|_| "numbers in the range must be integers")?;
        let to: u64 = to_str.parse().map_err(|_| "numbers in the range must be integers")?;
        if to <= from {
            return Err("the second number in the range must be larger than the first");
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
            assert!(NumberRange::from_str("invalid").is_err());
        }

        #[test]
        fn rejects_empty_parts() {
            assert!(NumberRange::from_str("-").is_err());
            assert!(NumberRange::from_str("1-").is_err());
            assert!(NumberRange::from_str("-1").is_err());
        }

        #[test]
        fn rejects_multiple_delimiters() {
            assert!(NumberRange::from_str("1-2-3").is_err());
        }

        #[test]
        fn rejects_leading_zeros() {
            assert!(NumberRange::from_str("01-2").is_err());
            assert!(NumberRange::from_str("1-02").is_err());
        }

        #[test]
        fn rejects_invalid_ordering() {
            assert!(NumberRange::from_str("2-1").is_err());
        }

        #[test]
        fn accepts_valid_range() {
            assert_eq!(
                NumberRange::from_str("11-22"),
                Ok(NumberRange(11, 22))
            );
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

