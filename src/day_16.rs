pub struct Solution;

fn count_energized_tiles(lines: &Vec<&[u8]>, start:(usize,usize,i32)) -> i32 {
    use std::collections::{HashSet, HashMap, VecDeque};

    let (n,m) = (lines.len(), lines[0].len());
    let advance = |i:usize,j:usize, dir:i32| {
        match dir {
            1 => i.checked_sub(1).map(|ii| (ii, j)),
            2 => (j+1<m).then_some((i, j+1)),
            3 => (i+1<n).then_some((i+1, j)),
            4 => j.checked_sub(1).map(|jj| (i, jj)),
            _ => unreachable!()
        }
    };
    let rotate = |el, dir| {
        if el==b'/' {
            match dir {
                1 => 2,
                2 => 1,
                3 => 4,
                4 => 3,
                _ => unreachable!()
            }
        } else { // if el == b'\\'
            match dir {
                1 => 4,
                2 => 3,
                3 => 2,
                4 => 1,
                _ => unreachable!()
            }
        }
    };
    let split = |el, dir| {
        if el==b'-' {
            if dir==2 || dir==4 {
                vec![dir]
            } else {
                vec![2,4]
            }
        } else {
            if dir==1 || dir==3 {
                vec![dir]
            } else {
                vec![1,3]
            }
        }
    };
    // N=1, E=2, S=3, W=4
    let mut energized = HashMap::new();
    let mut visited = HashSet::new();
    let mut beams = VecDeque::new();
    beams.push_back(start);

    while let Some((i,j,dir)) = beams.pop_front() {
        if !visited.insert((i,j,dir)) {
            continue;
        }
        *energized.entry((i,j)).or_insert(0)+=1;
        match lines[i][j] {
            b'.' => {
                if let Some((ii, jj)) = advance(i, j, dir) {
                    beams.push_back((ii, jj, dir));
                }
            },
            el => match el {
                b'/' | b'\\' => {
                    let new_dir = rotate(el, dir);
                    if let Some((ii, jj)) = advance(i, j, new_dir) {
                        beams.push_back((ii, jj, new_dir));
                    }
                },
                b'-' | b'|' => {
                    for new_dir in split(el, dir) {
                        if let Some((ii, jj)) = advance(i, j, new_dir) {
                            beams.push_back((ii, jj, new_dir));
                        }    
                    }
                },
                _ => unreachable!()
            }
        }
    }

    energized.len() as i32
}

impl Solution {
    pub fn count_energized_tiles(lines: Vec<&[u8]>) -> i32 {
        count_energized_tiles(&lines, (0,0,2))
    }

    pub fn count_max_energized_tiles(lines: Vec<&[u8]>) -> i32 {
        let (n, m) = (lines.len(), lines[0].len());
        let mut max_count = 0;
        for col in 0..m {
            max_count = max_count.max(count_energized_tiles(&lines, (0, col, 3)));
            max_count = max_count.max(count_energized_tiles(&lines, (n-1, col, 1)));
        }
        for row in 0..n {
            max_count = max_count.max(count_energized_tiles(&lines, (row, 0, 2)));
            max_count = max_count.max(count_energized_tiles(&lines, (row, m-1, 4)));
        }
        max_count
    }
    
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_16.txt").await.unwrap();
        let result = Solution::count_energized_tiles(lines.iter().map(|ln| ln.as_bytes()).collect::<Vec<_>>());
        println!("{}", result);
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_16.txt").await.unwrap();
        let result = Solution::count_max_energized_tiles(lines.iter().map(|ln| ln.as_bytes()).collect::<Vec<_>>());
        println!("{}", result);
    }
}