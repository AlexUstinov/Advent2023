use std::usize;

pub struct Solution;

impl Solution {
    pub fn calculate_load(lines: Vec<Vec<u8>>) -> i32 {
        let mut total_load = 0;
        let (n, m) = (lines.len(), lines[0].len());
        for col in 0..m {
            let mut min_pos = 0;
            for row in 0..n {
                let block = lines[row][col];
                match block {
                    b'#' => min_pos = row+1,
                    b'O' => {
                        total_load += (n - min_pos) as i32;
                        min_pos += 1;
                    },
                    _ => continue
                }
            }
        }
        total_load
    }

    pub fn calculate_load_in_cycles(mut platform: Vec<Vec<u8>>, num_rotations: i32) -> i32 {
        fn rotate(mut platform: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
            let (n, m) = (platform.len(), platform[0].len());
            let tilts: [(usize, usize, Box<dyn Fn(usize, usize) -> (usize, usize)>); 4] = [
                (m, n, Box::new(|row, col| (row, col))),
                (n, m, Box::new(|row, col| (n-col-1, row))),
                (m, n, Box::new(|row, col| (m-row-1, n-col-1))),
                (n, m, Box::new(|row, col| (col, m-row-1)))
            ];

            for (row_size, col_size, transform) in tilts {
                for col in 0..col_size {
                    let mut min_pos = 0;
                    for row in 0..row_size {
                        let (tr, tc) = transform(row, col);
                        match platform[tr][tc] {
                            b'#' => min_pos = row + 1,
                            b'O' => {
                                platform[tr][tc] = b'.';
                                let (tr_min, tc_min) = transform(min_pos, col);
                                platform[tr_min][tc_min] = b'O';
                                min_pos += 1;
                            },
                            _ => continue
                        }
                    }
                }
            }
            platform
        }
       
        let mut fast_platform = platform.clone();
        for current_rotation in 1..=num_rotations {
            platform = rotate(platform);
            fast_platform = rotate(rotate(fast_platform));
            if platform==fast_platform {
                let mut cycle_len = 1;
                while {
                    fast_platform = rotate(fast_platform);
                    platform!=fast_platform
                } {
                    cycle_len += 1;
                }
                let remained_rotations = (num_rotations - current_rotation) % cycle_len;
                for _ in 0..remained_rotations {
                    platform = rotate(platform);
                }
                break;
            }
        }

        let (n, m) = (platform.len(), platform[0].len());
        let mut total_load = 0;
        for row in 0..n {
            for col in 0..m {
                if platform[row][col]==b'O' {
                    total_load += (n - row) as i32;
                }
            }
        }
        total_load
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_14.txt").await.unwrap();
        let result = Solution::calculate_load(lines.iter().map(|ln| Vec::from(ln.as_bytes())).collect::<Vec<_>>());
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_14.txt").await.unwrap();
        let result = Solution::calculate_load_in_cycles(lines.iter().map(|ln| Vec::from(ln.as_bytes())).collect::<Vec<_>>(), 1_000_000_000);
        println!("{result:?}");
    }
}