use std::vec;

use anyhow::Result;
use itertools::Itertools;
use tracing::info;

use crate::{DDay, lines_from_file};

#[derive(Debug, Default)]
pub struct Day12;

#[derive(Clone, Debug, Default)]
struct Present {
    shape: Vec<Vec<bool>>,
}

#[derive(Debug, Default)]
struct Region {
    length: usize,
    width: usize,
    nshapes: Vec<i32>,
}

impl Present {
    fn new() -> Self {
        Present {
            shape: vec![vec![], vec![], vec![]],
        }
    }

    #[allow(unused)]
    fn area(&self) -> i32 {
        let mut area = 0;
        for line in &self.shape {
            area += line.iter().fold(0, |acc, &x| if x { acc + 1 } else { acc });
        }
        area
    }
}

impl Region {
    fn area(&self) -> i32 {
        (self.length * self.width) as i32
    }
}

impl From<&mut String> for Region {
    fn from(line: &mut String) -> Self {
        let (length, width) = line
            .split(':')
            .next()
            .unwrap()
            .split('x')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        let nshapes = line
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        Region {
            length,
            width,
            nshapes,
        }
    }
}

fn process(input: &mut [String]) -> i32 {
    let mut presents_list: Vec<Present> = vec![];
    let mut present: Option<Present> = None;
    let mut shape_idx = 0;
    let mut regions_list: Vec<Region> = vec![];

    for line in input {
        match line {
            _ if line.contains(':') && !line.contains('x') => {
                // new present
                shape_idx = 0;
                present = Some(Present::new());
            }
            _ if line.starts_with('.') || line.starts_with('#') => {
                // fill present shape
                line.chars().for_each(|c| match c {
                    '.' => present.as_mut().unwrap().shape[shape_idx].push(false),
                    '#' => present.as_mut().unwrap().shape[shape_idx].push(true),
                    _ => panic!(),
                });
                shape_idx += 1;
            }
            _ if line.is_empty() => {
                // finalize present
                presents_list.push(present.clone().unwrap());
            }
            _ if line.contains(':') && line.contains('x') => {
                // region
                regions_list.push(line.into());
            }
            _ => (),
        }
    }

    let mut total = 0;
    for region in regions_list {
        let sum = region.nshapes.iter().sum::<i32>() * 8;
        if sum < region.area() {
            total += 1;
            continue;
        }
    }

    total
}

impl DDay for Day12 {
    fn run(&self) -> Result<()> {
        let res = process(&mut lines_from_file("./input/day12.txt")?);
        info!("1st part: {}", res);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::*;

    #[test_log::test]
    fn test1() {
        let mut input = vec![
            "0:".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "##.".to_string(),
            "".to_string(),
            "1:".to_string(),
            "###".to_string(),
            "##.".to_string(),
            ".##".to_string(),
            "".to_string(),
            "2:".to_string(),
            ".##".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "".to_string(),
            "3:".to_string(),
            "##.".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "".to_string(),
            "4:".to_string(),
            "###".to_string(),
            "#..".to_string(),
            "###".to_string(),
            "".to_string(),
            "5:".to_string(),
            "###".to_string(),
            ".#.".to_string(),
            "###".to_string(),
            "".to_string(),
            "4x4: 0 0 0 0 2 0".to_string(),
            "12x5: 1 0 1 0 2 2".to_string(),
            "12x5: 1 0 1 0 3 2".to_string(),
        ];
        assert!(process(&mut input) == 2);
    }
}
