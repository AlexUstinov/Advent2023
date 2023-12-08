pub struct Solution;

impl Solution {
    pub fn count_lr(lines: Vec<String>) -> i32 {
        let x: &[_] = &['(', ')', ' '];
        use std::collections::HashMap;
        let mut lr = lines[0].chars().cycle();
        let mut map = HashMap::new();
        for l in lines.iter().skip(2) {
            let (from, (to_l, to_r)) = l.split_once('=').map(|(a, b)| {
                let (to_l, to_r) = b.trim_matches(x).split_once(',').map(|(a,b)| (a.trim(), b.trim())).unwrap();
                (a.trim(), (to_l, to_r))
            }).unwrap();
            map.insert(from, (to_l, to_r));
        }

        let mut count = 0;
        let mut curr = "AAA";
        while curr!="ZZZ" {
            if let Some(&(to_l, to_r)) = map.get(curr) {
                count += 1;
                if 'L'==lr.next().unwrap() {
                    curr = to_l;
                } else {
                    curr = to_r;
                }
            }
        }

        count
    }

    pub fn count_lr_parallel(lines: Vec<String>) -> i64 {
        use std::collections::HashMap;
        use num_integer::lcm;
    
        let x: &[_] = &['(', ')', ' '];
        let lr = &lines[0];
        let mut map = HashMap::new();
        let mut sources = Vec::new();
        for l in lines.iter().skip(2) {
            let (from, (to_l, to_r)) = l.split_once('=').map(|(a, b)| {
                let (to_l, to_r) = b.trim_matches(x).split_once(',').map(|(a,b)| (a.trim(), b.trim())).unwrap();
                (a.trim(), (to_l, to_r))
            }).unwrap();
            if from.chars().last() == Some('A') {
                sources.push(from)
            }
            map.insert(from, (to_l, to_r));
        }

        let mut distances = vec![0; sources.len()];
        for (i, source) in sources.into_iter().enumerate() {
            let mut count = 0;
            let mut curr = source;
            let mut lr_iter = lr.chars().cycle();
            loop {
                let turn = lr_iter.next().unwrap();
                let &(to_l, to_r) = map.get(curr).unwrap();
                let next = if turn=='L' { to_l } else { to_r };
                count += 1;
                if next.chars().last() == Some('Z') {
                    distances[i] = count as i64;
                    break;
                }
                curr = next;
            }
        }

        distances.into_iter().reduce(lcm).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_08.txt").await.unwrap();
        let result = Solution::count_lr(lines);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_08.txt").await.unwrap();
        let result = Solution::count_lr_parallel(lines);
        println!("{result:?}");
    }
}