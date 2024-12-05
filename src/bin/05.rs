use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_to_string<R: BufRead>(mut reader: R) -> Result<String> {
    let mut input_str = String::new();
    reader.read_to_string(&mut input_str)?;
    Ok(input_str)
}

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const PART1_ANSWER: usize = 143;
const PART2_ANSWER: usize = 123;

const TEST: &str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

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

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let input_str = read_to_string(reader)?;
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let split: Vec<&str> = input_str.split("\n\n").collect();

    for rule in split[0].trim().lines() {
        let parts: Vec<i32> = rule.split('|').map(|x| x.parse::<i32>().unwrap()).collect();
        rules.push((parts[0], parts[1]));
    }

    let mut valid_update_value = 0;
    for update in split[1].trim().lines() {
        let pages: Vec<i32> = update
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if is_valid_update(&pages, &rules) {
            valid_update_value += pages[pages.len() / 2];
        }
    }

    Ok(valid_update_value as usize)
}

fn is_valid_update(update: &[i32], rules: &[(i32, i32)]) -> bool {
    for &(page_a, page_b) in rules {
        let pos_a = update.iter().position(|&x| x == page_a);
        let pos_b = update.iter().position(|&x| x == page_b);

        if let (Some(a_idx), Some(b_idx)) = (pos_a, pos_b) {
            if a_idx > b_idx {
                return false;
            }
        }
    }
    true
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let input_str = read_to_string(reader)?;
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let split: Vec<&str> = input_str.split("\n\n").collect();

    for rule in split[0].trim().lines() {
        let parts: Vec<i32> = rule.split('|').map(|x| x.parse::<i32>().unwrap()).collect();
        rules.push((parts[0], parts[1]));
    }

    let mut valid_update_value = 0;
    for update in split[1].trim().lines() {
        let mut pages: Vec<i32> = update
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if !is_valid_update(&pages, &rules) {
            let sorted_pages = sort_update(&mut pages, &rules);
            println!("Sorted update: {:?}", sorted_pages);
            valid_update_value += sorted_pages[sorted_pages.len() / 2];
        }
    }

    Ok(valid_update_value as usize)
}

fn sort_update(update: &mut [i32], rules: &[(i32, i32)]) -> Vec<i32> {
    update.sort_by(|&a, &b| {
        if is_legal_order(a, b, rules) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    update.to_owned()
}

fn is_legal_order(a: i32, b: i32, rules: &[(i32, i32)]) -> bool {
    !rules.iter().any(|&(x, y)| x == b && y == a)
}
