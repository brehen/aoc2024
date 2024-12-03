use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "NN"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

// TODO: Set the expected answer for the test input
const PART1_ANSWER: usize = 0;
const PART2_ANSWER: usize = 0;

const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");
    assert_eq!(PART1_ANSWER, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    assert_eq!(PART2_ANSWER, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}

fn read_to_string<R: BufRead>(mut reader: R) -> Result<String> {
    let mut input_str = String::new();
    reader.read_to_string(&mut input_str)?;
    Ok(input_str)
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let input_str = read_to_string(reader);
    // TODO: Solve Part 1 of the puzzle
    Ok(input_str?.len())
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let input_str = read_to_string(reader);
    // TODO: Solve Part 1 of the puzzle
    Ok(input_str?.len())
}
