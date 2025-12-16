#![allow(clippy::all)]

//! # Day 1: Secret Entrance ([challenge description](https://adventofcode.com/2025/day/1))

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
/// 
pub fn puzzle1(init_pos: u8, rotations: Vec<String>) -> u64 {
    if init_pos > 99 {
        panic!("the initial position must be within 0 to 99");
    }
    let mut curr_pos: i64 = init_pos as i64;
    let mut zero_count: u64 = 0;
    for rotation in rotations.iter() {
        let mut chars: std::str::Chars<'_> = rotation.chars();
        let multiplier: i64 = match chars.next() {
            Some('L') => {
                -1
            }
            Some('R') => {
                1
            }
            _ => panic!("rotations must start with L or R"),
        };
        let dist: i64 = chars
            .as_str()
            .parse::<u64>()
            .expect("Failed to parse distance as an integer") as i64;
        curr_pos = ((((curr_pos) + (multiplier * dist)) % 100) + 100) % 100;
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
///
pub fn puzzle2(init_pos: u8, rotations: Vec<String>) -> u64 {
    if init_pos > 99 {
        panic!("the initial position must be within 0 to 99");
    }
    let mut curr_pos: i64 = init_pos as i64;
    let mut zero_count: u64 = 0;
    for rotation in rotations.iter() {
        let mut chars: std::str::Chars<'_> = rotation.chars();
        let multiplier: i64 = match chars.next() {
            Some('L') => {
                -1
            }
            Some('R') => {
                1
            }
            _ => panic!("rotations must start with L or R"),
        };
        let dist: i64 = chars
            .as_str()
            .parse::<u64>()
            .expect("Failed to parse distance as an integer") as i64;
        let quot: i64 = dist / 100;
        let rem: i64 = dist % 100;
        zero_count += quot as u64;
        if curr_pos != 0
            && (multiplier == -1 && rem >= curr_pos || multiplier == 1 && curr_pos + rem >= 100) {
                zero_count += 1;
            }
        curr_pos = (curr_pos + multiplier * rem).rem_euclid(100);
    }
    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_puzzle1() {
        assert_eq!(
            puzzle1(
                50,
                utils::str_slice_to_vec_string(&[
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
                ])
            ),
            3
        );
    }

     #[test]
    fn test_puzzle2() {
        assert_eq!(
            puzzle2(
                50,
                utils::str_slice_to_vec_string(&[
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
                ])
            ),
            6
        );
    }

    #[test]
    fn test_puzzle2_extended() {
        assert_eq!(
            puzzle2(
                50,
                utils::str_slice_to_vec_string(&[
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82", "L32",
                    "R1000", "L1000", "R1", "R1000", "L234", "R32", "R1000", "R99", "R202"
                ])
            ),
            54
        );
    }
}
