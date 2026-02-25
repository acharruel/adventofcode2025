use anyhow::Result;
use tracing::debug;

use crate::{AocRun, single_line_from_file};

fn is_invalid_part1(number: u64) -> bool {
    let len: u32 = ((number as f32).log10() + 1.0).floor() as u32 / 2;
    let a = number / (10_u64.pow(len));
    let b = number % (10_u64.pow(len));
    a == b
}

fn process_part1(input: String) -> u64 {
    let tokens: Vec<&str> = input.split(',').collect();
    let mut invalid = 0;

    for token in tokens {
        debug!(?token);
        let min = token.split('-').collect::<Vec<&str>>()[0]
            .parse::<u64>()
            .unwrap();
        let max = token.split('-').collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .unwrap();
        for i in min..(max + 1) {
            if is_invalid_part1(i) {
                invalid += i;
            }
        }
    }
    invalid
}

fn is_invalid_part2(number: u64) -> bool {
    let len: u32 = ((number as f32).log10() + 1.0).floor() as u32;
    for i in 1..((len / 2) + 1) {
        let mut failed = false;
        if !len.is_multiple_of(i) {
            continue;
        }

        let a = number % (10_u64.pow(i));
        let mut tmp = number / (10_u64.pow(i));
        while tmp > 0 {
            let b = tmp % (10_u64.pow(i));
            if a != b {
                failed = true;
                break;
            }
            tmp /= 10_u64.pow(i);
        }

        if !failed {
            return true;
        }
    }
    false
}

fn process_part2(input: String) -> u64 {
    let tokens: Vec<&str> = input.split(',').collect();
    let mut invalid = 0;

    for token in tokens {
        debug!(?token);
        let min = token.split('-').collect::<Vec<&str>>()[0]
            .parse::<u64>()
            .unwrap();
        let max = token.split('-').collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .unwrap();
        for i in min..(max + 1) {
            if is_invalid_part2(i) {
                invalid += i;
            }
        }
    }
    invalid
}

#[derive(Debug, Default)]
pub struct Day02;

impl AocRun for Day02 {
    fn run1(&self) -> Result<i64> {
        let res = process_part1(single_line_from_file("./input/day02.txt")?);
        Ok(res as i64)
    }
    fn run2(&self) -> Result<i64> {
        let res = process_part2(single_line_from_file("./input/day02.txt")?);
        Ok(res as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::day02::*;

    #[test_log::test]
    fn test1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        assert!(process_part1(input) == 1227775554);
    }

    #[test_log::test]
    fn test2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        assert!(process_part2(input) == 4174379265);
    }
}
