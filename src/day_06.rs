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

    pub fn num_ways_to_win_big_race(lines: Vec<String>) -> i64 {
        let time = lines[0].chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<i64>().unwrap();
        let distance = lines[1].chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<i64>().unwrap();

        let (mut l, mut r) = (0, time);
        while l < r {
            let start_time = l + (r-l)/2;
            let speed = start_time;
            let d1 = (time - start_time)*speed;
            let d2 = (time - start_time - 1)*(speed+1);
            if d2 > d1 {
                l = start_time + 1;
            } else {
                r = start_time;
            }
        }
        let best_start_time = l;
        let (mut l, mut r) = (0, best_start_time+1);
        while l < r {
            let start_time = l + (r-l)/2;
            let speed = start_time;
            let d = (time - start_time)*speed;
            if d<=distance {
                l = start_time + 1;
            } else {
                r = start_time;
            }
        }
        let win_range_start = l;
        let (mut l, mut r) = (best_start_time+1, time);
        while l < r {
            let start_time = l + (r-l)/2;
            let speed = start_time;
            let d = (time - start_time)*speed;
            if d>distance {
                l = start_time + 1;
            } else {
                r = start_time;
            }
        }
        let win_range_end = l;
        win_range_end - win_range_start
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

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_06.txt").await.unwrap();
        let result = Solution::num_ways_to_win_big_race(lines);
        println!("{result:?}");
    }
}