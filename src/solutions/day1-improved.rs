#[allow(dead_code)]

//! # Day 1: Secret Entrance ([challenge description](https://adventofcode.com/2025/day/1))
//!
//! ## Code Improvements
//!
//! The following idiomatic Rust improvements were applied:
//!
//! | Category | Before | After |
//! |----------|--------|-------|
//! | **Type modeling** | Inline parsing with magic numbers | `Direction` enum and `Rotation` struct encapsulate logic |
//! | **Function signatures** | `Vec<String>` (takes ownership) | `&[impl AsRef<str>]` (accepts `&[&str]`, `&[String]`, etc.) |
//! | **Preconditions** | `if cond { panic!(...) }` | `assert!(cond, "...")` macro |
//! | **Numeric conversions** | `as` casts (can truncate silently) | `i64::from()` (guaranteed lossless) |
//! | **Type annotations** | Explicit on every binding | Leverages type inference where clear |
//! | **Modular arithmetic** | `((x % 100) + 100) % 100` | `.rem_euclid(100)` |
//! | **Boolean logic** | `multiplier == -1 && ...` | `match` on `Direction` enum |
//! | **Test data** | Helper function to convert slices | `const` slice of `&str` literals |
//!
//! ### Key Takeaways
//!
//! - **Model your domain**: Small enums and structs make code self-documenting and catch errors at compile time.
//! - **Prefer borrowing**: Use `&[T]` or generics like `impl AsRef<str>` instead of owned `Vec<String>` when you only need to read data.
//! - **Use safe conversions**: `From`/`Into` traits guarantee correctness; `as` should be reserved for when you truly intend truncation.
//! - **Leverage the standard library**: Methods like `rem_euclid` exist precisely to handle common patterns cleanly.

/// Rotation direction on the dial
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    /// Parse direction from the first character of a rotation string
    fn parse(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("rotations must start with L or R"),
        }
    }

    /// Returns the multiplier for position calculation
    const fn multiplier(self) -> i64 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}

/// Parsed rotation instruction
#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: Direction,
    distance: i64,
}

impl Rotation {
    /// Parse a rotation string like "L68" or "R48"
    fn parse(s: &str) -> Self {
        let mut chars = s.chars();
        let direction = Direction::parse(chars.next().expect("empty rotation string"));
        let distance = chars
            .as_str()
            .parse()
            .expect("failed to parse distance as integer");

        Self { direction, distance }
    }
}

/// # Puzzle 1
///
/// ## Summary
/// There is a safe with a rotary lock that has a circular dial with the numbers 0 through 99.
/// To open the lock, which by default is set to 50, we need to process a series of consecutive rotations.
/// The rotations start with the letter L (left) or R (right), followed by an integer indicating the distance to rotate.
/// The actual password is the total number of times a rotation stops on the number 0.
///
/// ## Solution
/// The program starts with an initial dial position. It then parses each rotation, performs the
/// arithmetic based on the current dial position, and finally clamps the result such that it remains in the
/// range 0 to 99. If the rotation ends with the number 0, it should increment a counter, which is printed out
/// as the password after the last rotation is taken. This should have a time complexity of O(n).
///
/// ## Optimizations
/// Apart from batching and parallel processing, I can't think of a quicker way to do this, especially since
/// the result of each consecutive arithmetic operation needs to checked.
pub fn puzzle1(init_pos: u8, rotations: &[impl AsRef<str>]) -> u64 {
    assert!(init_pos <= 99, "the initial position must be within 0 to 99");

    let mut curr_pos = i64::from(init_pos);
    let mut zero_count = 0u64;

    for rotation in rotations {
        let rot = Rotation::parse(rotation.as_ref());
        curr_pos = (curr_pos + rot.direction.multiplier() * rot.distance).rem_euclid(100);

        if curr_pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

/// # Puzzle 2
///
/// ## Summary
/// This is similar to Puzzle 1, but with a twist: Rather than counting the number of times
/// a rotation ends in number 0, we should now count any time number 0 is crossed, whether during or
/// at the end of a rotation.
///
/// ## Solution
/// A solution could easily be found by drawing the lock and determining all the cases when 0 can be
/// pointed at. If the rotation distance is greater than 100, then 0 will be crossed every multiple of 100.
/// We should then look at the remainder of distance divided by 100, ignoring cases with a starting
/// position of 0, as they are included in the first calculation. If the direction of rotation is
/// counter-clockwise, a distance equal or greater than the remainder points at 0 once. For clockwise
/// rotations, a distance equal or greater than the sum of the current position and the remainder
/// points at 0 once.
pub fn puzzle2(init_pos: u8, rotations: &[impl AsRef<str>]) -> u64 {
    assert!(init_pos <= 99, "the initial position must be within 0 to 99");

    let mut curr_pos = i64::from(init_pos);
    let mut zero_count = 0u64;

    for rotation in rotations {
        let rot = Rotation::parse(rotation.as_ref());
        let full_rotations = rot.distance / 100;
        let remainder = rot.distance % 100;

        zero_count += full_rotations as u64;

        let crosses_zero = curr_pos != 0
            && match rot.direction {
                Direction::Left => remainder >= curr_pos,
                Direction::Right => curr_pos + remainder >= 100,
            };

        if crosses_zero {
            zero_count += 1;
        }

        curr_pos = (curr_pos + rot.direction.multiplier() * remainder).rem_euclid(100);
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_ROTATIONS: &[&str] = &[
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(50, SAMPLE_ROTATIONS), 3);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(50, SAMPLE_ROTATIONS), 6);
    }

    #[test]
    fn test_puzzle2_extended() {
        let rotations = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82", "L32", "R1000",
            "L1000", "R1", "R1000", "L234", "R32", "R1000", "R99", "R202",
        ];
        assert_eq!(puzzle2(50, &rotations), 54);
    }

    // Test with owned Strings to verify generic flexibility
    #[test]
    fn test_with_owned_strings() {
        let rotations: Vec<String> = SAMPLE_ROTATIONS.iter().map(|&s| s.to_owned()).collect();
        assert_eq!(puzzle1(50, &rotations), 3);
        assert_eq!(puzzle2(50, &rotations), 6);
    }
}