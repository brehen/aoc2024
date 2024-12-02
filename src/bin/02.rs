use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

const SAFE_LIMIT: i32 = 3;

fn main() -> Result<()> {
    start_day(DAY);

    // Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_reports = 0;
        for report in reader.lines() {
            let levels: Vec<i32> = report?
                .split_whitespace()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?;

            if is_safe(&levels) {
                safe_reports += 1;
            }
        }
        Ok(safe_reports)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    // Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_reports = 0;
        for report in reader.lines() {
            let levels: Vec<i32> = report?
                .split_whitespace()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?;

            if is_safe(&levels) {
                safe_reports += 1;
            } else {
                for i in 0..levels.len() {
                    let mut modified_levels = levels.clone();
                    modified_levels.remove(i);
                    if is_safe(&modified_levels) {
                        safe_reports += 1;
                        break;
                    }
                }
            }
        }
        Ok(safe_reports)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut is_asc = None;

    for window in levels.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 || diff.abs() > SAFE_LIMIT {
            return false;
        }
        match is_asc {
            None => is_asc = Some(diff > 0),
            Some(true) if diff < 0 => return false,
            Some(false) if diff > 0 => return false,
            _ => {}
        }
    }

    true
}
