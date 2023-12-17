pub struct Solution;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Side {N, E, S, W}

impl Solution {
    pub fn calculate_min_loss(lines: Vec<Vec<u8>>) -> i32 {
        use std::collections::{HashSet, BinaryHeap};
        use std::cmp::Reverse;

        let (n,m) = (lines.len(), lines[0].len());
        let get_moves = |i:usize, j:usize, side, d:i32| {
            match side {
                Side::N => [
                    (j>0).then(|| (i, j-1, Side::E, 1)),
                    (j+1<m).then(|| (i, j+1, Side::W, 1)),
                    (d<3 && i+1<n).then(|| (i+1, j, Side::N, d+1))
                ],
                Side::E => [
                    (i>0).then(|| (i-1, j, Side::S, 1)),
                    (i+1<n).then(|| (i+1, j, Side::N, 1)),
                    (d<3 && j>0).then(|| (i, j-1, Side::E, d+1))
                ],
                Side::S => [
                    (j+1<m).then(|| (i, j+1, Side::W, 1)),
                    (j>0).then(|| (i, j-1, Side::E, 1)),
                    (d<3 && i>0).then(|| (i-1, j, Side::S, d+1))
                ],
                Side::W => [
                    (i+1<n).then(|| (i+1, j, Side::N, 1)),
                    (i>0).then(|| (i-1, j, Side::S, 1)),
                    (d<3 && j+1<m).then(|| (i, j+1, Side::W, d+1))
                ],
            }.into_iter().filter_map(|mv| mv).collect::<Vec<_>>()
        };
        let mut visited = HashSet::new();
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0),0,0,Side::W,0));
        while let Some((Reverse(cost),i,j,side,d)) = pq.pop() {
            if i==n-1 && j==m-1 {
                return cost;
            }
            for (ii,jj,sside,dd) in get_moves(i,j,side,d) {
                if visited.insert((ii,jj,sside,dd)) {
                    let val = lines[ii][jj] as i32;
                    pq.push((Reverse(cost+val),ii,jj,sside,dd))
                }    
            }
        }
        
        unreachable!()
    }   

    pub fn calculate_super_min_loss(lines: Vec<Vec<u8>>) -> i32 {
        use std::collections::{HashSet, BinaryHeap};
        use std::cmp::Reverse;

        let (n,m) = (lines.len(), lines[0].len());
        let get_moves = |i:usize, j:usize, side, d:i32| {
            match side {
                Side::N => [
                    (j>3 && d>=4).then(|| (i, j-1, Side::E, 1)),
                    (j+4<m && d>=4).then(|| (i, j+1, Side::W, 1)),
                    (d<10 && i+1<n).then(|| (i+1, j, Side::N, d+1))
                ],
                Side::E => [
                    (i>3 && d>=4).then(|| (i-1, j, Side::S, 1)),
                    (i+4<n && d>=4).then(|| (i+1, j, Side::N, 1)),
                    (d<10 && j>0).then(|| (i, j-1, Side::E, d+1))
                ],
                Side::S => [
                    (j+4<m && d>=4).then(|| (i, j+1, Side::W, 1)),
                    (j>3 && d>=4).then(|| (i, j-1, Side::E, 1)),
                    (d<10 && i>0).then(|| (i-1, j, Side::S, d+1))
                ],
                Side::W => [
                    (i+4<n && (d>=4 || d==0)).then(|| (i+1, j, Side::N, 1)),
                    (i>3 && d>=4).then(|| (i-1, j, Side::S, 1)),
                    (d<10 && j+1<m).then(|| (i, j+1, Side::W, d+1))
                ],
            }.into_iter().filter_map(|mv| mv).collect::<Vec<_>>()
        };
        let mut visited = HashSet::new();
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0),0,0,Side::W,0));
        while let Some((Reverse(cost),i,j,side,d)) = pq.pop() {
            if i==n-1 && j==m-1 && d>=4 {
                return cost;
            }
            for (ii,jj,sside,dd) in get_moves(i,j,side,d) {
                if visited.insert((ii,jj,sside,dd)) {
                    let val = lines[ii][jj] as i32;
                    pq.push((Reverse(cost+val),ii,jj,sside,dd))
                }    
            }
        }
        
        unreachable!()
    }   
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_17.txt").await.unwrap();
        let result = Solution::calculate_min_loss(lines.iter().map(|ln| ln.bytes().map(|c| c-b'0').collect::<Vec<_>>()).collect::<Vec<_>>());
        println!("{result:?}")
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_17.txt").await.unwrap();
        let result = Solution::calculate_super_min_loss(lines.iter().map(|ln| ln.bytes().map(|c| c-b'0').collect::<Vec<_>>()).collect::<Vec<_>>());
        println!("{result:?}")
    }
}