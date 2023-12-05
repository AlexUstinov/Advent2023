pub struct Solution;

impl Solution {
    pub fn compute_score(lines: Vec<String>) -> i64 {
        use std::collections::HashSet;

        let mut total_score = 0;
        for card in lines {
            let mut match_count = 0;
            let (win_nums, player_nums) = card.split_once(':').map(|(_, nums)| nums.split_once('|')).flatten().unwrap();
            let mut win_nums_set = HashSet::new();
            win_nums_set.extend(win_nums.split(' ').filter(|num| num.len()>0));
            for num in player_nums.split(' ').filter(|num| num.len()>0) {
                if win_nums_set.contains(num) {
                    match_count += 1;
                }
            }
            if match_count > 0 {
                total_score += 1 << (match_count-1);
            }
        }
        total_score
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_04.txt").await.unwrap();
        let result = Solution::compute_score(lines);
        println!("{result:?}");
    }

}