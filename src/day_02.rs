pub struct Solution;

impl Solution {
    pub fn find_playable_games(games: Vec<String>) -> i32 {
        fn get_max(color: &str) -> i32 {
            match color { "red" => 12, "green" => 13, "blue" => 14, _ => unreachable!() }
        }

        let mut playable_game_sum = 0;
        'game_loop: for game in games {
            if let Some((game_name, game_rounds)) = game.split_once(':') {
                for round in game_rounds.split(';').map(|round| round.trim()).filter(|round| !round.is_empty()) {
                    for (num, color) in round.split(',').flat_map(|pair| pair.trim().split_once(' ')).map(|(a,b)| (a.trim(), b.trim())) {
                        let val:i32 = num.parse().unwrap();
                        if val > get_max(color) {
                            continue 'game_loop;
                        }
                    }
                }
                let game_id = game_name.split_once(' ').map(|(_, id)| id.trim()).unwrap();
                playable_game_sum += game_id.parse::<i32>().unwrap();
            }
        }
        playable_game_sum
    }

    pub fn find_aggregate_cube_count(games: Vec<String>) -> i32 {
        fn get_color_idx(color: &str) -> usize {
            match color { "red" => 0, "green" => 1, "blue" => 2, _ => unreachable!() }
        }

        let mut aggregate_cube_count = 0;
        for game in games {
            if let Some((_, game_rounds)) = game.split_once(':') {
                let mut min_cube_counts = [0, 0, 0];
                for round in game_rounds.split(';').map(|round| round.trim()).filter(|round| !round.is_empty()) {
                    for (num, color) in round.split(',').flat_map(|pair| pair.trim().split_once(' ')).map(|(a,b)| (a.trim(), b.trim())) {
                        let val:i32 = num.parse().unwrap();
                        let color_idx = get_color_idx(color);
                        min_cube_counts[color_idx] = min_cube_counts[color_idx].max(val);
                    }
                }
                aggregate_cube_count += min_cube_counts.into_iter().reduce(|a, b| a*b).unwrap_or(0);
            }
        }
        aggregate_cube_count
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve_part1() {
        let file_name: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input", "day_02.txt"].iter().collect();

        let lines = load_lines(file_name).await.unwrap();
        let result = Solution::find_playable_games(lines);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve_part2() {
        let file_name: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input", "day_02.txt"].iter().collect();

        let lines = load_lines(file_name).await.unwrap();
        let result = Solution::find_aggregate_cube_count(lines);
        println!("{result:?}");
    }
}