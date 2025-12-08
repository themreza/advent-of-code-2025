mod utils;

/// # Day 1, Puzzle 2: Secret Entrance ([challenge description](https://adventofcode.com/2025/day/1))
///
/// ## Summary
/// This is similar to Day 1, Puzzle 1, but with a twist: Rather than counting the number of times
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
fn day_1_puzzle_2(init_pos: u8, rotations: Vec<String>) -> u64 {
    if init_pos > 99 {
        panic!("the initial position must be within 0 to 99");
    }
    let mut curr_pos: i64 = init_pos as i64;
    let mut zero_count: u64 = 0;
    for rotation in rotations.iter() {
        let mut chars: std::str::Chars<'_> = rotation.chars();
        let multiplier: i64;
        match chars.next() {
            Some('L') => {
                multiplier = -1;
            }
            Some('R') => {
                multiplier = 1;
            }
            _ => panic!("rotations must start with L or R"),
        }
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

#[allow(dead_code)]
/// Same as above, but AI code reviewed by Claude Opus 4.5
///
/// Optimizations:
///
/// | Change | Benefit |
/// |--------|---------|
/// | `&[String]` parameter | No ownership transfer needed |
/// | `as_bytes()` + `bytes.first()` or `split_at(1)` | Faster than UTF-8 char iteration |
/// | `rotation[1..]` slice or `split_at(1)` | Direct slice vs iterator consumption |
/// | Parse directly as `i64` | Fewer casts |
/// | Unified crossing check (`new_pos <= 0 \|\| new_pos >= 100`) | Eliminates branching on direction; cleaner logic |
/// | Removed explicit `return` | Idiomatic Rust |
/// | Removed redundant type annotations | Cleaner code |
///
fn day_1_puzzle_2_improved(init_pos: u8, rotations: &[String]) -> u64 {
    assert!(init_pos <= 99, "initial position must be 0-99");

    let mut curr_pos = i64::from(init_pos);
    let mut zero_count = 0u64;

    for rotation in rotations {
        let (dir, dist_str) = rotation.split_at(1);
        let multiplier: i64 = if dir == "L" { -1 } else { 1 };
        let dist: i64 = dist_str.parse().expect("invalid distance");

        // Full rotations each cross zero once
        zero_count += (dist / 100) as u64;

        // Check if remainder crosses zero
        let rem = dist % 100;
        if curr_pos != 0 {
            let new_pos = curr_pos + multiplier * rem;
            if new_pos <= 0 || new_pos >= 100 {
                zero_count += 1;
            }
        }

        curr_pos = (curr_pos + multiplier * rem).rem_euclid(100);
    }

    zero_count
}

fn main() {
    // Same input for both puzzles
    let lines = utils::lines_from_file("inputs/day-1-puzzle-1.txt");
    println!("{}", day_1_puzzle_2(50, lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_puzzle_2() {
        assert_eq!(
            day_1_puzzle_2(
                50,
                utils::str_slice_to_vec_string(&[
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
                ])
            ),
            6
        );
    }

    #[test]
    fn test_day_1_puzzle_2_extended() {
        assert_eq!(
            day_1_puzzle_2(
                50,
                utils::str_slice_to_vec_string(&[
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82", "L32",
                    "R1000", "L1000", "R1", "R1000", "L234", "R32", "R1000", "R99", "R202"
                ])
            ),
            54
        );
    }

    #[test]
    fn test_day_1_puzzle_2_improved() {
        assert_eq!(
            day_1_puzzle_2_improved(
                50,
                &utils::str_slice_to_vec_string(&[
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
                ])
            ),
            6
        );
    }

    #[test]
    fn test_day_1_puzzle_2_improved_extended() {
        assert_eq!(
            day_1_puzzle_2_improved(
                50,
                &utils::str_slice_to_vec_string(&[
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82", "L32",
                    "R1000", "L1000", "R1", "R1000", "L234", "R32", "R1000", "R99", "R202"
                ])
            ),
            54
        );
    }
}
