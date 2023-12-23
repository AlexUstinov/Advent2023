use std::collections::{HashSet, HashMap};
pub struct Solution;

enum Action { Explore(usize, i32), Restore(usize) }

impl Solution {
    pub fn find_hike_len(lines: Vec<Vec<u8>>, slippery_slopes: bool) -> i32 {
        let g = build_graph(&lines, slippery_slopes);
        let end_node = g.len()-1;
        let mut visited = HashSet::new();
        let mut stack = vec![Action::Explore(0, 0)];
        let mut max_len = 0;
        while let Some(action) = stack.pop() {
            match action {
                Action::Explore(node, len) => {
                if node==end_node {
                    max_len = max_len.max(len);
                    continue;
                }
                if visited.insert(node) {
                        stack.push(Action::Restore(node));
                        for &(adj_node, edge_len) in g.get(&node).into_iter().flatten() {
                            stack.push(Action::Explore(adj_node, len + edge_len));
                        }
                    }
                },
                Action::Restore(node) => {
                    visited.remove(&node);
                }
            }
        }

        max_len
    }
}

fn build_graph(lines: &[Vec<u8>], is_directed: bool) -> HashMap<usize, Vec<(usize, i32)>> {
    fn get_adj_cells((i,j): (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
        [(i>0).then(|| (i-1, j)), Some((i, j+1)), Some((i+1, j)), (j>0).then(|| (i, j-1))].into_iter().filter_map(|adj| adj)
    }
    let (n, m) = (lines.len(), lines[0].len());
    let start = (0usize, lines[0].iter().position(|&c| c==b'.').unwrap());
    let end = (n-1, lines[n-1].iter().position(|&c| c==b'.').unwrap());
    let mut joint_points = HashMap::new();
    joint_points.insert(start, joint_points.len());
    for (i, ln) in lines.iter().enumerate().skip(1).take(n-2) {
        for (j, &cell) in ln.iter().enumerate().skip(1).take(m-2) {
            if cell!=b'#' && get_adj_cells((i, j)).filter(|&(ii,jj)| lines[ii][jj]==b'#').count() < 2 {
                joint_points.insert((i,j), joint_points.len());
            }
        }
    }
    joint_points.insert(end, joint_points.len());

    let follow_path = |start, block| {
        let mut pt = start;
        let mut prev_pt = block;
        let mut len = 1;
        while joint_points.get(&pt).is_none() {
            len += 1;
            let next_pt = get_adj_cells(pt).filter(|&(ii, jj)| lines[ii][jj]!=b'#' && (ii,jj)!=prev_pt).next().unwrap();
            (prev_pt, pt) = (pt, next_pt);
        }
        (joint_points[&pt], len)
    };

    let mut g = HashMap::new();
    let next_after_start = get_adj_cells(start).filter(|&(ii, jj)| lines[ii][jj]!=b'#').next().unwrap();
    let (end_id, len) = follow_path(next_after_start, start);
    g.entry(0).or_insert_with(Vec::new).push((end_id, len));
    if !is_directed {
        g.entry(end_id).or_insert_with(Vec::new).push((0, len));
    }
    g.insert(joint_points.len()-1, vec![]);

    for (&(i, j), &id) in joint_points.iter().filter(|(&pt, _)| pt!=end && pt!=start) {
        let outgoing_paths = [(i-1, j, b'^'), (i, j+1, b'>'), (i+1, j, b'v'), (i, j-1, b'<')];
        for start_pt in outgoing_paths.into_iter().filter_map(|(ii, jj, marker)| (lines[ii][jj]==marker).then_some((ii, jj))) {
            let (end_id, len) = follow_path(start_pt, (i, j));
            g.entry(id).or_insert_with(Vec::new).push((end_id, len));
            if !is_directed {
                g.entry(end_id).or_insert_with(Vec::new).push((id, len));
            }
        }
    }

    g
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;
    
    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_23.txt").await.unwrap();
        let result = Solution::find_hike_len(lines.iter().map(|ln| ln.bytes().collect::<Vec<u8>>()).collect::<Vec<_>>(), true);
        println!("{result:?}")
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_23.txt").await.unwrap();
        let result = Solution::find_hike_len(lines.iter().map(|ln| ln.bytes().collect::<Vec<u8>>()).collect::<Vec<_>>(), false);
        println!("{result:?}")
    }
}