use anyhow::Result;
use tracing::debug;

use crate::utils::dsu::Dsu;
use crate::{AocRun, load_input_file};

#[derive(Debug, Default)]
pub struct Day08;

#[derive(Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    circuit: i32,
}

impl JunctionBox {
    fn dist_sq(&self, other: &Self) -> i64 {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }
}

fn process(input: &mut [String], mut connections: i32) -> i32 {
    let mut list = vec![];
    let mut sorted_dist: Vec<(i64, i32, i32)> = vec![];
    let mut circuit_nb = 0;

    input.iter().for_each(|line| {
        let mut it = line.split(',');
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        let z: i64 = it.next().unwrap().parse().unwrap();
        list.push(JunctionBox {
            x,
            y,
            z,
            circuit: circuit_nb,
        });
        circuit_nb += 1;
    });

    for i in 0..(list.len() - 1) {
        for j in (i + 1)..list.len() {
            let a = &list[i];
            let b = &list[j];
            let dist = a.dist_sq(b);
            sorted_dist.push((dist, i as i32, j as i32));
        }
    }
    sorted_dist.sort_by_key(|(d, _, _)| *d);

    let mut dsu = Dsu::new(input.len());
    let mut it = sorted_dist.iter();
    while connections > 0 {
        connections -= 1;
        let (_, jb1, jb2) = it.next().unwrap();
        debug!("Connecting a={}<->b={}", jb1, jb2);
        let c1 = list.get_mut(*jb1 as usize).unwrap().circuit;
        let c2 = list.get_mut(*jb2 as usize).unwrap().circuit;
        dsu.union(c1, c2);
    }

    let mut sizes = dsu.get_sizes().clone();
    sizes.sort();
    sizes.reverse();
    sizes.iter().take(3).fold(1, |acc, x| acc * *x as i32)
}

fn process2(input: &mut [String]) -> i64 {
    let mut list = vec![];
    let mut sorted_dist: Vec<(i64, i32, i32)> = vec![];
    let mut circuit_nb = 0;

    input.iter().for_each(|line| {
        let mut it = line.split(',');
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        let z: i64 = it.next().unwrap().parse().unwrap();
        list.push(JunctionBox {
            x,
            y,
            z,
            circuit: circuit_nb,
        });
        circuit_nb += 1;
    });

    for i in 0..(list.len() - 1) {
        for j in (i + 1)..list.len() {
            let a = &list[i];
            let b = &list[j];
            let dist = a.dist_sq(b);
            sorted_dist.push((dist, i as i32, j as i32));
        }
    }
    sorted_dist.sort_by_key(|(d, _, _)| *d);

    let mut dsu = Dsu::new(input.len());
    let mut it = sorted_dist.iter();
    let mut res = 0;
    while dsu.component_sizes().len() > 1 {
        let (_, jb1, jb2) = it.next().unwrap();
        let c1 = list.get_mut(*jb1 as usize).unwrap().circuit;
        let c2 = list.get_mut(*jb2 as usize).unwrap().circuit;
        dsu.union(c1, c2);
        res = list.get(*jb1 as usize).unwrap().x;
        res *= list.get(*jb2 as usize).unwrap().x;
    }

    res
}

impl AocRun for Day08 {
    fn run1(&self) -> Result<i64> {
        let res = process(&mut load_input_file("./input/day08.txt")?, 1000);
        Ok(res as i64)
    }
    fn run2(&self) -> Result<i64> {
        let res = process2(&mut load_input_file("./input/day08.txt")?);
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{day08::*, tests::load_test_input};

    static TEST_INPUT: &str = r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test_log::test]
    fn test1() {
        assert!(process(&mut load_test_input(TEST_INPUT), 10) == 40);
    }

    #[test_log::test]
    fn test2() {
        assert!(process2(&mut load_test_input(TEST_INPUT)) == 25272);
    }
}
