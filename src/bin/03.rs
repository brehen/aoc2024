use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut input_str = String::new();

        reader.read_to_string(&mut input_str).unwrap();

        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut sum = 0;

        for caps in re.captures_iter(&input_str) {
            let x = &caps[1].parse::<i32>().unwrap();
            let y = &caps[2].parse::<i32>().unwrap();
            sum += x * y;
        }
        Ok(sum as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input_str = String::new();

        reader.read_to_string(&mut input_str).unwrap();

        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

        let mut matches: Vec<Match> = Vec::new();

        for caps in re.captures_iter(&input_str) {
            if let Some(matched) = caps.get(0) {
                let x = &caps.get(1);
                let y = &caps.get(2);
                if let (Some(x), Some(y)) = (x, y) {
                    let x = x.as_str().parse::<i32>().unwrap();
                    let y = y.as_str().parse::<i32>().unwrap();
                    println!("{:?}, {:?}", x, y);
                    matches.push(Match::Mul(x, y));
                } else {
                    matches.push(Match::Do(matched.as_str() == "do()"));
                }
            }
        }

        let mut should_mult = true;
        let mut sum = 0;

        for m in &matches {
            match m {
                Match::Mul(x, y) => {
                    if should_mult {
                        sum += x * y;
                    }
                }
                Match::Do(should) => {
                    should_mult = *should;
                }
            }
        }

        println!("{:?}", sum);
        Ok(sum as usize)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug)]
enum Match {
    Mul(i32, i32),
    Do(bool),
}
