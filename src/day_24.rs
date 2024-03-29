pub struct Solution;

impl Solution {
    pub fn get_num_crossing_points(lines: Vec<String>, test_min: i128, test_max: i128) -> i64 {
        let (min_x, max_x) = (test_min, test_max);
        let (min_y, max_y) = (test_min, test_max);
        let hailstones = parse_lines(lines);
        let mut collision_count = 0;
        for (i, &((p1x, p1y, _), (v1x, v1y, _))) in hailstones.iter().enumerate().skip(1) {
            for &((p2x, p2y, _), (v2x, v2y, _)) in &hailstones[0..i] {
                let (n1x, n1y) = (-v1y, v1x);
                let (n2x, n2y) = (-v2y, v2x);
                // n1(x-x0) + n2(y-y0) = 0

                let denom = n1x*n2y - n2x*n1y;
                if denom == 0 {
                    // lines do not cross
                    continue;
                }
                let x_nom = n1x*n2y*p1x - n2x*n1y*p2x + n1y*n2y*(p1y - p2y);
                let y_nom = n1x*n2y*p2y - n2x*n1y*p1y + n1x*n2x*(p2x - p1x);
                let x_cross = x_nom / denom;
                let y_cross = y_nom / denom;
                if x_cross >= min_x && (x_cross < max_x || (x_cross==max_x && x_nom/denom==0)) && y_cross >= min_y && (y_cross < max_y  || (y_cross==max_y && y_nom/denom==0)) {
                    let (d1x, d1y) = (x_cross - p1x, y_cross - p1y);
                    let (d2x, d2y) = (x_cross - p2x, y_cross - p2y);
                    let is_1st_in_future = ((v1x==0 && d1x==0) || ((v1x>0) == (d1x>0))) && ((v1y==0 && d1y==0) || ((v1y>0) == (d1y>0)));
                    let is_2nd_in_future = ((v2x==0 && d2x==0) || ((v2x>0) == (d2x>0))) && ((v2y==0 && d2y==0) || ((v2y>0) == (d2y>0)));
                    if is_1st_in_future && is_2nd_in_future {
                        collision_count += 1;
                    }
                }

            }
        }


        collision_count
    }

    // Part 2 (code) was very fun. Given explicit mention of integer positions and velocities,
    // I realized that if rock's initial position is (x_r, y_r, z_r) and it has velocity (dx_r, dy_r, dz_r) then
    // for each hailstone i, (x_r + y_r + z_r) ≡ (x_i + y_i + z_i) mod ((dx_i+dy_i+dz_i) - (dx_r+dy_r+dz_r)).
    // The insight is that we don't care about individual dimensions, only sums of all dimensions.
    // Therefore, I iterate over what I though of as a reasonable range for values of dx_r+dy_r+dz_r based on inputs,
    // apply Chinese remainder theorem to compute x_r+y_r+z_r candidates, eliminate any candidates that result
    // in any collisions in the past or at non-integer time, and get at the answer.

    pub fn get_start_position(lines: Vec<String>) -> i64 {
        // (x_r + y_r + z_r) ≡ (x_i + y_i + z_i) mod ((dx_i+dy_i+dz_i) - (dx_r+dy_r+dz_r))
        // https://github.com/admp/aoc-2023/blob/main/24/p2.py
        let hailstones = parse_lines(lines);

        // ans 786617045860267

        // s_r = x_r+y_r+z_r
        // sd_r = dx_r+dy_r+dz_r
        let (mut s, mut sd) = (vec![], vec![]);
        for ((x, y, z), (dx, dy, dz)) in hailstones {
            s.push(x+y+z);
            sd.push(dx+dy+dz);
        }

        0
    }
}

fn parse_lines(lines: Vec<String>) -> Vec<((i128, i128, i128), (i128, i128, i128))> {
    let mut result = Vec::with_capacity(lines.len());
    for ln in lines {
        let (pos, v) = ln.split_once("@").unwrap();
        let (pos, v) = (pos.split(',').flat_map(|num| num.trim().parse::<i128>()).collect::<Vec<_>>(), v.split(',').flat_map(|num| num.trim().parse::<i128>()).collect::<Vec<_>>());
        result.push(((pos[0], pos[1], pos[2]), (v[0], v[1], v[2])));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        // let lines = load_lines("day_24_sample.txt").await.unwrap();
        // let result = Solution::solve1(lines, 7, 27);
        let lines = load_lines("day_24.txt").await.unwrap();
        let result = Solution::get_num_crossing_points(lines, 200000000000000, 400000000000000);
        println!("{result:?}")
    }

    #[tokio::test]
    async fn solve2() {
        // let lines = load_lines("day_24_sample.txt").await.unwrap();
        // let result = Solution::solve1(lines, 7, 27);
        let lines = load_lines("day_24.txt").await.unwrap();
        let result = Solution::get_start_position(lines);
        println!("{result:?}")
    }

}