use std::{collections::VecDeque, fmt::Display};

use anyhow::Result;
use tracing::{debug, info};

use crate::{DDay, lines_from_file};

#[derive(Debug, Default)]
pub struct Day10;

#[derive(Debug, Default)]
struct Machine {
    target_len: i32,
    target: u16,
    switches: Vec<u16>,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "len={} target={:#b}\t", self.target_len, self.target)?;
        for s in &self.switches {
            write!(f, " {:#b}\t", s)?;
        }
        Ok(())
    }
}

fn bfs(machine: &Machine) -> i32 {
    let mut queue: VecDeque<(u16, i32)> = VecDeque::new();
    let mut visited: Vec<u16> = vec![];

    // initial state = 0
    queue.push_back((0, 0));
    visited.push(0);

    while !queue.is_empty() {
        // pop front to traverse the tree horizontally
        if let Some((state, dist)) = queue.pop_front() {
            if state == machine.target {
                return dist;
            }

            for sw in &machine.switches {
                let next_state = state ^ sw;
                if !visited.contains(&next_state) {
                    visited.push(next_state);
                    queue.push_back((next_state, dist + 1));
                }
            }
        }
    }
    0
}

fn process(input: &mut [String]) -> i32 {
    let machines: Vec<Machine> = input
        .iter()
        .map(|l| {
            let mut target = 0;
            let mut target_len = 0;
            let mut switches = vec![];
            for s in l.split_whitespace() {
                match s.chars().next().unwrap() {
                    '[' => {
                        target_len = s.len() - 2;
                        let s = &s[1..s.len() - 1];
                        s.char_indices().for_each(|c| {
                            if c.1 == '#' {
                                target |= 1 << (target_len - 1 - c.0);
                            }
                        });
                    }
                    '(' => {
                        let s = &s[1..s.len() - 1];
                        let sw = s.split(',').fold(0_u16, |mut acc, n| {
                            let bit = n.parse::<u16>().unwrap();
                            acc |= 1 << (target_len - 1 - bit as usize);
                            acc
                        });
                        switches.push(sw);
                    }
                    '{' => (),
                    _ => panic!(),
                }
            }

            Machine {
                target_len: target_len as i32,
                target,
                switches,
            }
        })
        .collect();

    machines.iter().fold(0, |acc, n| acc + bfs(&n))
}

impl DDay for Day10 {
    fn run(&self) -> Result<()> {
        let res = process(&mut lines_from_file("./input/day10.txt")?);
        info!("1st part: {}", res);
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
}
