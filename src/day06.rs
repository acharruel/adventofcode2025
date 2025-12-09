use anyhow::Result;
use tracing::{debug, info};

use crate::{DDay, lines_from_file};

#[derive(Debug, Default)]
pub struct Day06;

fn process(input: &mut [String]) -> u64 {
    let mut totals: Vec<u64> = vec![];
    let mut it = input.iter().rev();
    let operations: Vec<_> = it.next().unwrap().split_whitespace().collect();

    // init totals vector with neutral element
    for operator in operations.iter() {
        match *operator {
            "+" => totals.push(0),
            "*" => totals.push(1),
            &_ => panic!(),
        }
    }

    for line in it {
        let numbers: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        for (pos, (operand, operator)) in numbers.iter().zip(operations.iter()).enumerate() {
            totals[pos] = match *operator {
                "+" => totals[pos] + operand,
                "*" => totals[pos] * operand,
                &_ => panic!(),
            };
        }
    }

    totals.iter().sum::<u64>()
}

#[derive(Debug)]
struct Op<'a> {
    operation: char,
    size: usize,
    total: u64,
    column: Vec<&'a str>,
}

fn process2(input: &mut [String]) -> u64 {
    let mut operations: Vec<Op> = vec![];
    let mut size = 0;
    let mut total = 0;
    for c in input[input.len() - 1].chars() {
        let len = operations.len();
        match c {
            '+' => {
                if len > 0 {
                    operations[len - 1].size = size;
                }
                operations.push(Op {
                    operation: c,
                    size,
                    total: 0,
                    column: vec![],
                });
                size = 0;
            }
            '*' => {
                if len > 0 {
                    operations[len - 1].size = size;
                }
                operations.push(Op {
                    operation: c,
                    size,
                    total: 1,
                    column: vec![],
                });
                size = 0;
            }
            ' ' => {
                size += 1;
            }
            _ => panic!(),
        }
    }

    for line in input.iter().take(input.len() - 1) {
        let mut prev = 0;
        for op in operations.iter_mut() {
            op.column.push(&line[prev..(prev + op.size)]);
            debug!(?prev, ?op.size, ?op.column);
            prev += op.size + 1;
        }
    }

    for op in operations.iter_mut() {
        debug!("{:#?}", op);
        for i in 0..op.size {
            let mut power = 0;
            let n: u64 = op.column.iter().rev().fold(0, |mut acc, x| {
                if let Some(x) = x.chars().nth(i).unwrap().to_digit(10) {
                    acc += (x * 10_u32.pow(power)) as u64;
                    power += 1;
                }
                acc
            });
            op.total = match op.operation {
                '+' => op.total + n,
                '*' => op.total * n,
                _ => panic!(),
            };
            debug!(?n);
        }
        debug!(?op.total);
        total += op.total;
    }
    debug!(total);
    total
}

impl DDay for Day06 {
    fn run(&self) -> Result<()> {
        let res = process(&mut lines_from_file("./input/day06.txt")?);
        info!("1st part: {}", res);
        let res = process2(&mut lines_from_file("./input/day06.txt")?);
        info!("2nd part: {}", res);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::day06::*;

    #[test_log::test]
    fn test1() {
        let mut input = vec![
            "123 328  51 64".to_string(),
            " 45 64  387 23".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        assert!(process(&mut input) == 4277556);
    }

    #[test_log::test]
    fn test2() {
        let mut input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        assert!(process2(&mut input) == 3263827);
    }
}
