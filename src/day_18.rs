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

        let (x0, y0) = (min_h.abs() as usize, min_v.abs() as usize);
        let height = (max_v-min_v+1) as usize;
        let mut field = vec![vec![]; height];
        let first_cmd = &commands[0];
        let commands_iter = commands.iter().zip(commands.iter().skip(1).chain(std::iter::once(first_cmd)));
        let (mut x, mut y) = (x0, y0);
        for (dir, next_dir, len) in commands_iter.map(|(&(dir, len), &(next_dir, _))| (dir, next_dir, len)) {
            for _ in 0..len {
                match dir {
                    Dir::U => y-=1,
                    Dir::R => x+=1,
                    Dir::D => y+=1,
                    Dir::L => x-=1,
                }
                let dir_is_vertical = dir==Dir::U || dir==Dir::D;
                let next_dir_is_vertical = next_dir==Dir::U || next_dir==Dir::D;
                if dir_is_vertical || next_dir_is_vertical {
                    let v_dir = if dir_is_vertical { dir } else { next_dir };
                    field[y].push((x, v_dir));
                }
            }
        }

        for row in field.iter_mut() {
            row.sort_unstable();
        }

        let interior_is_to_the_right = matches!(field[0][0], (_, Dir::U));
        let mut count = 0;
        for row in field {
            let mut is_interior = false;
            let (mut prev_x, mut prev_v_dir) = (0, Dir::R);
            for (x, v_dir) in row {
                count += 1;
                let shifted_cross = v_dir==prev_v_dir;
                if is_interior || shifted_cross {
                    let delta = x - prev_x - 1;
                    count += delta;
                }
                is_interior = interior_is_to_the_right ^ (v_dir==Dir::D);
                prev_x = x;
                prev_v_dir = v_dir;
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