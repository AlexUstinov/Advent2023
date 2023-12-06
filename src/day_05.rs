pub struct Solution;

impl Solution {
    pub fn find_location(lines: Vec<String>) -> i64 {
        let mut seeds = Vec::new();
        let mut active_map_id = 0;
        let mut mapping_chain: Vec<Vec<(i64, i64, i64)>> = Vec::new();
        for line in lines.iter().filter(|line| !line.is_empty()) {
            if line.starts_with("seeds") {
                seeds.extend(line[7..].split(' ').filter_map(|num| num.parse::<i64>().ok()));
            } else {
                let bytes = line.as_bytes();
                if bytes[0].is_ascii_digit() {
                    let mapping = line.split(' ').filter_map(|num| num.parse::<i64>().ok()).collect::<Vec<_>>();
                    mapping_chain[active_map_id].push((mapping[1], mapping[0], mapping[2]));
                } else {
                    mapping_chain.push(Vec::new());
                    active_map_id = mapping_chain.len()-1;
                }
            }
        }

        fn map_seed(seed: i64, mapping_chain: &[Vec<(i64, i64, i64)>]) -> i64 {
            let mut result = seed;
            for map in mapping_chain {
                for &(source, destination, len) in map {
                    if result>=source && result < source + len {
                        result = destination + (result - source);
                        break;
                    }
                }
            }
            result
        }

        let mut min_location = i64::MAX;

        for seed in seeds {
            min_location = min_location.min(map_seed(seed, &mapping_chain));
        }

        min_location
    }

    
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_05.txt").await.unwrap();
        let result = Solution::find_location(lines);
        println!("{result:?}");
    }
}