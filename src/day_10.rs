pub struct Solution;

fn get_allowed_dirs(pipe: u8) -> &'static str {
    match pipe {
        b'|' => "NS",
        b'-' => "WE",
        b'F' => "SE",
        b'7' => "SW",
        b'J' => "NW",
        b'L' => "NE",
        _ => "NESW"
    }    
}

impl Solution {
    pub fn get_farthest_point_distance(lines: Vec<String>) -> i32 {
        use std::collections::{VecDeque, HashSet};

        let lines = lines.iter().map(|line| line.as_bytes()).collect::<Vec<_>>();

        let (mut x0, mut y0) = (0, 0);
        'start_loop: for (i, row) in lines.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c==b'S' {
                    (x0, y0) = (i as i32, j as i32);
                    break 'start_loop;
                }
            }
        }

        let (n, m) = (lines.len() as i32, lines[0].len() as i32);
        let moves = [(-1, 0, 'N'), (0, 1, 'E'), (1, 0, 'S'), (0, -1, 'W')];
        let mut step_count = 0;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert((x0, y0));
        queue.push_back((x0, y0));
        while !queue.is_empty() {
            let size = queue.len();
            let mut is_moved = false;
            for _ in 0..size {
                if let Some((x, y)) = queue.pop_front() {
                    let current_pipe = lines[x as usize][y as usize];
                    let allowed_dirs = get_allowed_dirs(current_pipe);
                    for &(dx, dy, dir) in moves.iter().filter(|&&(_,_,dir)| allowed_dirs.contains(dir)) {
                        let (xx, yy) = (x+dx, y+dy);
                        if xx>=0 && xx < n && yy>=0 && yy < m {
                            let pipe = lines[xx as usize][yy as usize];
                            let is_connected_pipe =
                                matches!((pipe, dir), (b'|' | b'F' | b'7', 'N')) ||
                                matches!((pipe, dir), (b'-' | b'J' | b'7', 'E')) ||
                                matches!((pipe, dir), (b'|' | b'J' | b'L', 'S')) ||
                                matches!((pipe, dir), (b'-' | b'F' | b'L', 'W'));
                            if is_connected_pipe && visited.insert((xx, yy)) {
                                queue.push_back((xx, yy));
                                is_moved = true;
                            }
                        }
                    }
                }
            }
            if is_moved {
                step_count += 1;
            }
        }

        step_count
    }

    pub fn get_inner_loop_area(lines: Vec<String>) -> i32 {
        use std::collections::{VecDeque, HashSet};

        let lines = lines.iter().map(|line| line.as_bytes()).collect::<Vec<_>>();

        let (mut x0, mut y0) = (0, 0);
        'start_loop: for (i, row) in lines.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c==b'S' {
                    (x0, y0) = (i as i32, j as i32);
                    break 'start_loop;
                }
            }
        }

        let (n, m) = (lines.len() as i32, lines[0].len() as i32);
        let moves = [(-1, 0, 'N'), (0, 1, 'E'), (1, 0, 'S'), (0, -1, 'W')];
        let mut loop_pipes = vec![vec![]; n as usize];
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert((x0, y0));
        queue.push_back((x0, y0));
        while let Some((x, y)) = queue.pop_front() {
            let current_pipe = lines[x as usize][y as usize];
            let allowed_dirs = get_allowed_dirs(current_pipe);
            for &(dx, dy, dir) in moves.iter().filter(|&&(_,_,dir)| allowed_dirs.contains(dir)) {
                let (xx, yy) = (x+dx, y+dy);
                if xx>=0 && xx < n && yy>=0 && yy < m {
                    let pipe = lines[xx as usize][yy as usize];
                    let is_connected_pipe =
                        matches!((pipe, dir), (b'|' | b'F' | b'7', 'N')) ||
                        matches!((pipe, dir), (b'-' | b'J' | b'7', 'E')) ||
                        matches!((pipe, dir), (b'|' | b'J' | b'L', 'S')) ||
                        matches!((pipe, dir), (b'-' | b'F' | b'L', 'W'));
                    if is_connected_pipe && visited.insert((xx, yy)) {
                        queue.push_back((xx, yy));
                        if pipe!=b'-' {
                            loop_pipes[xx as usize].push(yy);
                        }
                    }
                }
            }
        }

        let mut area = 0;

        for (x, row) in loop_pipes.iter_mut().enumerate() {
            if !row.is_empty() {
                row.sort_unstable();
                let mut is_out = true;
                let mut prev = 0;
                let mut cross_start_state = (true, 0);
                for &y in row.iter() {
                    let pipe = lines[x][y as usize];
                    let delta = y - prev - 1;
                    if pipe==b'|' {
                        if !is_out {
                            area += delta;
                        }
                        is_out ^= true;
                    } else if matches!(pipe, b'F' | b'L') {
                        if !is_out {
                            area += delta;
                        }
                        cross_start_state = (is_out, pipe);
                        is_out = true;
                    } else {
                        match (cross_start_state, pipe) {
                            ((true, b'F'), b'J') => { is_out = false; },
                            ((true, b'F'), b'7') => {},
                            ((true, b'L'), b'J') => {},
                            ((true, b'L'), b'7') => { is_out = false; },
                            ((false, b'F'), b'J') => {},
                            ((false, b'F'), b'7') => { is_out = false; },
                            ((false, b'L'), b'J') => { is_out = false; },
                            ((false, b'L'), b'7') => {},
                            _ => unreachable!()
                        }
                    }
                    prev = y;
                }
            }
        }

        area
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_10.txt").await.unwrap();
        let result = Solution::get_farthest_point_distance(lines);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_10.txt").await.unwrap();
        let result = Solution::get_inner_loop_area(lines);
        println!("{result:?}");
    }
}