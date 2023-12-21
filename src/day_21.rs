use std::collections::{HashSet, HashMap, VecDeque};
pub struct Solution;

fn find_start(garden: &[Vec<u8>]) -> (usize, usize) {
    for (i, row) in garden.iter().enumerate() {
        for (j, &plot) in row.iter().enumerate() {
            if plot==b'S' {
                return (i, j);
            }
        }
    }
    unreachable!();
}

impl Solution {
    pub fn get_num_of_reachable_plots(garden: Vec<Vec<u8>>, steps: i64) -> i64 {
        let start = find_start(&garden[..]);
        count_num_of_reachable_points(start, steps, &garden)
    }

    pub fn get_extended_num_of_reachable_plots(garden: Vec<Vec<u8>>, steps: i64) -> i64 {
        let (n, m) = (garden.len(), garden[0].len());
        let (x0, y0) = find_start(&garden[..]);

        let (up, right, left, down) = get_side_start_points(&garden, x0, y0);
        let (odd_reachable, even_reachable) = get_odd_even_reachable(&garden, x0, y0);
        let beam_start_points = [(0, up), (right, m-1), (n-1, down), (left, 0)];
        let corner_points = [(0, 0), (0, m-1), (n-1, m-1), (n-1, 0)];
        let dimensions = [n, m];

        let mut distances = HashMap::new();
        for pt in beam_start_points.iter().chain(corner_points.iter()).copied() {
            distances.insert(pt, None);
        }
        calculate_distances(&mut distances, &garden, x0, y0);

        let is_odd_num_of_steps = steps%2 > 0;
        let mut total_count = if is_odd_num_of_steps { odd_reachable } else { even_reachable }; // for the central segment
        // calculate beam counts
        for (i, start) in beam_start_points.iter().copied().enumerate() {
            let dist = distances[&start].unwrap();
            let steps_to_go = steps - dist;
            let segment_size = dimensions[i % 2] as i64;
            let last_segment_steps = if steps_to_go % segment_size == 0 { segment_size } else { steps_to_go % segment_size };
            let num_segments = steps_to_go / segment_size - (steps_to_go % segment_size == 0).then_some(1).unwrap_or(0);
            total_count += count_num_of_reachable_points(start, last_segment_steps-1, &garden); // one step to "step in"
            let first_beam_segment_is_odd = !is_odd_num_of_steps;
            let half_num_of_segments = num_segments / 2;
            total_count += if first_beam_segment_is_odd {
                half_num_of_segments * even_reachable + (num_segments-half_num_of_segments)*odd_reachable
            } else {
                half_num_of_segments * odd_reachable + (num_segments-half_num_of_segments)*even_reachable
            }
        }

        // calculate quadrants
        for (i, start) in corner_points.iter().copied().enumerate() {
            let dist = distances[&start].unwrap();
            let steps_to_go = steps - dist - 1;
            let segment_size = dimensions[i % 2] as i64; // assume dimensions are equal
            let last_segment_steps = if steps_to_go % segment_size == 0 { segment_size } else { steps_to_go % segment_size };
            let num_segments = steps_to_go / segment_size - (steps_to_go % segment_size == 0).then_some(1).unwrap_or(0);
            let last_segment_points = count_num_of_reachable_points(start, last_segment_steps-1, &garden); // one step to "step in"
            let last_line_size = (steps_to_go + segment_size - 1) / segment_size;
            total_count += last_line_size * last_segment_points;
            let (odd_segment_count, even_segment_count) = if is_odd_num_of_steps { (odd_reachable, even_reachable) } else { (even_reachable, odd_reachable) };
            let num_of_even_segment_lines = num_segments / 2;
            let num_of_odd_segment_lines = num_segments - num_of_even_segment_lines;
            let num_of_even_segments = num_of_even_segment_lines * (num_of_even_segment_lines+1);
            let num_of_odd_segments = num_of_odd_segment_lines * num_of_odd_segment_lines;
            total_count += num_of_odd_segments*odd_segment_count + num_of_even_segments*even_segment_count;
        }

        total_count
    }
}

fn count_num_of_reachable_points(start: (usize, usize), steps: i64, garden: &Vec<Vec<u8>>) -> i64 {
    let mut queue = VecDeque::from(vec![start]);

    for _ in 0..steps {
        let mut visited = HashSet::new();
        let size = queue.len();
        for _ in 0..size {
            if let Some(plot) = queue.pop_front() {
                for next_plot in enum_next_plots(plot, &garden).filter(|&p| visited.insert(p)) {
                    queue.push_back(next_plot);
                }
            }
        }
    }

    queue.len() as i64
}

fn calculate_distances(distances: &mut HashMap<(usize, usize), Option<i64>>, garden: &[Vec<u8>], x0: usize, y0: usize) {
    let distance_count = distances.len();
    let mut hit_count = 0;

    let mut queue = VecDeque::from(vec![(x0, y0)]);
    let mut visited = HashSet::new();
    let mut distance = 0;
    while hit_count < distance_count && !queue.is_empty() {
        let size = queue.len() as i64;
        for _ in 0..size {
            if let Some(plot) = queue.pop_front() {
                if let Some(opt_dist) = distances.get_mut(&plot) {
                    opt_dist.insert(distance);
                    hit_count += 1;
                }
                for next_plot in enum_next_plots(plot, garden).filter(|&p| visited.insert(p)) {
                    queue.push_back(next_plot);
                }
            }
        }
        distance += 1;
    }
}

fn get_odd_even_reachable(garden: &[Vec<u8>], x0: usize, y0: usize) -> (i64, i64) {
    let (mut odd_reachable, mut even_reachable) = (0, 0);
    let mut queue = VecDeque::from(vec![(x0, y0)]);

    for step in 1..=262 {
        let mut visited = HashSet::new();
        let size = queue.len() as i64;
        if step>=261 {
            if step % 2 == 0 {
                even_reachable = size;
            } else {
                odd_reachable = size;
            }
        }
        for _ in 0..size {
            if let Some(plot) = queue.pop_front() {
                for next_plot in enum_next_plots(plot, garden).filter(|&p| visited.insert(p)) {
                    queue.push_back(next_plot);
                }
            }
        }
    }
    (odd_reachable, even_reachable)
}

fn enum_next_plots((x, y): (usize, usize), garden: &[Vec<u8>]) -> impl Iterator<Item=(usize, usize)> {
    let (n, m) = (garden.len(), garden[0].len());
    [
        (x > 0 && garden[x-1][y]!=b'#').then(||(x-1, y)),
        (y+1 < m && garden[x][y+1]!=b'#').then(||(x, y+1)),
        (x+1 < n && garden[x+1][y]!=b'#').then(||(x+1, y)),
        (y > 0 && garden[x][y-1]!=b'#').then(||(x, y-1))
    ].into_iter().filter_map(|p| p)
}

fn get_side_start_points(garden: &Vec<Vec<u8>>, x0: usize, y0: usize) -> (usize, usize, usize, usize) {
    let (n, m) = (garden.len(), garden[0].len());
    let mut queue = VecDeque::from(vec![(x0, y0)]);
    let mut visited = HashSet::new();

    let (mut up, mut right, mut down, mut left) = (0, 0, 0, 0);
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            if let Some(plot) = queue.pop_front() {
                if plot.0==0 && up==0 {
                    up = plot.1;
                } else if plot.0==n-1 && down==0 {
                    down = plot.1;
                } else if plot.1==0 && left==0{
                    left = plot.0;
                } else if plot.1==m-1 && right==0 {
                    right = plot.0;
                }
                for next_plot in enum_next_plots(plot, &garden[..]).filter(|&p| visited.insert(p)) {
                    queue.push_back(next_plot);
                }
            }
        }
    }
    (up, left, down, right)
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        // let lines = load_lines("day_21_sample.txt").await.unwrap();
        // let result = Solution::get_num_reachable_plots(lines.iter().map(|ln| ln.bytes().collect::<Vec<u8>>()).collect::<Vec<_>>(), 6);
        let lines = load_lines("day_21.txt").await.unwrap();
        let result = Solution::get_num_of_reachable_plots(lines.iter().map(|ln| ln.bytes().collect::<Vec<u8>>()).collect::<Vec<_>>(), 64);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_21.txt").await.unwrap();
        let result = Solution::get_extended_num_of_reachable_plots(lines.iter().map(|ln| ln.bytes().collect::<Vec<u8>>()).collect::<Vec<_>>(), 26501365);
        println!("{result:?}");
    }
}