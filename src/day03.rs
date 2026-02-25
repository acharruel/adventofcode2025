use anyhow::Result;
use tracing::debug;

use crate::{AocRun, lines_from_file};

#[derive(Debug, Default)]
pub struct Day03;

fn process(input: Vec<String>, size: usize) -> u64 {
    let mut total: u64 = 0;

    for line in input {
        let mut pos = 0;
        let mut size = size;
        debug!(?line);

        while size > 0 {
            let mut i = 0;
            let mut max_pos = 0;
            let scan = &line[pos..(line.len() - size + 1)];
            let max: u64 = scan.chars().fold(0, |max, c| {
                i += 1;
                let c = u64::from(c.to_digit(10).unwrap());
                if c > max {
                    max_pos = i;
                    c
                } else {
                    max
                }
            });
            debug!(?max, ?pos);

            pos += max_pos;
            size -= 1;
            total += 10_u64.pow(size as u32) * max;
        }
    }
    total
}

impl AocRun for Day03 {
    fn run1(&self) -> Result<i64> {
        let res = process(lines_from_file("./input/day03.txt")?, 2);
        Ok(res as i64)
    }
    fn run2(&self) -> Result<i64> {
        let res = process(lines_from_file("./input/day03.txt")?, 12);
        Ok(res as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::day03::*;

    #[test_log::test]
    fn test1() {
        let input = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        assert!(process(input, 2) == 357);
    }

    #[test_log::test]
    fn test2() {
        let input = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        assert!(process(input, 12) == 3121910778619);
    }
}
