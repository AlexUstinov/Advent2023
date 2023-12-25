
use rand::Rng;
use std::collections::HashMap;
struct UnionFind {
    root: Vec<usize>,
    rank: Vec<i32>
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            root: (0..size).collect(),
            rank: vec![0; size]
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.root[a] != a {
            self.root[a] = self.find(self.root[a]);
        }
        self.root[a]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let (a, b) = (self.find(a), self.find(b));
        a != b && {
            if self.rank[a]==self.rank[b] {
                self.rank[a] += 1;
            }
            if self.rank[a] > self.rank[b] {
                self.root[b] = a;
            } else {
                self.root[a] = b;
            }
            true
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn disconnect_wires(lines: Vec<String>) -> i32 {
        let (v_count, edges) = parse_graph(lines);
        let e_count = edges.len();
        let mut rng = rand::thread_rng();

        loop {
            let mut uf = UnionFind::new(v_count);
            let mut contraction_count = 0;
            while v_count - contraction_count > 2 {
                let edge_id = rng.gen_range(0..e_count);
                let (from, to) = edges[edge_id];
                if uf.union(from, to) {
                    contraction_count+=1;
                }
            }
            let mut cut_edges = Vec::new();
            for (i, &(from, to)) in edges.iter().enumerate() {
                if uf.find(from)!=uf.find(to) {
                    cut_edges.push(i);
                }
            }
            if cut_edges.len()==3 {
                let mut counts = HashMap::new();
                for v in 0..v_count {
                    *counts.entry(uf.find(v)).or_insert(0) += 1;
                }
                break counts.values().fold(1, |acc, &val| acc*val);
            }
        }
    }

    
}

fn parse_graph(lines: Vec<String>) -> (usize, Vec<(usize, usize)>) {
    let mut next_id = 0usize;
    let mut id_gen = move || {
        let id = next_id;
        next_id += 1;
        id
    };
    let mut vertex_map = HashMap::new();
    let mut edges = Vec::new();
    for ln in lines.iter() {
        let (from, to_iter) = ln.split_once(':')
            .map(|(f, to)| (f.trim(), to.trim().split(' ').map(|to| to.trim()))).unwrap();
        let from_id = *vertex_map.entry(from).or_insert_with(|| id_gen());
        for to in to_iter {
            let to_id = *vertex_map.entry(to).or_insert_with(|| id_gen());
            edges.push((from_id, to_id));
        }
    }
    (vertex_map.len(), edges)
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_25.txt").await.unwrap();
        let result = Solution::disconnect_wires(lines);
        println!("{result:?}");
    }

}