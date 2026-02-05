use std::{collections::HashMap, vec};

use anyhow::Result;
use tracing::{debug, info};

use crate::{DDay, lines_from_file};

#[derive(Debug, Default)]
pub struct Day11;

#[derive(Debug)]
struct Graph {
    map: HashMap<String, Vec<String>>,
    cache: HashMap<String, i64>,
}

impl Graph {
    fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl From<&mut [String]> for Graph {
    fn from(value: &mut [String]) -> Self {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for line in value {
            let node = line.split(':').collect::<Vec<&str>>()[0];
            let neighbours = line.split(':').collect::<Vec<&str>>()[1]
                .trim()
                .split(' ')
                .collect::<Vec<&str>>()
                .iter()
                .map(|&x| x.into())
                .collect::<Vec<String>>();

            map.insert(node.into(), neighbours);
        }

        let cache = HashMap::new();

        Graph { map, cache }
    }
}

fn walk(graph: &mut Graph, node: &str, mut visited: Vec<String>, target: &str) -> i64 {
    let mut total = 0;
    visited.push(node.into());
    debug!(?node, ?visited, ?target, "Entering walk2");

    if node == target {
        return 1;
    }

    if node == "out" {
        return 0;
    }

    if let Some((_, &v)) = graph.cache.get_key_value(node) {
        return v;
    }

    for neighbour in graph.map[node].clone() {
        if visited.contains(&neighbour) {
            continue;
        }
        total += walk(graph, &neighbour, visited.clone(), target);
    }

    graph.cache.insert(node.into(), total);

    total
}

fn process(input: &mut [String]) -> i64 {
    let mut graph: Graph = input.into();
    let visited = vec![];
    walk(&mut graph, "you", visited, "out")
}

fn process2(input: &mut [String]) -> i64 {
    let mut graph: Graph = input.into();
    let a = walk(&mut graph, "svr", vec![], "fft");
    graph.clear_cache();
    let b = walk(&mut graph, "fft", vec![], "dac");
    graph.clear_cache();
    let c = walk(&mut graph, "dac", vec![], "out");
    a * b * c
}

impl DDay for Day11 {
    fn run(&self) -> Result<()> {
        let res = process(&mut lines_from_file("./input/day11.txt")?);
        info!("1st part: {}", res);
        let res = process2(&mut lines_from_file("./input/day11.txt")?);
        info!("2nd part: {}", res);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::*;

    #[test_log::test]
    fn test1() {
        let mut input = vec![
            "aaa: you hhh".to_string(),
            "you: bbb ccc".to_string(),
            "bbb: ddd eee".to_string(),
            "ccc: ddd eee fff".to_string(),
            "ddd: ggg".to_string(),
            "eee: out".to_string(),
            "fff: out".to_string(),
            "ggg: out".to_string(),
            "hhh: ccc fff iii".to_string(),
            "iii: out".to_string(),
        ];
        assert!(process(&mut input) == 5);
    }

    #[test_log::test]
    fn test2() {
        let mut input = vec![
            "svr: aaa bbb".to_string(),
            "aaa: fft".to_string(),
            "fft: ccc".to_string(),
            "bbb: tty".to_string(),
            "tty: ccc".to_string(),
            "ccc: ddd eee".to_string(),
            "ddd: hub".to_string(),
            "hub: fff".to_string(),
            "eee: dac".to_string(),
            "dac: fff".to_string(),
            "fff: ggg hhh".to_string(),
            "ggg: out".to_string(),
            "hhh: out".to_string(),
        ];
        assert!(process2(&mut input) == 2);
    }
}
