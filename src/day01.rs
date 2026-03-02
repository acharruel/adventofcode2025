use anyhow::Result;
use tracing::debug;

use crate::{AocRun, load_input_file};

fn process(input: Vec<String>) -> i32 {
    let mut pos = 50;

    input.iter().fold(0, |acc, line| {
        let mut n = 0;
        pos = match line.chars().next().unwrap() {
            'L' => pos - line[1..].parse::<i32>().unwrap(),
            'R' => pos + line[1..].parse::<i32>().unwrap(),
            _ => panic!("wrong input"),
        };
        pos %= 100;
        if pos < 0 {
            pos += 100;
        }
        if pos == 0 {
            n += 1;
        }
        acc + n
    })
}

fn process2(input: Vec<String>) -> i32 {
    let mut pos = 50;
    input.iter().fold(0, |acc, line| {
        debug!(?line, ?acc, ">>> ");
        let mut n = 0;
        let prev = pos;
        pos = match line.chars().next().unwrap() {
            'L' => {
                let p = line[1..].parse::<i32>().unwrap();
                n += (p / 100).abs();
                pos - (p % 100)
            }
            'R' => {
                let p = line[1..].parse::<i32>().unwrap();
                n += (p / 100).abs();
                pos + (p % 100)
            }
            _ => panic!("wrong input"),
        };
        if pos < 0 {
            if prev != 0 {
                n += 1;
            }
            pos += 100;
        }
        if pos == 0 {
            n += 1;
        }
        if pos >= 100 {
            n += 1;
            pos -= 100;
        }
        debug!(?prev, ?pos, ?n, new_acc = ?(acc + n), " ");
        acc + n
    })
}

#[derive(Debug, Default)]
pub struct Day01;

impl AocRun for Day01 {
    fn run1(&self) -> Result<i64> {
        let res = process(load_input_file("./input/day01.txt")?);
        Ok(res as i64)
    }

    fn run2(&self) -> Result<i64> {
        let res = process2(load_input_file("./input/day01.txt")?);
        Ok(res as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::{day01::*, tests::load_test_input};

    static TEST_INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test_log::test]
    fn test1() {
        assert!(process(load_test_input(TEST_INPUT)) == 3);
    }

    #[test_log::test]
    fn test2() {
        assert!(process2(load_test_input(TEST_INPUT)) == 6);
    }
}
