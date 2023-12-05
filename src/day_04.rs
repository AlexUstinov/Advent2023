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

    pub fn count_card_copies(lines: Vec<String>) -> i32 {
        use std::collections::{HashMap, HashSet};

        let mut card_counters = HashMap::<i32, i32>::new();
        for game in lines {
            let (card_id, win_nums, player_nums) = game.split_once(':')
                .map(|(card, nums)| { 
                    let (win_nums, player_nums) = nums.split_once('|').unwrap();
                    (card[5..].trim().parse::<i32>().unwrap(),
                        win_nums.trim().split(' ').filter(|num| num.len()>0),
                        player_nums.trim().split(' ').filter(|num| num.len()>0))
                })
                .unwrap();            
            let card_count = card_counters.entry(card_id).or_insert(0);
            *card_count += 1;
            let card_count = *card_count;
            let mut win_nums_set = HashSet::new();
            win_nums_set.extend(win_nums);
            let match_count = player_nums.filter(|num| win_nums_set.contains(num)).count() as i32;
            if match_count>0 {
                for next_card_id in (1..=match_count).map(|d| card_id + d) {
                    *card_counters.entry(next_card_id).or_insert(0) += card_count;
                }
            }
        }

        card_counters.values().sum()
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

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_04.txt").await.unwrap();
        let result = Solution::count_card_copies(lines);
        println!("{result:?}");
    }
}