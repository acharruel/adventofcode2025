use std::collections::HashMap;

use anyhow::Result;
use tracing::{debug, info};

use crate::{DDay, lines_from_file};

#[derive(Debug, Default)]
pub struct Day08;

#[derive(Debug)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
    circuit: i32,
}

impl JunctionBox {
    fn dist_sq(&self, other: &Self) -> u64 {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }

    fn set_circuit_nb(&mut self, circuit: i32) {
        self.circuit = circuit;
    }
}

fn process(input: &mut [String], mut connections: i32) -> i32 {
    let mut list = vec![];
    let mut sorted_dist: Vec<(u64, i32, i32)> = vec![];
    let mut circuit_nb = 0;
    let mut circuits: HashMap<i32, Vec<i32>> = HashMap::new();

    input.iter().for_each(|line| {
        let mut it = line.split(',');
        let x: u64 = it.next().unwrap().parse().unwrap();
        let y: u64 = it.next().unwrap().parse().unwrap();
        let z: u64 = it.next().unwrap().parse().unwrap();
        list.push(JunctionBox {
            x,
            y,
            z,
            circuit: circuit_nb,
        });
        circuits.insert(circuit_nb, vec![circuit_nb]);
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

    let mut it = sorted_dist.iter();
    while connections > 0 {
        connections -= 1;
        let (_, jb1, jb2) = it.next().unwrap();
        debug!("Connecting a={}<->b={}", jb1, jb2);

        let c1 = list.get_mut(*jb1 as usize).unwrap().circuit;
        let c2 = list.get_mut(*jb2 as usize).unwrap().circuit;

        if c1 == c2 {
            continue;
        }

        debug!("Merging");
        let src = circuits.get(&c2).unwrap();
        let src = src.clone();
        let dest = circuits.get_mut(&c1).unwrap();
        for value in src {
            dest.push(value);
            let junction_box = list.get_mut(value as usize).unwrap();
            junction_box.set_circuit_nb(c1);
        }
        circuits.remove(&c2);
    }

    debug!(?circuits);

    let mut circuits_sizes: Vec<i32> = vec![];
    for c in circuits.into_values() {
        circuits_sizes.push(c.len() as i32);
    }
    circuits_sizes.sort();
    circuits_sizes.reverse();
    circuits_sizes.iter().take(3).product()
}

impl DDay for Day08 {
    fn run(&self) -> Result<()> {
        let res = process(&mut lines_from_file("./input/day08.txt")?, 1000);
        info!("1st part: {}", res);
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
}
