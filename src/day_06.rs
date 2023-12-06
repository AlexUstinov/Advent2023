pub struct Solution;

impl Solution {
    pub fn num_ways_to_win(lines: Vec<String>) -> i64 {
        let time = lines[0].split_once(':').map(|(_, nums)| nums).unwrap()
            .split(' ').filter_map(|num| num.parse::<i64>().ok()).collect::<Vec<_>>();
        let distance = lines[1].split_once(':').map(|(_, nums)| nums).unwrap()
            .split(' ').filter_map(|num| num.parse::<i64>().ok()).collect::<Vec<_>>();

        let mut aggregated_num_ways = 1;
        for (t, d) in time.into_iter().zip(distance.into_iter()) {
            let mut num_ways = 0;
            let mut speed = 0;
            for start in 0..=t {
                let travel_distance = (t-start)*speed;
                if travel_distance > d {
                    num_ways += 1;
                }
                speed += 1;
            }

            aggregated_num_ways *= num_ways;
        }
        aggregated_num_ways
    }

    
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_06.txt").await.unwrap();
        let result = Solution::num_ways_to_win(lines);
        println!("{result:?}");
    }
}