use std::collections::BTreeSet;

use anyhow::Result;
use tracing::debug;

use crate::{AocRun, lines_from_file};

#[derive(Debug, Default)]
pub struct Day05;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Interval {
    lower: u64,
    upper: u64,
}

fn fill_fresh_intervals(set: &mut BTreeSet<Interval>, lower: u64, upper: u64) {
    let interval = Interval { lower, upper };
    let mut entry_containg_lower = None;
    let mut entry_containg_upper = None;

    for entry in set.iter() {
        // check if lower value is in set
        if lower > entry.lower && lower < entry.upper {
            entry_containg_lower = Some(entry);
        }
        // check if upper value is in set
        if upper > entry.lower && upper < entry.upper {
            entry_containg_upper = Some(entry);
        }
    }

    if let Some(entry) = entry_containg_lower {
        set.insert(Interval {
            lower: entry.lower,
            upper,
        });
    } else if let Some(entry) = entry_containg_upper {
        set.insert(Interval {
            lower,
            upper: entry.upper,
        });
    } else {
        set.insert(interval);
    }

    // TODO intervals are not merged!!
}

fn check_fresh(set: &BTreeSet<Interval>, n: u64) -> i32 {
    let mut total = 0;
    for entry in set.iter() {
        if n >= entry.lower && n <= entry.upper {
            total += 1;
            // break immediately because we didn't merge intervals
            break;
        }
    }
    total
}

fn check_fresh2(set: &BTreeSet<Interval>) -> u64 {
    let mut prev_upper = 0;
    set.iter().fold(0, |total, entry| {
        // intervals are not merged, skip if current entry was contained in
        // previous one
        if prev_upper > entry.upper {
            return total;
        }
        let add = entry.upper - entry.lower.max(prev_upper) + 1;
        // store upper bound of current interval
        prev_upper = entry.upper + 1;
        total + add
    })
}

enum State {
    Fill,
    Check,
}

fn process(input: Vec<String>) -> i32 {
    let mut state = State::Fill;
    let mut set: BTreeSet<Interval> = BTreeSet::new();
    let mut total = 0;
    for l in &input {
        if l.is_empty() {
            state = State::Check;
            continue;
        }

        match state {
            State::Fill => {
                let mut it = l.split('-');
                fill_fresh_intervals(
                    &mut set,
                    it.next().unwrap().parse().unwrap(),
                    it.next().unwrap().parse().unwrap(),
                );
            }
            State::Check => {
                total += check_fresh(&set, l.parse().unwrap());
            }
        }
    }

    debug!("total={}", total);
    total
}

fn process2(input: Vec<String>) -> u64 {
    let mut set: BTreeSet<Interval> = BTreeSet::new();
    for l in &input {
        if l.is_empty() {
            break;
        }
        let mut it = l.split('-');
        fill_fresh_intervals(
            &mut set,
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        );
    }

    let total = check_fresh2(&set);
    debug!("process2 total={}", total);
    total
}

impl AocRun for Day05 {
    fn run1(&self) -> Result<i64> {
        let res = process(lines_from_file("./input/day05.txt")?);
        Ok(res as i64)
    }
    fn run2(&self) -> Result<i64> {
        let res = process2(lines_from_file("./input/day05.txt")?);
        Ok(res as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::day05::*;

    #[test_log::test]
    fn test1() {
        let input = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            "".to_string(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string(),
        ];
        assert!(process(input) == 3);
    }

    #[test_log::test]
    fn test2() {
        let input = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            "".to_string(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string(),
        ];
        assert!(process2(input) == 14);
    }
}
