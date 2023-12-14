pub struct Solution;

impl Solution {
    pub fn solve1(lines: Vec<String>) -> i32 {
        todo!()
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
        let result = Solution::solve1(lines);
        println!("{result:?}");
    }

}