#![allow(dead_code)]

mod utils;

/// # Day 2 - Puzzle 1 (AI Improved by Claude Opus 4.5)
/// 
/// It's worth mentioning that, when I provided the original challenge description,
/// it suggested a different algorithm where instead of looping through all numbers
/// in the range, we take a reverse approach and calculate all the possible invalid
/// IDs using an arithmetic series. I ignored that as the actual math is not my focus
/// here, but Rust is.
/// 
/// ## Changes Overview
/// 
/// | Category | Before | After | Benefit |
/// |----------|--------|-------|---------|
/// | **Collection strategy** | `split(",").collect::<Vec<_>>()` then `for` loop | Iterator chain with `.filter_map().flat_map().sum()` | Avoids heap allocation; lazy evaluation |
/// | **Character matching** | String slice `","` and `"0"` | Char literal `','` and `'0'` | Faster comparison; no slice overhead |
/// | **Struct fields** | Tuple struct `NumberRange(u64, u64)` accessed via `.0`, `.1` | Named fields `start`, `end` | Self-documenting; clearer intent |
/// | **Error type** | Implicit `&str` | Explicit `&'static str` | Clear lifetime; better API documentation |
/// | **Digit logic** | Inline repeated code | Extracted `digit_count()` and `is_repeated_halves()` | DRY; reusable; testable in isolation |
/// | **Compile-time eval** | Regular function | `const fn digit_count()` | Compiler can evaluate at compile time |
/// | **Inlining hints** | None | `#[inline]` on small hot-path functions | Suggests inlining to avoid call overhead |
/// | **Derive traits** | Only `PartialEq` | Added `Eq` | Idiomatic for types with reflexive equality |
/// | **Test structure** | Single large test with many assertions | Nested modules with focused single-assertion tests | Easier debugging; clearer failure messages |
/// | **Error assertions** | Exact string matching | `is_err()` checks | More maintainable; decoupled from message text |
/// 
/// ## Performance Impact
/// 
/// | Optimization | Impact Level | Notes |
/// |--------------|--------------|-------|
/// | Iterator chains vs collect | Medium | Eliminates `Vec` allocation; processes lazily |
/// | Char vs string literals | Low | Minor but measurable in tight loops |
/// | `#[inline]` hints | Low-Medium | Depends on compiler decisions; helps small functions |
/// | `const fn` | Low | Only helps if called with compile-time constants |
/// | Extracted functions | Neutral | No runtime cost; improves maintainability |
/// 
/// ## Readability Improvements
/// 
/// | Aspect | Improvement |
/// |--------|-------------|
/// | Function naming | `is_repeated_halves()` clearly describes intent |
/// | Field naming | `range.start` vs `range.0` is immediately understandable |
/// | Doc comments | Error conditions documented on `from_str` |
/// | Test organization | Grouped by unit under test (`mod number_range`) |
/// | Variable names | `half_divisor` explains purpose vs `oe_half_len` |
/// 
fn day_2_puzzle_1_improved(input: &str) -> u128 {
    input
        .split(',')  // Use char literal, not string slice - slightly more efficient
        .filter_map(|s| NumberRange::from_str(s).ok())
        .flat_map(|range| range.start..=range.end)
        .filter(|&n| is_repeated_halves(n))
        .map(u128::from)
        .sum()
}

/// Returns the number of decimal digits in `n`.
#[inline]
const fn digit_count(n: u64) -> u32 {
    // For n=0, ilog10 would panic, so we treat it as 1 digit
    match n.checked_ilog10() {
        Some(log) => log + 1,
        None => 1,
    }
}

/// Returns `true` if `n` has an even number of digits and its first half equals its second half.
/// For example: 1212 -> true (12 == 12), 1234 -> false (12 != 34), 123 -> false (odd length)
#[inline]
fn is_repeated_halves(n: u64) -> bool {
    let len = digit_count(n);
    if len % 2 != 0 {
        return false;
    }

    let half_divisor = 10_u64.pow(len / 2);
    let first_half = n / half_divisor;
    let second_half = n % half_divisor;

    first_half == second_half
}

#[derive(PartialEq, Eq, Debug)]
struct NumberRange {
    start: u64,
    end: u64,
}

impl NumberRange {
    /// Parses a string like "11-99" into a `NumberRange`.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The string doesn't contain exactly one hyphen
    /// - Either number starts with '0'
    /// - Either part isn't a valid integer
    /// - The end is not greater than the start
    /// - Both numbers have the same odd digit count (optimization: no valid numbers exist)
    fn from_str(input: &str) -> Result<Self, &'static str> {
        let (start_str, end_str) = input
            .split_once('-')
            .ok_or("number range must be delimited by a '-'")?;

        // Check for leading zeros before parsing
        if start_str.starts_with('0') || end_str.starts_with('0') {
            return Err("numbers in the range must not begin with '0'");
        }

        let start: u64 = start_str
            .parse()
            .map_err(|_| "numbers in the range must be valid integers")?;

        let end: u64 = end_str
            .parse()
            .map_err(|_| "numbers in the range must be valid integers")?;

        if end <= start {
            return Err("end of range must be greater than start");
        }

        // Optimization: skip ranges where no valid repeated-halves numbers can exist
        let start_len = digit_count(start);
        let end_len = digit_count(end);
        if start_len == end_len && start_len % 2 != 0 {
            return Err("range contains only odd-length numbers (no valid candidates)");
        }

        Ok(Self { start, end })
    }
}

fn main() {
    let lines = utils::lines_from_file("inputs/day-2-puzzle-1.txt");
    let input = lines
        .first()
        .expect("input file must not be empty for day 2 puzzle 1");

    println!("{}", day_2_puzzle_1_improved(input));
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
            assert!(NumberRange::from_str("5-5").is_err());
        }

        #[test]
        fn rejects_same_odd_length() {
            assert!(NumberRange::from_str("1-2").is_err());
            assert!(NumberRange::from_str("100-999").is_err());
        }

        #[test]
        fn accepts_valid_range() {
            assert_eq!(
                NumberRange::from_str("11-22"),
                Ok(NumberRange { start: 11, end: 22 })
            );
        }
    }

    mod is_repeated_halves {
        use super::*;

        #[test]
        fn rejects_odd_length() {
            assert!(!is_repeated_halves(1));
            assert!(!is_repeated_halves(123));
        }

        #[test]
        fn rejects_non_matching_halves() {
            assert!(!is_repeated_halves(12));
            assert!(!is_repeated_halves(1234));
        }

        #[test]
        fn accepts_matching_halves() {
            assert!(is_repeated_halves(11));
            assert!(is_repeated_halves(1212));
            assert!(is_repeated_halves(123123));
        }
    }

    #[test]
    fn test_day_2_puzzle_1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                     1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                     824824821-824824827,2121212118-2121212124";

        assert_eq!(day_2_puzzle_1(input), 1227775554);
    }
}