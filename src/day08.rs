use anyhow::Result;
use tracing::{debug, info};

use crate::utils::dsu::Dsu;
use crate::{DDay, lines_from_file};

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

impl DDay for Day08 {
    fn run(&self) -> Result<()> {
        let res = process(&mut lines_from_file("./input/day08.txt")?, 1000);
        info!("1st part: {}", res);
        let res = process2(&mut lines_from_file("./input/day08.txt")?);
        info!("2nd part: {}", res);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::day08::*;

    #[test_log::test]
    fn test1() {
        let mut input = vec![
            "162,817,812".to_string(),
            "57,618,57".to_string(),
            "906,360,560".to_string(),
            "592,479,940".to_string(),
            "352,342,300".to_string(),
            "466,668,158".to_string(),
            "542,29,236".to_string(),
            "431,825,988".to_string(),
            "739,650,466".to_string(),
            "52,470,668".to_string(),
            "216,146,977".to_string(),
            "819,987,18".to_string(),
            "117,168,530".to_string(),
            "805,96,715".to_string(),
            "346,949,466".to_string(),
            "970,615,88".to_string(),
            "941,993,340".to_string(),
            "862,61,35".to_string(),
            "984,92,344".to_string(),
            "425,690,689".to_string(),
        ];
        assert!(process(&mut input, 10) == 40);
    }

    #[test_log::test]
    fn test2() {
        let mut input = vec![
            "162,817,812".to_string(),
            "57,618,57".to_string(),
            "906,360,560".to_string(),
            "592,479,940".to_string(),
            "352,342,300".to_string(),
            "466,668,158".to_string(),
            "542,29,236".to_string(),
            "431,825,988".to_string(),
            "739,650,466".to_string(),
            "52,470,668".to_string(),
            "216,146,977".to_string(),
            "819,987,18".to_string(),
            "117,168,530".to_string(),
            "805,96,715".to_string(),
            "346,949,466".to_string(),
            "970,615,88".to_string(),
            "941,993,340".to_string(),
            "862,61,35".to_string(),
            "984,92,344".to_string(),
            "425,690,689".to_string(),
        ];
        assert!(process2(&mut input) == 25272);
    }
}
