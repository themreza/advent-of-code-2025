//! # Day 4: Printing Department ([challenge description](https://adventofcode.com/2025/day/4))

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
/// 
#[must_use]
pub fn puzzle1(input: &str) -> u128 {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|r| r.chars().collect::<Vec<char>>())
        .collect();
    let mut final_count = 0u128;
    // tuples represent offsets to check (row_offset, col_offset)
    // right and down are positive
    let check_coordinates = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, 1),
        (1, 1),
        (-1, -1),
        (1, -1)
    ];
    let c_len = check_coordinates.len();
    let len_i = matrix.len() as i128;
    if len_i == 0 {
        panic!("input may not be empty")
    }
    let len_j = matrix.first().map(|r| r.len()).unwrap_or(0) as i128;
    for i in 0..len_i {
        for j in 0..len_j {
            // usize is really messing things up :/
            if matrix[i as usize][j as usize] != '@' {
                continue;
            }
            let mut count = 0u8;
            for c in 0..c_len {
                let i2 = i + check_coordinates[c].0;
                let j2 = j + check_coordinates[c].1;
                if i2 < 0 || i2 >= len_i || j2 < 0 || j2 >= len_j {
                    continue;
                }
                if matrix[i2 as usize][j2 as usize] == '@' {
                    count += 1;
                }
            }
            if count < 4 {
                final_count += 1;
            }
        }
    }
    final_count
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
/// 
#[must_use]
pub fn puzzle2(input: &str) -> u128 {
    let mut matrix: Vec<Vec<char>> = input
        .lines()
        .map(|r| r.chars().collect::<Vec<char>>())
        .collect();
    let mut temp_matrix = matrix.clone();
    let mut final_count = 0u128;
    let mut keep_checking = true;
    // tuples represent offsets to check (row_offset, col_offset)
    // right and down are positive
    let check_coordinates = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, 1),
        (1, 1),
        (-1, -1),
        (1, -1)
    ];
    let c_len = check_coordinates.len();
    let len_i = matrix.len() as i128;
    if len_i == 0 {
        panic!("input may not be empty")
    }
    let len_j = matrix.first().map(|r| r.len()).unwrap_or(0) as i128;

    while keep_checking {
        keep_checking = false;
        for i in 0..len_i {
            for j in 0..len_j {
                // usize is really messing things up :/
                if matrix[i as usize][j as usize] != '@' {
                    continue;
                }
                let mut count = 0u8;
                for c in 0..c_len {
                    let i2 = i + check_coordinates[c].0;
                    let j2 = j + check_coordinates[c].1;
                    if i2 < 0 || i2 >= len_i || j2 < 0 || j2 >= len_j {
                        continue;
                    }
                    if matrix[i2 as usize][j2 as usize] == '@' {
                        count += 1;
                    }
                }
                if count < 4 {
                    final_count += 1;
                    temp_matrix[i as usize][j as usize] = 'x';
                    keep_checking = true;
                }
            }
        }
        matrix = temp_matrix.clone();
    }
    final_count
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