mod utils;

/// # Day 2 - Puzzle 2: Gift Shop ([challenge description](https://adventofcode.com/2025/day/2))
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
fn day_2_puzzle_2(input: &str) -> u128 {
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
        let (from_str, to_str) = input
            .split_once('-')
            .ok_or("number range must be delimited by a -")?;
        if from_str.starts_with('0') || to_str.starts_with('0') {
            return Err("numbers in the ranger must not begin with a 0");
        }
        let from: u64 = from_str
            .parse()
            .map_err(|_| "numbers in the range must be integers")?;
        let to: u64 = to_str
            .parse()
            .map_err(|_| "numbers in the range must be integers")?;
        if to < from {
            return Err("the second number in the range must be larger than the first");
        }
        Ok(NumberRange(from, to))
    }
}

fn main() {
    // Same input for both puzzles
    let lines: Vec<String> = utils::lines_from_file("inputs/day-2-puzzle-1.txt");
    println!(
        "{}",
        day_2_puzzle_2(
            lines
                .first()
                .expect("input file must not be empty for day 2 puzzle 1")
        )
    );
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

    #[test]
    fn test_day_2_puzzle_2() {
        assert_eq!(
            day_2_puzzle_2(
                &"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
        446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            4174379265
        );
    }
}
