use anyhow::Result;
use itertools::Itertools;
use tracing::{debug, info};

use crate::{DDay, lines_from_file};

#[derive(Debug, Default)]
pub struct Day04;

#[derive(Debug)]
struct Position {
    line: usize,
    col: usize,
}

fn find_number_of_rolls(
    current: &str,
    prev: Option<&str>,
    next: Option<&str>,
    line: usize,
) -> Vec<Position> {
    let mut col = 0;
    let mut valid_positions: Vec<Position> = vec![];

    for c in current.chars() {
        let mut n = 0;
        if c != '@' {
            col += 1;
            continue;
        }
        if let Some(prev) = prev {
            if col > 0 && prev.chars().nth(col - 1).unwrap() == '@' {
                n += 1;
            }
            if prev.chars().nth(col).unwrap() == '@' {
                n += 1;
            }
            if prev.chars().nth(col + 1).unwrap_or('.') == '@' {
                n += 1;
            }
        }
        if let Some(next) = next {
            if col > 0 && next.chars().nth(col - 1).unwrap() == '@' {
                n += 1;
            }
            if next.chars().nth(col).unwrap() == '@' {
                n += 1;
            }
            if next.chars().nth(col + 1).unwrap_or('.') == '@' {
                n += 1;
            }
        }
        if col > 0 && current.chars().nth(col - 1).unwrap() == '@' {
            n += 1;
        }
        if current.chars().nth(col + 1).unwrap_or('.') == '@' {
            n += 1;
        }
        if n >= 4 {
            // debug!(?col, "invalid");
            col += 1;
            continue;
        }

        // debug!(?col, "valid");
        valid_positions.push(Position { line, col });
        col += 1;
    }

    // debug!(?valid_positions);
    // debug!("");
    valid_positions
}

fn iteration(input: &[String]) -> Vec<Position> {
    let mut valid_positions: Vec<Position> = vec![];

    // first line
    valid_positions.extend(find_number_of_rolls(&input[0], None, Some(&input[1]), 0));

    // last line
    valid_positions.extend(find_number_of_rolls(
        &input[input.len() - 1],
        Some(&input[input.len() - 2]),
        None,
        input.len() - 1,
    ));

    // iterate
    let mut i = 1;
    for (prev, cur, next) in input.iter().tuple_windows() {
        valid_positions.extend(find_number_of_rolls(cur, Some(prev), Some(next), i));
        i += 1;
    }

    valid_positions
}

impl DDay for Day04 {
    fn run(&self) -> Result<()> {
        let res = iteration(&lines_from_file("./input/day04.txt")?).len();
        info!("1st part: {}", res);
        let res = process(lines_from_file("./input/day04.txt")?);
        info!("2nd part: {}", res);
        Ok(())
    }
}

fn change_char_in_string(s: &str, index: usize, new_char: char) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    if index < chars.len() {
        chars[index] = new_char;
    }
    String::from_iter(chars)
}

fn process(input: Vec<String>) -> usize {
    let mut input = input;
    let mut total = 0;
    loop {
        let valid_positions = iteration(&input);
        if valid_positions.is_empty() {
            break;
        }
        total += valid_positions.len();
        debug!("{:#?}", valid_positions);
        for pos in valid_positions {
            let line = &mut input[pos.line];
            *line = change_char_in_string(line, pos.col, 'x');
        }
        debug!("{:#?}", input);
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::day04::*;

    #[test_log::test]
    fn test1() {
        let input = vec![
            "..@@.@@@@.".to_string(),
            "@@@.@.@.@@".to_string(),
            "@@@@@.@.@@".to_string(),
            "@.@@@@..@.".to_string(),
            "@@.@@@@.@@".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "@.@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "@.@.@@@.@.".to_string(),
        ];
        assert!(iteration(&input).len() == 13);
    }

    #[test_log::test]
    fn test2() {
        let input = vec![
            "..@@.@@@@.".to_string(),
            "@@@.@.@.@@".to_string(),
            "@@@@@.@.@@".to_string(),
            "@.@@@@..@.".to_string(),
            "@@.@@@@.@@".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "@.@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "@.@.@@@.@.".to_string(),
        ];
        assert!(process(input) == 43);
    }
}
