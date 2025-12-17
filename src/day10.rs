use std::{collections::HashMap, fmt::Display};

use anyhow::Result;
use itertools::Itertools;
use tracing::{debug, info};

use crate::{DDay, lines_from_file};

#[derive(Debug, Default)]
pub struct Day10;

#[derive(Debug, Default)]
struct Machine {
    target_len: i32,
    target: u16,
    switches: Vec<u16>,
    joltages: Vec<i32>,
    cache: HashMap<u16, Vec<Vec<u16>>>,
}

impl From<&String> for Machine {
    fn from(value: &String) -> Self {
        let mut target = 0;
        let mut target_len = 0;
        let mut switches = vec![];
        let mut joltages = vec![];
        for s in value.split_whitespace() {
            match s.chars().next().unwrap() {
                '[' => {
                    target_len = s.len() - 2;
                    let s = &s[1..s.len() - 1];
                    s.char_indices().for_each(|c| {
                        if c.1 == '#' {
                            target |= 1 << c.0;
                        }
                    });
                }
                '(' => {
                    let s = &s[1..s.len() - 1];
                    let sw = s.split(',').fold(0_u16, |mut acc, n| {
                        let bit = n.parse::<u16>().unwrap();
                        acc |= 1 << bit as usize;
                        acc
                    });
                    switches.push(sw);
                }
                '{' => {
                    let s = &s[1..s.len() - 1];
                    joltages = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
                }
                _ => panic!(),
            }
        }

        Machine {
            target_len: target_len as i32,
            target,
            switches,
            joltages,
            cache: HashMap::new(),
        }
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "len={} target={:#b}\t", self.target_len, self.target)?;
        for s in &self.switches {
            write!(f, " {:#b}\t", s)?;
        }
        write!(f, "joltages={:?}", self.joltages)
    }
}

fn get_presses(machine: &Machine, target: u16) -> Vec<Vec<u16>> {
    debug!(?target, "get_presses");
    let mut presses = vec![];
    for i in 0..machine.switches.len() + 1 {
        for switches_combination in machine.switches.iter().combinations(i) {
            if switches_combination.iter().fold(0, |acc, &x| acc ^ x) == target {
                presses.push(switches_combination.iter().map(|&x| *x).collect());
            }
        }
    }
    presses
}

fn configure_indicators(machine: &Machine) -> i32 {
    let presses = get_presses(machine, machine.target);
    presses.iter().min_by_key(|x| x.len()).unwrap().len() as i32
}

fn process(input: &mut [String]) -> i32 {
    input
        .iter()
        .fold(0, |acc, line| acc + configure_indicators(&line.into()))
}

fn configure_joltages(machine: &mut Machine, joltages: Vec<i32>) -> Option<i32> {
    debug!(?joltages, "Target joltages");

    // target joltages are all 0, we are done
    if joltages.iter().all(|&x| x == 0) {
        return Some(0);
    }

    let indicators: Vec<u16> = joltages.iter().map(|x| (x % 2) as u16).collect();
    let mut target = 0;
    for (pos, n) in indicators.iter().enumerate() {
        target |= n << pos;
    }

    let patterns = match machine.cache.get(&target) {
        Some(p) => p.clone(),
        None => {
            let p = get_presses(machine, target);
            machine.cache.insert(target, p.clone());
            p
        }
    };
    debug!(?patterns, "Patterns");

    let mut result: Option<i32> = None;
    for presses in &patterns {
        debug!(?presses, "Applying presses:");
        // Apply presses to find new target joltages
        let mut new_joltages = joltages.clone();
        for p in presses {
            for idx in 0..new_joltages.len() {
                if p & (1 << idx) != 0 {
                    new_joltages[idx] -= 1;
                }
            }
        }

        if new_joltages.iter().any(|&x| x < 0) {
            continue;
        }

        // all joltages are even
        let new_joltages: Vec<i32> = new_joltages.iter().map(|&x| x / 2).collect();
        if let Some(half_target_presses) = configure_joltages(machine, new_joltages) {
            let num_presses = presses.len() as i32 + 2 * half_target_presses;
            result = match result {
                Some(res) => Some(num_presses.min(res)),
                None => Some(num_presses),
            };
        } else {
            continue;
        }
    }

    result
}

fn process2(input: &mut [String]) -> i32 {
    input.iter().fold(0, |acc, line| {
        let mut machine: Machine = line.into();
        debug!("Processing machine: {}", machine);
        let joltages = machine.joltages.clone();
        acc + configure_joltages(&mut machine, joltages).unwrap()
    })
}

impl DDay for Day10 {
    fn run(&self) -> Result<()> {
        let res = process(&mut lines_from_file("./input/day10.txt")?);
        info!("1st part: {}", res);
        let res = process2(&mut lines_from_file("./input/day10.txt")?);
        info!("2nd part: {}", res);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    #[test_log::test]
    fn test1() {
        let mut input = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        ];
        assert!(process(&mut input) == 7);
    }

    #[test_log::test]
    fn test2() {
        let mut input = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        ];
        assert!(process2(&mut input) == 33);
    }
}
