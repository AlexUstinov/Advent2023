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

    pub fn calculate_load_in_cycles(mut lines: Vec<Vec<u8>>, num_cycles: i32) -> i32 {
        let (n, m) = (lines.len(), lines[0].len());
        for cycle in 0..num_cycles {
            // tilt north
            for col in 0..m {
                let mut min_pos = 0;
                for row in 0..n {
                    match lines[row][col] {
                        b'#' => min_pos = row + 1,
                        b'O' => {
                            lines[row][col] = b'.';
                            lines[min_pos][col] = b'O';
                            min_pos += 1;
                        },
                        _ => continue
                    }
                }
            }
            // tilt west
            for row in 0..n {
                let mut min_pos = 0;
                for col in 0..m {
                    match lines[row][col] {
                        b'#' => min_pos = col + 1,
                        b'O' => {
                            lines[row][col] = b'.';
                            lines[row][min_pos] = b'O';
                            min_pos += 1;
                        },
                        _ => continue
                    }
                }
            }
            // tilt south
            for col in 0..m {
                let mut min_pos = n-1;
                for row in (0..n).rev() {
                    match lines[row][col] {
                        b'#' => min_pos = row.saturating_sub(1),
                        b'O' => {
                            lines[row][col] = b'.';
                            lines[min_pos][col] = b'O';
                            min_pos = min_pos.saturating_sub(1);
                        },
                        _ => continue
                    }
                }
            }
            // tilt east
            for row in 0..n {
                let mut min_pos = m-1;
                for col in (0..m).rev() {
                    match lines[row][col] {
                        b'#' => min_pos = col.saturating_sub(1),
                        b'O' => {
                            lines[row][col] = b'.';
                            lines[row][min_pos] = b'O';
                            min_pos = min_pos.saturating_sub(1);
                        },
                        _ => continue
                    }
                }
            }
            let mut total_load = 0;
            for row in 0..n {
                for col in 0..m {
                    if lines[row][col]==b'O' {
                        total_load += (n - row) as i32;
                    }
                }
            }
            println!("{}: {}", cycle, total_load)

            // let mut platform = Vec::new();
            // for line in lines.iter() {
            //     platform.push(String::from_utf8(line.clone()).unwrap());
            // }
            // for platform_line in platform {
            //     println!("{platform_line:?}");
            // }
            // println!();
        }

        let mut total_load = 0;
        for row in 0..n {
            for col in 0..m {
                if lines[row][col]==b'O' {
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
        let result = Solution::calculate_load_in_cycles(lines.iter().map(|ln| Vec::from(ln.as_bytes())).collect::<Vec<_>>(), 1000);
//        let result = Solution::calculate_load_in_cycles(lines.iter().map(|ln| Vec::from(ln.as_bytes())).collect::<Vec<_>>(), 1_000_000_000);
        println!("{result:?}");
    }
}