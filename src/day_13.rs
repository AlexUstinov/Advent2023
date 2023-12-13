pub struct Solution;

impl Solution {
    pub fn find_reflections(input: Vec<&[u8]>, expected_errors: i32) -> i64 {
        fn process_note(lines: &Vec<&[u8]>, expected_errors: i32) -> i64 {
            let len = lines.len() as i32;
            let mut result = 0;
            'row_loop: for (win_id, w) in lines.windows(2).enumerate() {
                let mut error_count = w[0].iter().zip(w[1].iter()).map(|(a, b)| if a==b {0} else {1}).sum::<i32>();
                if error_count<=expected_errors {
                    let (mut i, mut j) = (win_id as i32 - 1, win_id as i32 + 2);
                    while i >= 0 && j<len {
                        error_count += lines[i as usize].iter().zip(lines[j as usize].iter()).map(|(a, b)| if a==b {0} else {1}).sum::<i32>();
                        if error_count>expected_errors {
                            continue 'row_loop;
                        }
                        i -= 1;
                        j += 1;
                    }
                    if error_count==expected_errors {
                        result += 100*(win_id as i64 + 1);
                    }
                }
            }
            let row_len = lines[0].len();
            'col_loop: for (col1_id, col2_id) in (1..row_len).map(|j| (j-1, j)) {
                let mut error_count = lines.iter().map(|ln| ln[col1_id]).zip(lines.iter().map(|ln| ln[col2_id]))
                    .map(|(a, b)| if a==b {0} else {1}).sum::<i32>();
                if error_count<=expected_errors {
                    let (mut i, mut j) = (col1_id as i32 - 1, col2_id as i32 + 1);
                    while i >= 0 && j<row_len as i32 {
                        error_count += lines.iter().map(|ln| ln[i as usize]).zip(lines.iter().map(|ln| ln[j as usize]))
                            .map(|(a, b)| if a==b {0} else {1}).sum::<i32>();

                        if error_count>expected_errors {
                                continue 'col_loop;
                        }
                        i -= 1;
                        j += 1;
                    }
                    if error_count==expected_errors {
                        result += col2_id as i64
                    }
                }
            }
            result
        }
        let mut result = 0;
        let mut lines = Vec::new();
        for line in input {
            if !line.is_empty() {
                lines.push(line);
                continue;                
            }

            result += process_note(&lines, expected_errors);

            lines.clear();
        }
        result += process_note(&lines, expected_errors);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_13.txt").await.unwrap();
        let result = Solution::find_reflections(lines.iter().map(|ln| ln.as_bytes()).collect::<Vec<_>>(), 0);
        println!("{result:?}")
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_13.txt").await.unwrap();
        let result = Solution::find_reflections(lines.iter().map(|ln| ln.as_bytes()).collect::<Vec<_>>(), 1);
        println!("{result:?}")
    }
}