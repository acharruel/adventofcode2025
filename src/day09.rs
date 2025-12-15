use std::cmp::{max, min};

use anyhow::Result;
use itertools::Itertools;
use tracing::{debug, info};

use crate::{DDay, lines_from_file};

#[derive(Debug, Default)]
pub struct Day09;

#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn area(a: &Point, b: &Point) -> i64 {
    (max(a.x, b.x) - min(a.x, b.x) + 1) * (max(a.y, b.y) - min(a.y, b.y) + 1)
}

fn process(input: &mut [String]) -> i64 {
    let mut areas: Vec<i64> = input
        .iter()
        .map(|l| Point {
            x: l.split(',').nth(0).unwrap().parse::<i64>().unwrap(),
            y: l.split(',').nth(1).unwrap().parse::<i64>().unwrap(),
        })
        .tuple_combinations()
        .map(|elem: (Point, Point)| area(&elem.0, &elem.1))
        .collect();

    areas.sort();
    debug!(?areas);
    *areas.iter().rev().next().unwrap()
}

fn process2(input: &mut Vec<String>) -> i64 {
    let mut rectangles: Vec<(Point, Point)> = input
        .iter()
        .map(|l| Point {
            x: l.split(',').nth(0).unwrap().parse::<i64>().unwrap(),
            y: l.split(',').nth(1).unwrap().parse::<i64>().unwrap(),
        })
        .tuple_combinations()
        .collect();

    rectangles.sort_by_key(|rect| area(&rect.0, &rect.1));
    rectangles.reverse();

    input.push(input[0].clone());
    for rect in rectangles {
        debug!(?rect);
        let mut bad = false;
        for (a, b) in input.iter().tuple_windows() {
            let a = Point {
                x: a.split(',').nth(0).unwrap().parse::<i64>().unwrap(),
                y: a.split(',').nth(1).unwrap().parse::<i64>().unwrap(),
            };
            let b = Point {
                x: b.split(',').nth(0).unwrap().parse::<i64>().unwrap(),
                y: b.split(',').nth(1).unwrap().parse::<i64>().unwrap(),
            };
            // debug!(?a, ?b);

            // intersection test
            if min(a.x, b.x) < max(rect.0.x, rect.1.x)
                && min(a.y, b.y) < max(rect.0.y, rect.1.y)
                && max(a.x, b.x) > min(rect.0.x, rect.1.x)
                && max(a.y, b.y) > min(rect.0.y, rect.1.y)
            {
                debug!("intersection...");
                bad = true;
                break;
            }
        }
        if bad {
            continue;
        } else {
            info!(?rect, "Found area: {}", area(&rect.0, &rect.1));
            return area(&rect.0, &rect.1);
        }
    }

    0
}

impl DDay for Day09 {
    fn run(&self) -> Result<()> {
        let res = process(&mut lines_from_file("./input/day09.txt")?);
        info!("1st part: {}", res);
        let res = process2(&mut lines_from_file("./input/day09.txt")?);
        info!("2nd part: {}", res);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::day09::*;

    #[test_log::test]
    fn test1() {
        let mut input = vec![
            "7,1".to_string(),
            "11,1".to_string(),
            "11,7".to_string(),
            "9,7".to_string(),
            "9,5".to_string(),
            "2,5".to_string(),
            "2,3".to_string(),
            "7,3".to_string(),
        ];
        assert!(process(&mut input) == 50);
    }

    #[test_log::test]
    fn test2() {
        let mut input = vec![
            "7,1".to_string(),
            "11,1".to_string(),
            "11,7".to_string(),
            "9,7".to_string(),
            "9,5".to_string(),
            "2,5".to_string(),
            "2,3".to_string(),
            "7,3".to_string(),
        ];
        assert!(process2(&mut input) == 24);
    }
}

//   0 1 2 3 4 5 6 7 8 9 0 1 2 3
// 0 . . . . . . . . . . . . . .
// 1 . . . . . . . # . . . # . .
// 2 . . . . . . . . . . . . . .
// 3 . . # . . . . # . . . . . .
// 4 . . . . . . . . . . . . . .
// 5 . . # . . . . . . # . . . .
// 6 . . . . . . . . . . . . . .
// 7 . . . . . . . . . # . # . .
// 8 . . . . . . . . . . . . . .
//
//   0 1 2 3 4 5 6 7 8 9 0 1 2 3
// 0 . . . . . . . . . . . . . .
// 1 . . . . . . . # X X X # . .
// 2 . . . . . . . X . . . X . .
// 3 . . # X X X X # . . . X . .
// 4 . . X . . . . . . . . X . .
// 5 . . # X X X X X X # . X . .
// 6 . . . . . . . . . X . X . .
// 7 . . . . . . . . . # X # . .
// 8 . . . . . . . . . . . . . .
