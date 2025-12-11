use aoc2025::solutions::day1;
use aoc2025::utils;

fn main() {
    let input = utils::lines_from_file("inputs/day1.txt");
    println!("Puzzle 1: {}", day1::puzzle1(50, input.clone()));
    println!("Puzzle 2: {}", day1::puzzle2(50, input));
}