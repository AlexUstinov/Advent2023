pub struct Solution;

fn count_num_ways_impl(data: Vec<(Vec<u8>, Vec<usize>)>) -> usize {
    use std::collections::HashMap;

    fn get_num_ways(dp: &mut HashMap<(usize, usize), usize>, springs: &[u8], nums: &[usize]) -> usize {
        if springs.is_empty() {
            return if nums.is_empty() { 1 } else { 0 };
        }
        let dp_key = (springs.len(), nums.len());
        if let Some(&num) = dp.get(&dp_key) {
            return num;
        }
        let first_spring = springs[0];
        let num = if first_spring == b'.' {
            get_num_ways(dp, &springs[1..], nums)
        } else {
            (if first_spring == b'?' {
                get_num_ways(dp, &springs[1..], nums)
            } else {
                0
            }) + {
                if let Some(first_num) = nums.get(0).map(|&num| num as usize) {
                    if first_num > springs.len() {
                        0
                    } else {
                        let can_match =
                            springs[..first_num].iter().all(|&s| s == b'?' || s == b'#');
                        if can_match
                            && (first_num == springs.len()
                                || springs[first_num] == b'.'
                                || springs[first_num] == b'?')
                        {
                            get_num_ways(
                                dp,
                                &springs[(first_num + 1).min(springs.len())..],
                                &nums[1..],
                            )
                        } else {
                            0
                        }
                    }
                } else {
                    0
                }
            }
        };
        dp.insert(dp_key, num);
        num
    }

    let mut total_possibilities = 0;
    for (springs, nums) in data {
        let mut dp = HashMap::new();
        total_possibilities += get_num_ways(&mut dp, &springs[..], &nums[..]);
    }
    total_possibilities

}

impl Solution {
    pub fn count_num_ways(lines: Vec<String>, unfold: bool) -> usize {
        let mut data = Vec::new();
        for line in lines {
            let (mut springs, mut nums) = line
                .split_once(' ')
                .map(|(springs, nums)| {
                    (
                        Vec::from(springs.as_bytes()),
                        nums.split(',')
                            .filter_map(|num| num.parse::<usize>().ok())
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap();
            if unfold {
                let len = springs.len();
                springs = std::iter::repeat(springs.into_iter().chain(std::iter::once(b'?'))).flatten().take(len*5 + 4).collect();
                nums = std::iter::repeat(nums.into_iter()).take(5).flatten().collect();
            }
            data.push((springs, nums));
        }
        count_num_ways_impl(data)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_12.txt").await.unwrap();
        let result = Solution::count_num_ways(lines, false);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_12.txt").await.unwrap();
        let result = Solution::count_num_ways(lines, true);
        println!("{result:?}");
    }
}
