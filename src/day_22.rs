use std::collections::{HashMap, HashSet};
pub struct Solution;

impl Solution {
    pub fn git_num_bricks_to_safely_desintegrate(lines: Vec<String>) -> i64 {
        let mut bricks = parse_bricks(lines);
        bricks.sort_by_key(|&((_,_,z1), (_,_,z2))| z1.max(z2));
        let brick_map = build_brick_map(&bricks);
        let mut count = 0; 
        for id in 0..bricks.len() {
            let (supports, _) = &brick_map[&id];
            if supports.is_empty() || supports.iter().map(|id| &brick_map[id]).map(|(_, supported_by)| supported_by.len()).all(|cnt| cnt > 1) {
                count += 1;
            }
        }
        count
    }

    pub fn get_num_bricks_to_fall(lines: Vec<String>) -> i64 {
        let mut bricks = parse_bricks(lines);
        bricks.sort_by_key(|&((_,_,z1), (_,_,z2))| z1.max(z2));
        let brick_map = build_brick_map(&bricks);

        let mut total_bricks_to_fall = 0;
        for id in 0..bricks.len() {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(id);
            let mut fallen_bricks = HashSet::new();
            while let Some(id) = queue.pop_front() {
                fallen_bricks.insert(id);
                let (supports, _) = &brick_map[&id];
                for next_id in supports.iter().copied() {
                    let (_, supported_by) = &brick_map[&next_id];
                    let mut supporter_count = supported_by.len();
                    if supporter_count > 0 {
                        for supporter_id in supported_by.iter().copied() {
                            if fallen_bricks.contains(&supporter_id) {
                                supporter_count -= 1;
                            }
                        }
                        if supporter_count == 0 {
                            fallen_bricks.insert(next_id);
                            queue.push_back(next_id);
                        }
                    }
                }
            }
            total_bricks_to_fall += (fallen_bricks.len() - 1) as i64;
        }

        total_bricks_to_fall
    }
}

fn build_brick_map(bricks: &Vec<((i32, i32, i32), (i32, i32, i32))>) -> HashMap<usize, (HashSet<usize>, HashSet<usize>)> {
    let mut top_side = vec![0; bricks.len()];
    let mut brick_map: HashMap<usize, (HashSet<usize>, HashSet<usize>)> = HashMap::new();
    for (id, &((x1, y1, z1), (x2, y2, z2))) in bricks.iter().enumerate() {
        let bottom_side = z1.min(z2);
        let mut candidate_supporters = Vec::new();
        // check candidates
        for (candidate_id, &((xx1, yy1, _), (xx2, yy2, _))) in bricks[0..bricks.partition_point(|&((_,_,z1),(_,_,z2))| z1.max(z2) <= bottom_side)].iter().enumerate() {
            if crossing_bricks(((x1,y1),(x2,y2)), ((xx1,yy1),(xx2,yy2))) {
                candidate_supporters.push(candidate_id);
            }
        }
        let support_z = candidate_supporters.iter().map(|&id|top_side[id]).max().unwrap_or(0);
        let brick_height = z1.max(z2) - z1.min(z2) + 1;
        top_side[id] = support_z + brick_height;
        brick_map.insert(id, (HashSet::new(), HashSet::new()));
        for supporter_id in candidate_supporters.iter().filter(|&&id| top_side[id] == support_z) {
            if let Some((supports, _)) = brick_map.get_mut(supporter_id) {
                supports.insert(id);
            }
            let (_, supported_by) = brick_map.get_mut(&id).unwrap();
            supported_by.insert(*supporter_id);
        }
    }
    brick_map
}

fn crossing_bricks(((x1,y1),(x2,y2)): ((i32, i32), (i32, i32)), ((xx1,yy1),(xx2,yy2)): ((i32, i32), (i32, i32))) -> bool {
    x1.min(x2) <= xx1.max(xx2) && x1.max(x2) >= xx1.min(xx2) && y1.min(y2) <= yy1.max(yy2) && y1.max(y2) >= yy1.min(yy2)
}

fn parse_bricks(lines: Vec<String>) -> Vec<((i32,i32,i32),(i32,i32,i32))> {
    let mut bricks = Vec::with_capacity(lines.len());
    for ln in lines {
        let (start, end) = ln.split_once("~").map(|(start, end)|
            (start.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<_>>(), end.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<_>>())).unwrap();
        bricks.push(((start[0],start[1],start[2]), (end[0], end[1], end[2])));
    }

    bricks
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_22.txt").await.unwrap();
        let result = Solution::git_num_bricks_to_safely_desintegrate(lines);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_22.txt").await.unwrap();
        let result = Solution::get_num_bricks_to_fall(lines);
        println!("{result:?}");
    }
}