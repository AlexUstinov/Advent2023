pub struct Solution;

impl Solution {
    pub fn total_win_amount(lines: Vec<String>) -> i32 {
        fn get_hand_key(hand: &str) -> (i32, i32) {
            fn get_val(c: char) -> i32 {
                match c {'A'=>14, 'K'=>13, 'Q'=>12, 'J'=>11, 'T'=>10, _ => (c as u8 - b'0') as i32 }
            }
            let mut freqs = [0; 15];
            let mut total_val = 0;
            for c in hand.chars() {
                let val = get_val(c);
                freqs[val as usize] += 1;
                total_val = total_val*15 + val;
            }
            let mut count = 0;
            let mut max_freq = 0;
            for f in freqs {
                if f>0 {
                    count+=1;
                    max_freq = max_freq.max(f);
                }
            }
            let rank = match (count, max_freq) {
                (1, 5) => 7,
                (2, 4) => 6,
                (2, 3) => 5,
                (3, 3) => 4,
                (3, 2) => 3,
                (4, 2) => 2,
                _ => 1
            };

            (rank, total_val)
        }
        let mut hands = Vec::with_capacity(lines.len());
        for line in lines {
            let (hand, bid) = line.split_once(' ').map(|(hand, num)| (hand, num.trim().parse::<i32>().unwrap())).unwrap();
            hands.push((get_hand_key(hand), bid));
        }
        hands.sort_unstable_by_key(|(key, _)| *key);
        
        hands.iter().enumerate().map(|(idx, &(_, bid))| ((idx+1) as i32)*bid).sum::<i32>()
    }

    pub fn total_win_amount_tweaked(lines: Vec<String>) -> i64 {
        fn get_hand_key(hand: &str) -> (i64, i64) {
            fn get_val(c: char) -> i64 {
                match c {'A'=>14, 'K'=>13, 'Q'=>12, 'J'=>0, 'T'=>10, _ => (c as u8 - b'0') as i64 }
            }
            let mut freqs = [0; 15];
            let mut total_val = 0;
            for c in hand.chars() {
                let val = get_val(c);
                freqs[val as usize] += 1;
                total_val = total_val*15 + val;
            }
            let mut count = 0;
            let mut max_freq = 0;
            let j_freq = freqs[0];
            let j_count = if j_freq>0 {1} else {0};
            for (i, f) in freqs.into_iter().enumerate() {
                if f>0 {
                    if i!=0 {
                        max_freq = max_freq.max(f);
                    }
                    count+=1;
                }
            }
            let rank = match (count-j_count, max_freq+j_freq) {
                (_, 5) => 7,
                (2, 4) => 6,
                (2, 3) => 5,
                (3, 3) => 4,
                (3, 2) => 3,
                (4, 2) => 2,
                _ => 1
            };

            (rank, total_val)
        }
        let mut hands = Vec::with_capacity(lines.len());
        for line in lines {
            let (hand, bid) = line.split_once(' ').map(|(hand, num)| (hand, num.trim().parse::<i64>().unwrap())).unwrap();
            hands.push((get_hand_key(hand), bid));
        }
        hands.sort_unstable_by_key(|(key, _)| *key);
        
        hands.iter().enumerate().map(|(idx, &(_, bid))| ((idx+1) as i64)*bid).sum::<i64>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_07.txt").await.unwrap();
        let result = Solution::total_win_amount(lines);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_07.txt").await.unwrap();
        let result = Solution::total_win_amount_tweaked(lines);
        println!("{result:?}");
    }
}