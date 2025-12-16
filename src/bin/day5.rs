use std::fs;

use aoc2025::solutions::day5;

fn main() {
    let lines: &String  = &fs::read_to_string("inputs/day5.txt").expect("failed to read day 5 input");
    println!("Puzzle 1:\n{}", day5::puzzle1(lines));
    println!("Puzzle 2:\n{}", day5::puzzle2(lines));
}