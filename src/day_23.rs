use std::collections::{HashSet, HashMap};
pub struct Solution;

enum Action { Explore(Vec<(usize, usize)>), Restore(Vec<(usize, usize)>) }

impl Solution {
    pub fn find_hike_len(lines: Vec<Vec<u8>>) -> i32 {
        fn get_move(pt: u8) -> Option<(i32, i32)> {
            match pt {
                b'^' => Some((-1, 0)),
                b'>' => Some((0, 1)),
                b'v' => Some((1, 0)),
                b'<' => Some((0, -1)),
                _ => None
            }
        }

        let (n, m) = (lines.len(), lines[0].len());
        let get_directions = |(x, y): (usize, usize)| {
            let moves = [
                (x > 0).then(|| (x-1, y)),
                (y < m-1).then(|| (x, y+1)),
                (x < n-1).then(|| (x+1, y)),
                (y > 0).then(|| (x, y-1))
            ];
            moves.into_iter().flatten()
                .filter(|&(x, y)| lines[x][y]!=b'#')
                .map(|(mut x, mut y)| {
                    let mut steps = vec![(x, y)];
                    while let Some((dx, dy)) = get_move(lines[x][y]) {
                        let is_safe_move_by_x = (dx>=0 && ((x as i32 + dx) as usize) < n) || (dx<0 && x>0);
                        let is_safe_move_by_y = (dy>=0 && ((y as i32 + dy) as usize) < m) || (dy<0 && y>0);
                        if is_safe_move_by_x && is_safe_move_by_y {
                            let next_x = (x as i32 + dx) as usize;
                            let next_y = (y as i32 + dy) as usize;
                            if lines[next_x][next_y]==b'#' {
                                break;
                            }
                            (x, y) = (next_x, next_y);
                            steps.push((x, y));
                        }

                    }
                    steps
                })
        };
        let start = (0usize, lines[0].iter().position(|&c| c==b'.').unwrap());
        let end = (n-1, lines[n-1].iter().position(|&c| c==b'.').unwrap());
        let mut visited = HashSet::new();
        let mut stack = vec![Action::Explore(vec![start])];
        let mut max_len = 0;
        while let Some(action) = stack.pop() {
            match action {
                Action::Explore(mut steps) => {
                    let pos = steps[steps.len()-1];
                    let mut insertion_count = 0;
                    for &step in steps.iter() {
                        if visited.insert(step) {
                            insertion_count += 1;
                            continue;
                        }
                        break;
                    }
                    let can_go = insertion_count == steps.len();
                    if can_go {
                        stack.push(Action::Restore(steps));
                        if pos==end {
                            max_len = max_len.max(visited.len());
                            continue;
                        }
                        for next_steps in get_directions(pos) {
                            stack.push(Action::Explore(next_steps));
                        }
                    }
                    else {
                        steps.truncate(insertion_count);
                        stack.push(Action::Restore(steps));
                    }
                },
                Action::Restore(steps) => {
                    for step in steps {
                        visited.remove(&step);
                    }
                }
            }
        }

        max_len as i32 - 1
    }

    pub fn find_long_hike_len(lines: Vec<Vec<u8>>) -> i32 {
        let g = build_graph(&lines[..]);
        0
    }

    
}

fn build_graph(lines: &[Vec<u8>]) -> HashMap<usize, Vec<(usize, i32)>> {
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
        let mut len = 0;
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
    g.entry(end_id).or_insert_with(Vec::new).push((0, len));

    for (&(i, j), &id) in joint_points.iter().filter(|(&pt, _)| pt!=end && pt!=start) {
        let outgoing_paths = [(i-1, j, b'^'), (i, j+1, b'>'), (i+1, j, b'v'), (i, j-1, b'<')];
        for start_pt in outgoing_paths.into_iter().filter_map(|(ii, jj, marker)| (lines[ii][jj]==marker).then_some((ii, jj))) {
            let (end_id, len) = follow_path(start_pt, (i, j));
            g.entry(id).or_insert_with(Vec::new).push((end_id, len));
            g.entry(end_id).or_insert_with(Vec::new).push((id, len));
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
        let result = Solution::find_hike_len(lines.iter().map(|ln| ln.bytes().collect::<Vec<u8>>()).collect::<Vec<_>>());
        println!("{result:?}")
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_23.txt").await.unwrap();
        let result = Solution::find_long_hike_len(lines.iter().map(|ln| ln.bytes().collect::<Vec<u8>>()).collect::<Vec<_>>());
        println!("{result:?}")
    }
}