use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    fmt::Display,
};

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

fn backtrace(parents: HashMap<u16, u16>, end: u16) -> Vec<u16> {
    let mut path: Vec<u16> = vec![end];
    debug!(?parents, ?end);
    while path[path.len() - 1] != 0 {
        path.push(parents[&path[path.len() - 1]]);
    }
    path.reverse();
    path
}

fn part1_bfs(machine: &Machine) -> i32 {
    let mut queue: VecDeque<(u16, i32)> = VecDeque::new();
    let mut visited: Vec<u16> = vec![];
    // backtrack state + switch (ie. transition)
    let mut parents: HashMap<u16, u16> = HashMap::new();

    // initial state = 0
    queue.push_back((0, 0));
    visited.push(0);

    while !queue.is_empty() {
        // pop front to traverse the tree horizontally
        if let Some((state, dist)) = queue.pop_front() {
            if state == machine.target {
                debug!(?dist, path = ?backtrace(parents, state));
                return dist;
            }

            for sw in &machine.switches {
                let next_state = state ^ sw;
                if !visited.contains(&next_state) {
                    visited.push(next_state);
                    queue.push_back((next_state, dist + 1));
                    parents.insert(next_state, *sw);
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

    machines.iter().fold(0, |acc, n| acc + part1_bfs(n))
}

#[derive(Debug, Default)]
struct Machine2 {
    switches: Vec<Vec<i32>>,
    joltages: Vec<i32>,
}

fn test_linear_combination(
    switches: &[Vec<i32>],
    combination: &[i32],
    joltages: &[i32],
) -> Ordering {
    debug!(?combination, "Testing...");

    let mut ordering = Ordering::Equal;
    for index in 0..joltages.len() {
        let mut i = 0;
        let total = switches.iter().fold(0, |mut acc, x| {
            acc += x[index] * combination[i];
            i += 1;
            acc
        });
        if total > joltages[index] {
            return Ordering::Greater;
        } else if total < joltages[index] {
            ordering = Ordering::Less;
        }
    }

    ordering
}

fn part2_ilp(switches: &[Vec<i32>], joltages: &[i32]) -> i32 {
    let mut combination = vec![0; switches.len()];
    let mut combination_max: Vec<i32> = vec![0; switches.len()];

    // calculate max range for the linear combination
    for i in 0..switches.len() {
        let sw = &switches[i];
        for j in 0..sw.len() {
            if sw[j] == 1 {
                if combination_max[i] == 0 {
                    combination_max[i] = joltages[j];
                } else if combination_max[j] > joltages[j] {
                    combination_max[i] = joltages[j];
                }
            }
        }
    }

    debug!(?combination_max);

    for i in 0..combination_max.len() {
        let vec: Vec<i32> = (0..combination_max[i]).collect();
        debug!(?vec);
    }

    for jolt_idx in 0..joltages.len() {
        for sw_idx in 0..switches.len() {
            for value in 0..(combination_max[sw_idx] + 1) {
                debug!(?jolt_idx, ?sw_idx, ?value);
                combination[sw_idx] = value;
                debug!(?combination);
            }
        }
    }

    0
}

fn process2(input: &mut [String]) -> i32 {
    let machines: Vec<Machine2> = input
        .iter()
        .map(|l| {
            let mut target_len = 0;
            let mut switches = vec![];
            let mut joltages = vec![];

            for s in l.split_whitespace() {
                match s.chars().next().unwrap() {
                    '[' => {
                        target_len = s.len() - 2;
                    }
                    '(' => {
                        let mut sw = vec![0; target_len];
                        s.trim_matches('(')
                            .trim_matches(')')
                            .split(',')
                            .for_each(|n| {
                                let n: usize = n.parse().unwrap();
                                sw[n] = 1;
                            });
                        switches.push(sw);
                    }
                    '{' => s
                        .trim_matches('{')
                        .trim_matches('}')
                        .split(',')
                        .for_each(|n| {
                            joltages.push(n.parse().unwrap());
                        }),
                    _ => panic!(),
                }
            }

            Machine2 { switches, joltages }
        })
        .collect();

    debug!(?machines);
    let res = part2_ilp(&machines[0].switches, &machines[0].joltages);

    0
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

    // #[test_log::test]
    // fn test2() {
    //     let mut input = vec![
    //         "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
    //         "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
    //         "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
    //     ];
    //     assert!(process2(&mut input) == 7);
    // }
}
