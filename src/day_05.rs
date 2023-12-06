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

    pub fn find_better_location(lines: Vec<String>) -> i64 {
        let mut seed_ranges = Vec::new();
        let mut active_map_id = 0;
        let mut mapping_chain: Vec<Vec<(i64, i64, i64)>> = Vec::new();
        for line in lines.iter().filter(|line| !line.is_empty()) {
            if line.starts_with("seeds") {
                let mut seed_iter = line[7..].split(' ').filter_map(|num| num.parse::<i64>().ok());
                while let Some(range) = seed_iter.next().zip(seed_iter.next()) {
                    seed_ranges.push(range);
                }
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

        fn map_seed_ranges(seed_ranges: Vec<(i64, i64)>, mapping_chain: &[Vec<(i64, i64, i64)>]) -> Vec<(i64, i64)> {
            let mut result = seed_ranges;
            for map in mapping_chain {
                let mut next_ranges = Vec::new();
                for &(source, destination, len) in map {
                    for &(range_start, range_len) in result.iter() {
                        if range_start < source + len && range_start + range_len > source {
                            let intersection_start = source.max(range_start);
                            let intersection_len = (range_start + range_len).min(source+len) - intersection_start;
                            next_ranges.push((destination + intersection_start - source, intersection_len));
                        }
                    }
                }
                result = next_ranges;
            }
            result
        }

        map_seed_ranges(seed_ranges, &mapping_chain).into_iter().map(|(start, _)| start).min().unwrap()
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

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_05.txt").await.unwrap();
        let result = Solution::find_better_location(lines);
        println!("{result:?}");
    }
}