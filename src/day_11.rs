pub struct Solution;

impl Solution {
    pub fn calculate_galaxy_distances(lines: Vec<String>, expansion_scale: usize) -> i64 {
        let lines = lines.iter().map(|line| line.as_bytes()).collect::<Vec<_>>();
        let (n, m) = (lines.len(), lines[0].len());
        let mut col_galaxy_counts = vec![0; m];
        let mut row_expanders = Vec::with_capacity(n);
        let mut galaxies = Vec::new();
        for (i, &line) in lines.iter().enumerate() {
            let mut is_empty_row = true;
            for (j, &c) in line.iter().enumerate() {
                if c==b'#' {
                    galaxies.push((i, j));
                    col_galaxy_counts[j] += 1;
                    is_empty_row = false;
                }
            }

            if is_empty_row {
                row_expanders.push(i);
            }
        }
        let col_expanders = col_galaxy_counts.iter().enumerate()
            .filter_map(|(j, &count)| (count==0).then_some(j))
            .collect::<Vec<_>>();

        let mut distance = 0;
        for (i, &(x1, y1)) in galaxies.iter().enumerate() {
            for &(x2, y2) in &galaxies[0..i] {
                let (x1, x2) = if x1<x2 { (x1, x2) } else { (x2, x1) };
                let (y1, y2) = if y1<y2 { (y1, y2) } else { (y2, y1) };
                let row_expansion = row_expanders.partition_point(|&r| r<x2) - row_expanders.partition_point(|&r| r<=x1);
                let col_expansion = col_expanders.partition_point(|&c| c<y2) - col_expanders.partition_point(|&c| c<=y1);
                distance += (x2-x1 + y2 - y1 + (expansion_scale-1)*(row_expansion + col_expansion)) as i64;
            }
        }

        distance
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_11.txt").await.unwrap();
        let result = Solution::calculate_galaxy_distances(lines, 2);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_11.txt").await.unwrap();
        let result = Solution::calculate_galaxy_distances(lines, 1_000_000);
        println!("{result:?}");
    }
}