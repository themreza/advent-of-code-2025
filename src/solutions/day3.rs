//! # Day 3: Lobby ([challenge description](https://adventofcode.com/2025/day/3))

/// # Puzzle 1
/// 
/// ## Summary
/// Given an input file with a sequence of digits on every line, determine the largest number that can be formed
/// in each line by taking any two digits from that sequence in the same order, and calculate the sum of numbers 
/// formed from all lines. For example, if a line contains the digits 234234234234278, the largest number formable 
/// for this line is 78 (87 is not possible since that would break the order of digits).
/// 
/// ## Solution
/// Convert every line into an array of integers. Create two 0-valued integer variables for each digit of the number 
/// to be formed. Loop through each integer. If a number (excluding the last number) larger than the current first digit 
/// is found, replace it with the first digit and set the second digit to 0. Otherwise, replace the number with the current 
/// second digit if it's larger. Finally, increment the total sum with the number formed for that line.
/// 
#[must_use]
pub fn puzzle1(input: &str) -> u128 {
    input
        .lines()
        .filter_map(|s| {
            let digits: Vec<u32> = s
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect();
            let mut arr: [u32; 2] = [0, 0];
            let last_i: usize = digits.len()-1;
            let d1_max_i: usize = last_i - 1;
            for i in 0..=last_i {
                let digit: u32 = digits[i];
                if digit > arr[0] && i <= d1_max_i {
                    arr[0] = digit;
                    arr[1] = 0;
                } else if digit > arr[1] {
                    arr[1] = digit;
                }
            }
            match format!("{}{}", arr[0], arr[1]).parse::<u128>() {
                Ok(num) => Some(num),
                Err(e) => {
                    println!("failed to parse integer from number string: {}", e);
                    Some(0)
                }
            }
        })
        .map(u128::from)
        .sum()
}

const BATTERY_BANK_SIZE: usize = 12;

/// # Puzzle 2
/// 
/// ## Summary
/// The number of batteries to turn on has increased from 2 to 12. The rules and the objective are still the same: Find the
/// largest number that can be formed by taking 12 digits from left to right in the same order. If there are more than 12
/// digits in a line, some may be skipped, leaving digits that result in the largest possible number.
/// 
/// ## Solution
/// We need to generalize the answer to the first puzzle by creating an array of final digits equal to the maximum battery
/// count in each bank. Each digit of the largest number for a line may only be replaced up to a certain index in the line,
/// such that there would be at least (battery_bank_size - digit_index) more integers left to complete the digits from the
/// point of replacement. For example, if the battery bank size is 12 and the line has 18 numbers in it, the first digit
/// of the 12-digit number to be formed can only be set up to the 6th line digit, so that there would still be 12 more digits
/// left to finish the number.
#[must_use]
pub fn puzzle2(input: &str) -> u128 {
    input
        .lines()
        .filter_map(|s| {
            let digits: Vec<u32> = s
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect();
            let mut arr: [u32; BATTERY_BANK_SIZE] = [0; BATTERY_BANK_SIZE];
            let last_i: usize = digits.len()-1;
            let dx_max_i: [usize; BATTERY_BANK_SIZE] = std::array::from_fn(|i| {
                let max: i32 = last_i as i32 - (BATTERY_BANK_SIZE as i32 - i as i32 - 1);
                if max >= 0 {
                    max as usize
                } else {
                    0
                }
            });
            let last_j: usize = BATTERY_BANK_SIZE-1;
            'outer: for i in 0..=last_i {
                let digit: u32 = digits[i];
                for j in 0..=last_j {
                    if digit > arr[j] && i <= dx_max_i[j] {
                        arr[j] = digit;
                        if j < last_j {
                            arr[j+1] = 0;
                            continue 'outer;
                        }
                    }
                }
            }
            match arr.iter().map(|&num| num.to_string()).collect::<String>().parse::<u128>() {
                Ok(num) => Some(num),
                Err(e) => {
                    println!("failed to parse integer from number string: {}", e);
                    Some(0)
                }
            }
        })
        .map(u128::from)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(TEST_INPUT), 357);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(TEST_INPUT), 3121910778619);
    }
}