#[allow(dead_code)]

//! # Day 4: Printing Department ([challenge description](https://adventofcode.com/2025/day/4))
//!
//! ## Changes Summary
//!
//! - Extracted direction offsets into a module-level `const`
//! - Created `count_adjacent` helper function to eliminate code duplication
//! - Replaced index-based loops with idiomatic `.enumerate()` iterators
//! - Used `checked_add_signed` with `Option` combinators for safe bounds checking
//! - Used `&[Vec<char>]` slice type in helper signature for flexibility
//! - Used `loop` with explicit `break` instead of `while` with boolean flag
//! - Replaced `if` + `panic!` with `assert!` for clearer input validation
//! - Removed unnecessary type annotations (e.g., `collect::<Vec<char>>()`)
//! - Removed redundant intermediate variables like `c_len`, `len_i`, `len_j`

/// Direction offsets for the 8 adjacent cells (row_offset, col_offset).
const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (-1, 1),
    (1, 1),
    (-1, -1),
    (1, -1),
];

/// Counts the number of adjacent '@' characters around a position.
fn count_adjacent(matrix: &[Vec<char>], row: usize, col: usize) -> usize {
    DIRECTIONS
        .iter()
        .filter(|(dr, dc)| {
            row.checked_add_signed(*dr)
                .zip(col.checked_add_signed(*dc))
                .and_then(|(r, c)| matrix.get(r)?.get(c))
                .is_some_and(|&ch| ch == '@')
        })
        .count()
}

/// # Puzzle 1
///
/// ## Summary
/// The puzzle's input is the textual grid diagram of the locations of a series
/// of paper rolls. The objective is to identify and mark all rolls of paper which
/// have fewer than 4 rolls in the 8 adjacent positions.
///
/// ## Solution
/// Given a grid formed by multiline a sequence of . and @ characters, find all @ characters
/// that have fewer than 4 adjacent @ characters, that is, check the perimeter of a box around
/// each @ character. Print the final string with those @ characters replaced with x characters.
/// Not every cell in the grid has 8 adjacent cells.
#[must_use]
pub fn puzzle1(input: &str) -> u128 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    assert!(!matrix.is_empty(), "input may not be empty");

    let mut count = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '@' && count_adjacent(&matrix, i, j) < 4 {
                count += 1;
            }
        }
    }
    count
}

/// # Puzzle 2
///
/// ## Summary
/// The program should now continue processing the grid and marking rolls of papers that can be
/// removed until no more
///
/// ## Solution
/// Add a while loop to continue checking until there are no new rolls of paper that can be removed.
/// In each iteration of the loop, update a temporary copy of the grid where @ characters are replaced
/// with x. At the end of the iteration, replace the original matrix with the temporary matrix.
#[must_use]
pub fn puzzle2(input: &str) -> u128 {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    assert!(!matrix.is_empty(), "input may not be empty");

    let mut total_count = 0;

    loop {
        let mut temp_matrix = matrix.clone();
        let mut changed = false;

        for (i, row) in matrix.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == '@' && count_adjacent(&matrix, i, j) < 4 {
                    temp_matrix[i][j] = 'x';
                    total_count += 1;
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
        matrix = temp_matrix;
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(TEST_INPUT), 13);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(TEST_INPUT), 43);
    }
}