pub struct Solution;

enum Action { Visit(Vec<i32>, Option<usize>), Extrapolate(i32, i32, Option<usize>) }

impl Solution {
    pub fn extrapolate_sequences(lines: Vec<String>) -> i32 {
        let mut total = 0;
        for line_nums in lines.into_iter().map(|line| line.split(' ').filter_map(|val| val.parse::<i32>().ok()).collect::<Vec<_>>()) {
            let mut stack = vec![Action::Visit(line_nums, None)];
            while let Some(action) = stack.pop() {
                match action {
                    Action::Visit(line_nums, parent_id) => {
                        let diffs = line_nums.windows(2).map(|w| w[1]-w[0]).collect::<Vec<_>>();
                        stack.push(Action::Extrapolate(line_nums[line_nums.len()-1], 0, parent_id));
                        if diffs.iter().any(|&num| num!=0) {
                            stack.push(Action::Visit(diffs, Some(stack.len()-1)));
                        }
                    },
                    Action::Extrapolate(last, diff, parent_id) => {
                        let next_val = last+diff;
                        if let Some(parent_id) = parent_id {
                            if let Some(Action::Extrapolate(_, ref mut parent_diff, _)) = stack.get_mut(parent_id) {
                                *parent_diff = next_val;
                            }
                        } else {
                            total += next_val;
                        }
                    }
                }
            }
        }
        total
    }

    pub fn extrapolate_sequences_backward(lines: Vec<String>) -> i32 {
        let mut total = 0;
        for line_nums in lines.into_iter().map(|line| line.split(' ').filter_map(|val| val.parse::<i32>().ok()).collect::<Vec<_>>()) {
            let mut stack = vec![Action::Visit(line_nums, None)];
            while let Some(action) = stack.pop() {
                match action {
                    Action::Visit(line_nums, parent_id) => {
                        let diffs = line_nums.windows(2).map(|w| w[1]-w[0]).collect::<Vec<_>>();
                        stack.push(Action::Extrapolate(line_nums[0], 0, parent_id));
                        if diffs.iter().any(|&num| num!=0) {
                            stack.push(Action::Visit(diffs, Some(stack.len()-1)));
                        }
                    },
                    Action::Extrapolate(first, diff, parent_id) => {
                        let pre_first = first - diff;
                        if let Some(parent_id) = parent_id {
                            if let Some(Action::Extrapolate(_, ref mut parent_diff, _)) = stack.get_mut(parent_id) {
                                *parent_diff = pre_first;
                            }
                        } else {
                            total += pre_first;
                        }
                    }
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_09.txt").await.unwrap();
        let result = Solution::extrapolate_sequences(lines);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_09.txt").await.unwrap();
        let result = Solution::extrapolate_sequences_backward(lines);
        println!("{result:?}");
    }
}