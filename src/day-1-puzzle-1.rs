mod utils;

/// # Day 1, Puzzle 1: Secret Entrance ([challenge description](https://adventofcode.com/2025/day/1))
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
fn day_1_puzzle_1(init_pos: u8, rotations: Vec<String>) -> u64 {
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
        curr_pos = ((((curr_pos) + (multiplier * dist)) % 100) + 100) % 100;
        if curr_pos == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

#[allow(dead_code)]
// Same as above, but AI code reviewed
fn day_1_puzzle_1_improved(init_pos: u8, rotations: Vec<String>) -> u64 {
    assert!(init_pos <= 99, "initial position must be within 0 to 99");

    let mut curr_pos = init_pos as i64;
    let mut zero_count = 0;

    for rotation in rotations {
        let (dir, dist_str) = rotation.split_at(1);
        let dist: i64 = dist_str.parse().expect("distance must be an integer");

        let multiplier = match dir {
            "L" => -1,
            "R" => 1,
            _ => panic!("rotation must start with L or R"),
        };

        curr_pos = (curr_pos + multiplier * dist).rem_euclid(100);

        if curr_pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn main() {
    let lines = utils::lines_from_file("inputs/day-1-puzzle-1.txt");
    println!("{}", day_1_puzzle_1(50, lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_puzzle_1() {
        assert_eq!(
            day_1_puzzle_1(
                50,
                utils::str_slice_to_vec_string(&[
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
                ])
            ),
            3
        );
    }

    #[test]
    fn test_day_1_puzzle_1_improved() {
        assert_eq!(
            day_1_puzzle_1_improved(
                50,
                utils::str_slice_to_vec_string(&[
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
                ])
            ),
            3
        );
    }
}
