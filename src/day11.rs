use std::{collections::HashMap, vec};

use anyhow::Result;
use tracing::debug;

use crate::{AocRun, load_input_file};

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

impl AocRun for Day11 {
    fn run1(&self) -> Result<i64> {
        let res = process(&mut load_input_file("./input/day11.txt")?);
        Ok(res as i64)
    }
    fn run2(&self) -> Result<i64> {
        let res = process2(&mut load_input_file("./input/day11.txt")?);
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{day11::*, tests::load_test_input};

    #[test_log::test]
    fn test1() {
        let input = r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;
        assert!(process(&mut load_test_input(input)) == 5);
    }

    #[test_log::test]
    fn test2() {
        let input = r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;
        assert!(process2(&mut load_test_input(input)) == 2);
    }
}
