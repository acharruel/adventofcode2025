use anyhow::Result;
use tracing::{debug, info};

use crate::lines_from_file;

fn process(input: Vec<String>) -> i32 {
    let mut pos = 50;

    input.iter().fold(0, |acc, line| {
        let mut n = 0;
        pos = match line.chars().nth(0).unwrap() {
            'L' => pos - line[1..].parse::<i32>().unwrap(),
            'R' => pos + line[1..].parse::<i32>().unwrap(),
            _ => panic!("wrong input"),
        };
        pos = pos % 100;
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
        pos = match line.chars().nth(0).unwrap() {
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

pub fn run() -> Result<()> {
    let res = process(lines_from_file("./input/day01.txt")?);
    info!("1st part: {}", res);
    let res = process2(lines_from_file("./input/day01.txt")?);
    info!("2nd part: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day01::*;

    #[test]
    fn test1() {
        let input = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        assert!(process(input) == 3);
    }

    #[test]
    fn test2() {
        let input = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        assert!(process2(input) == 6);
    }
}
