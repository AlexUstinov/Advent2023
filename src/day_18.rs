use std::collections::BTreeMap;
use std::ops::Bound::{Included, Excluded};

pub struct Solution;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Dir {U, R, D, L}

fn to_dir(val: &str) -> Dir {
    match val { "U" => Dir::U, "R" => Dir::R, "D" => Dir::D, "L" => Dir::L, _ => unreachable!() }
}

fn preprocess_input_lines_1(lines: Vec<String>) -> Vec<(Dir, i32)> {
    let mut result = Vec::new();
    for line in lines {
        let parts = line.split(' ').collect::<Vec<_>>();
        result.push((to_dir(parts[0]), parts[1].parse::<i32>().unwrap()));
    }

    result
}

impl Solution {
    pub fn solve1(lines: Vec<String>) -> i64 {
        let commands = preprocess_input_lines_1(lines);
        let (mut h, mut min_h, mut max_h) = (0, i32::MAX, i32::MIN);
        let (mut v, mut min_v, mut max_v) = (0, i32::MAX, i32::MIN);
        for (dir, len) in commands.iter() {
            match dir {
                Dir::U => { v -= len; min_v = min_v.min(v); },
                Dir::R => { h += len; max_h = max_h.max(h); },
                Dir::D => { v += len; max_v = max_v.max(v); },
                Dir::L => { h -= len; min_h = min_h.min(h); },
            }
        }

        let (mut x, mut y) = (min_h.abs(), min_v.abs());
        let mut field = BTreeMap::new();
        for (dir, len) in commands.iter() {
            match dir {
                Dir::U => y-=len,
                Dir::R => x+=len,
                Dir::D => y+=len,
                Dir::L => x-=len,
            }
            _ = field.entry(y).or_insert_with(|| (1, Vec::new()));
        }

        let mut expander_rows = Vec::new();
        for (y1, y2) in field.keys().zip(field.keys().skip(1)) {
            let size = y2-y1-1;
            if size > 0 {
                expander_rows.push((y1+1, size));
            }
        }
        for &(pos, size) in expander_rows.iter() {
            _ = field.entry(pos).or_insert_with(|| (size, Vec::new()));
        }

        // x and y must be at the starting position now

        let mut d_len = 0;
        let first_cmd = &commands[0];
        let commands_iter = commands.iter().zip(commands.iter().skip(1).chain(std::iter::once(first_cmd)));
        for (dir, next_dir, len) in commands_iter.map(|(&(dir, len), &(next_dir, _))| (dir, next_dir, len)) {
            d_len += len;
            let dir_is_vertical = dir==Dir::U || dir==Dir::D;
            let next_dir_is_vertical = next_dir==Dir::U || next_dir==Dir::D;
            if dir_is_vertical && next_dir_is_vertical {
                continue;
            }
            let prev_y = y;
            match dir {
                Dir::U => y-=d_len,
                Dir::R => x+=d_len,
                Dir::D => y+=d_len,
                Dir::L => x-=d_len,
            }
            if dir_is_vertical && !next_dir_is_vertical {
                let cell_dir = if next_dir==Dir::R { [dir, Dir::R] } else { [Dir::L, dir ] };
                if let Some((_, v_crossings)) = field.get_mut(&y) {
                    v_crossings.push((x, cell_dir))
                }
            } else if !dir_is_vertical && next_dir_is_vertical {
                let cell_dir = if dir==Dir::R { [Dir::L, next_dir] } else { [next_dir, Dir::R] };
                if let Some((_, v_crossings)) = field.get_mut(&y) {
                    v_crossings.push((x, cell_dir))
                }

            }
            if dir_is_vertical {
                for (_, (_, v_crossings)) in field.range_mut((Excluded(prev_y.min(y)), Excluded(prev_y.max(y)))) {
                    v_crossings.push((x, [dir, dir]));
                }
            }
            d_len = 0;
        }

        for (_, v_crossings) in field.values_mut() {
            v_crossings.sort_unstable();
        }

        let mut count = 0;
        let interior_is_to_the_right = matches!(field.first_key_value().map(|(_, (_, v_crossings))| v_crossings[0]).unwrap(), (_, [Dir::U, _]));
        for (size, v_crossings) in field.values() {
            let size = *size;
            let mut is_interior = false;
            let (mut prev_x, mut prev_dir) = (0, [Dir::R, Dir::U]);
            for &(x, dir) in v_crossings {
                count += size as i64;
                let is_horizontal_border = matches!((prev_dir[1], dir[0]), (Dir::R, Dir::L));
                if is_interior || is_horizontal_border {
                    let delta = x - prev_x - 1;
                    count += (delta as i64) * (size as i64);
                }
                is_interior = interior_is_to_the_right ^ (dir[1]==Dir::D || dir[0]==Dir::D);
                prev_x = x;
                prev_dir = dir;
            }
        }
        count as i64
    }

    
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_18.txt").await.unwrap();
        let result = Solution::solve1(lines);
        println!("{result:?}")
    }
}