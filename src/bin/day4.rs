use std::fs;

use aoc2025::solutions::day4;

fn main() {
    let lines: &String  = &fs::read_to_string("inputs/day4.txt").expect("failed to read day 4 input");
    println!("Puzzle 1:\n{}", day4::puzzle1(lines));
    println!("Puzzle 2:\n{}", day4::puzzle2(lines));
}