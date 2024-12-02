use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize, anyhow::Error> {
        let mut column1 = Vec::new();
        let mut column2 = Vec::new();

        for line in reader.lines() {
            let line = line?;

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let num1: i32 = parts[0].parse()?;
                let num2: i32 = parts[1].parse()?;
                column1.push(num1);
                column2.push(num2);
            }
        }
        column1.sort();
        column2.sort();

        let all_distances = column1
            .iter()
            .zip(column2.iter())
            .map(|(a, b)| (b - a).abs())
            .sum::<i32>();

        println!("{:?}", all_distances);

        Ok(all_distances as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut column1 = Vec::new();
        let mut column2 = Vec::new();

        for line in reader.lines() {
            let line = line?;

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let num1: i32 = parts[0].parse()?;
                let num2: i32 = parts[1].parse()?;
                column1.push(num1);
                column2.push(num2);
            }
        }

        let all_similarities = column1
            .iter()
            .map(|a| {
                let num_times_in_col2 = column2.iter().filter(|b| *b == a).count();
                a * num_times_in_col2 as i32
            })
            .sum::<i32>();

        println!("{:?}", all_similarities);

        Ok(all_similarities as usize)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
