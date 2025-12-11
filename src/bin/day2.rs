use aoc2025::solutions::day2;
use aoc2025::utils;

fn main() {
    let lines = utils::lines_from_file("inputs/day2.txt");
    let input = lines.first().expect("input file for day 2 must not be empty");
    println!("Puzzle 1: {}", day2::puzzle1(input));
    println!("Puzzle 2: {}", day2::puzzle2(input));
}