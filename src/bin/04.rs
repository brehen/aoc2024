use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

// TODO: Set the expected answer for the test input
const PART1_ANSWER: usize = 18;
const PART2_ANSWER: usize = 9;

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
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
    let input_str = read_to_string(reader)?;
    let grid: Vec<Vec<char>> = input_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();
    // TODO: Solve Part 1 of the puzzle
    Ok(count_word_occurrences(&grid, "XMAS"))
}

fn is_valid_direction(
    grid: &[Vec<char>],
    row_index: usize,
    column_index: usize,
    dx: isize,
    dy: isize,
) -> bool {
    let new_row = row_index as isize + 3 * dy;
    let new_col = column_index as isize + 3 * dx;
    new_row >= 0
        && new_row < grid.len() as isize
        && new_col >= 0
        && new_col < grid[0].len() as isize
}

fn check_word(
    grid: &[Vec<char>],
    row_index: usize,
    column_index: usize,
    dx: isize,
    dy: isize,
    word: &str,
) -> bool {
    for (i, ch) in word.chars().enumerate() {
        let new_row = row_index as isize + i as isize * dy;
        let new_col = column_index as isize + i as isize * dx;
        if grid[new_row as usize][new_col as usize] != ch {
            return false;
        }
    }
    true
}

fn count_word_occurrences(grid: &[Vec<char>], word: &str) -> usize {
    let mut occurences = 0;

    let directions = [
        (0, -1),  // N
        (1, -1),  // NE
        (1, 0),   // E
        (1, 1),   // SE
        (0, 1),   // S
        (-1, 1),  // SW
        (-1, 0),  // W
        (-1, -1), // NW
    ];

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, _char) in row.iter().enumerate() {
            for &(dx, dy) in &directions {
                if is_valid_direction(grid, row_index, column_index, dx, dy)
                    && check_word(grid, row_index, column_index, dx, dy, word)
                {
                    occurences += 1;
                }
            }
        }
    }
    occurences
}

fn count_xes(grid: &[Vec<char>]) -> usize {
    let mut occurences = 0;

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, char) in row.iter().enumerate() {
            if col_index > 0
                && row_index > 0
                && row.len() - col_index > 1
                && grid.len() - row_index > 1
                && char == &'A'
            {
                let nw = grid[row_index - 1][col_index - 1];
                let se = grid[row_index + 1][col_index + 1];
                let ne = grid[row_index - 1][col_index + 1];
                let sw = grid[row_index + 1][col_index - 1];

                let mut mas_es = 0;
                if (nw == 'M' && se == 'S') || (nw == 'S' && se == 'M') {
                    mas_es += 1;
                }
                if (ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M') {
                    mas_es += 1;
                }

                if (mas_es == 2) {
                    occurences += 1;
                }
            }
        }
    }

    occurences
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let input_str = read_to_string(reader)?;
    let grid: Vec<Vec<char>> = input_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    // TODO: Solve Part 1 of the puzzle
    Ok(count_xes(&grid))
}
