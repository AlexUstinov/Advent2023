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
            let num_if_first_works = (first_spring==b'?').then(|| get_num_ways(dp, &springs[1..], nums)).unwrap_or(0);
            let num_if_first_doesnt_work = nums.get(0)
                .map(|&broken_len| broken_len as usize)
                .filter(|&broken_len| broken_len<=springs.len() && springs[..broken_len].iter().all(|&s| s == b'?' || s == b'#'))
                .filter(|&broken_len| broken_len==springs.len() || springs[broken_len]!=b'#')
                .map_or(0, |broken_len| get_num_ways(dp, &springs[(broken_len + 1).min(springs.len())..], &nums[1..],));

            num_if_first_works + num_if_first_doesnt_work
        };
        dp.insert(dp_key, num);
        num
    }

    data.into_iter()
        .map(|(springs, nums)| get_num_ways(&mut HashMap::new(), &springs[..], &nums[..]))
        .sum()
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
