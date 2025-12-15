#[derive(Debug)]
pub struct Dsu {
    parents: Vec<i32>,
    sizes: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Dsu {
            parents: (0..n as i32).collect(),
            sizes: vec![1; n],
        }
    }

    pub fn find(&mut self, mut x: i32) -> i32 {
        let mut root = x;
        while self.parents[root as usize] != root {
            root = self.parents[root as usize];
        }

        while self.parents[x as usize] != root {
            let next = self.parents[x as usize];
            self.parents[x as usize] = root;
            x = next;
        }

        root
    }

    pub fn union(&mut self, a: i32, b: i32) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.sizes[ra as usize] < self.sizes[rb as usize] {
            self.parents[ra as usize] = rb;
            self.sizes[rb as usize] += self.sizes[ra as usize];
        } else {
            self.parents[rb as usize] = ra;
            self.sizes[ra as usize] += self.sizes[rb as usize];
        }

        true
    }

    pub fn component_sizes(&mut self) -> Vec<usize> {
        let n = self.parents.len();
        let mut counts = vec![0usize; n];
        for i in 0..n {
            let r = self.find(i as i32);
            counts[r as usize] += 1;
        }
        counts.into_iter().filter(|&c| c > 0).collect()
    }

    pub fn get_sizes(&self) -> &Vec<usize> {
        &self.sizes
    }
}
