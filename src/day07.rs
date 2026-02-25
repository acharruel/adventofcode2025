use std::collections::{BTreeSet, HashMap};

use anyhow::Result;
use tracing::debug;

use crate::{AocRun, lines_from_file};

#[derive(Debug, Default)]
pub struct Day07;

fn process(input: &mut [String]) -> i32 {
    let mut positions: BTreeSet<usize> = BTreeSet::new();
    let mut it = input.iter();
    let mut total = 0;

    if let Some(line) = it.next() {
        positions.insert(line.chars().position(|c| c == 'S').unwrap());
    };

    debug!(?positions);

    it.for_each(|line| {
        let mut splitters: Vec<usize> = vec![];
        for elem in positions.iter() {
            if line.chars().nth(*elem).unwrap() == '^' {
                total += 1;
                splitters.push(*elem);
            }
        }
        for s in splitters {
            positions.insert(s - 1);
            positions.insert(s + 1);
            positions.remove(&s);
        }
        debug!(?positions);
    });

    total
}

fn recurse(
    input: &Vec<String>,
    pos: usize,
    depth: usize,
    hash: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let mut n = 0;

    debug!(?pos, ?depth);

    let line = match input.get(depth) {
        Some(l) => l,
        None => {
            debug!(?depth, ?pos, "END OF TIMELINE");
            return 1;
        }
    };

    if hash.contains_key(&(pos, depth)) {
        let n = hash[&(pos, depth)];
        debug!(?depth, ?pos, ?n, "using hash");
        return n;
    }

    if line.chars().nth(pos).unwrap() == '^' {
        n += recurse(input, pos - 1, depth + 2, hash);
        n += recurse(input, pos + 1, depth + 2, hash);

        debug!(?depth, ?pos, ?n, "node complete, insert hash");
        hash.insert((pos, depth), n);
    } else {
        n += recurse(input, pos, depth + 2, hash);
    }

    n
}

fn process2(input: &mut Vec<String>) -> u64 {
    let mut it = input.iter();
    let mut hash: HashMap<(usize, usize), u64> = HashMap::new();

    let pos = if let Some(line) = it.next() {
        line.chars().position(|c| c == 'S').unwrap()
    } else {
        panic!();
    };

    recurse(input, pos, 2, &mut hash)
}

impl AocRun for Day07 {
    fn run1(&self) -> Result<i64> {
        let res = process(&mut lines_from_file("./input/day07.txt")?);
        Ok(res as i64)
    }
    fn run2(&self) -> Result<i64> {
        let res = process2(&mut lines_from_file("./input/day07.txt")?);
        Ok(res as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::day07::*;

    #[test_log::test]
    fn test1() {
        let mut input = vec![
            ".......S.......".to_string(), //0
            "...............".to_string(), //1
            ".......^.......".to_string(), //2
            "...............".to_string(), //3
            "......^.^......".to_string(), //4
            "...............".to_string(), //5
            ".....^.^.^.....".to_string(), //6
            "...............".to_string(), //7
            "....^.^...^....".to_string(), //8
            "...............".to_string(), //9
            "...^.^...^.^...".to_string(), //10
            "...............".to_string(), //11
            "..^...^.....^..".to_string(), //12
            "...............".to_string(), //13
            ".^.^.^.^.^...^.".to_string(), //14
            "...............".to_string(), //15
        ];
        assert!(process(&mut input) == 21);
    }

    #[test_log::test]
    fn test2() {
        let mut input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string(),
        ];
        assert!(process2(&mut input) == 40);
    }
}
