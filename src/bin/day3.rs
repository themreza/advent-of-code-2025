use std::fs;

use aoc2025::solutions::day3;

fn main() {
    let lines: &String  = &fs::read_to_string("inputs/day3.txt").expect("failed to read day 3 input");
    println!("Puzzle 1: {}", day3::puzzle1(&lines));
    println!("Puzzle 2: {}", day3::puzzle2(&lines));
}