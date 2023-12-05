pub struct Solution;

impl Solution {
    pub fn find_engine_schematic(lines: Vec<String>) -> i32 {
        let (n, m) = (lines.len(), lines[0].len());
        let mut schematic_cells = vec![vec![false; m]; n];
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.bytes().enumerate() {
                if c!=b'.' && (c<b'0' || c>b'9') {
                    for ii in i.saturating_sub(1)..=i+1 {
                        for jj in j.saturating_sub(1)..=j+1 {
                            if let Some(sc) = schematic_cells.get_mut(ii).map(|row| row.get_mut(jj)).flatten() {
                                *sc = true;
                            }
                        }
                    }
                }
            }
        }

        let mut schematic_sum = 0;
        for (i, line) in lines.iter().enumerate() {
            let mut num = None;
            let mut is_schematic = false;
            for (j, c) in line.bytes().enumerate() {
                if c>=b'0' && c<=b'9' {
                    let d = (c - b'0') as i32;
                    num = (num.or(Some(0))).map(|val| val*10 + d);
                    is_schematic |= schematic_cells[i][j];
                } else {
                    if let Some(val) = num {
                        if is_schematic {
                            schematic_sum += val;
                        }
                    }
                    is_schematic = false;
                    num = None;
                }
            }
            if let Some(val) = num {
                if is_schematic {
                    schematic_sum += val;
                }
            }
        }
        schematic_sum
    }

    pub fn find_gear_ratio(lines: Vec<String>) -> i32 {
        use std::collections::{HashMap, HashSet};

        let mut gear_id = 0;
        let mut cell_gears = HashMap::new();
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.bytes().enumerate() {
                if c==b'*' {
                    gear_id += 1;
                    for ii in i.saturating_sub(1)..=i+1 {
                        for jj in j.saturating_sub(1)..=j+1 {
                            cell_gears.entry((ii, jj)).or_insert_with(Vec::new).push(gear_id);
                        }
                    }
                }
            }
        }
        let mut gear_parts = HashMap::new();
        let mut num_gears:HashSet<i32> = HashSet::new();

        for (i, line) in lines.iter().enumerate() {
            let mut num = None;
            num_gears.clear();
            for (j, c) in line.bytes().enumerate() {
                if c>=b'0' && c<=b'9' {
                    let d = (c - b'0') as i32;
                    num = (num.or(Some(0))).map(|val| val*10 + d);
                    if let Some(gears) = cell_gears.get(&(i, j)) {
                        num_gears.extend(gears);
                    }
                } else {
                    if let Some(val) = num {
                        for &gear in num_gears.iter() {
                            gear_parts.entry(gear).or_insert_with(Vec::new).push(val);
                        }
                    }
                    num = None;
                    num_gears.clear();
                }
            }
            if let Some(val) = num {
                for &gear in num_gears.iter() {
                    gear_parts.entry(gear).or_insert_with(Vec::new).push(val);
                }
            }
        }

        let mut aggregated_gear_ratio = 0;
        for parts in gear_parts.values().filter(|parts| parts.len()==2) {
            aggregated_gear_ratio += parts[0]*parts[1];
        }
        aggregated_gear_ratio
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_03.txt").await.unwrap();
        let result = Solution::find_engine_schematic(lines);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_03.txt").await.unwrap();
        let result = Solution::find_gear_ratio(lines);
        println!("{result:?}");
    }
}