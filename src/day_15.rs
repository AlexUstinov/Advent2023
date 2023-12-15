use std::collections::HashMap;

fn get_hashcode(val: &str) -> i32 {
    let mut hash = 0;
    for c in val.bytes() {
        hash = ((hash + c as i32) * 17) % 256;
    }
    hash
}

struct Lens<'a> {
    prev_label: Option<&'a str>,
    next_label: Option<&'a str>,
    val: i32
}

impl<'a> Lens<'a> {
    fn new(prev: Option<&'a str>, next: Option<&'a str>, val: i32) -> Self {
        Self {
            prev_label: prev,
            next_label: next,
            val
        }
    }
}
struct LensBox<'a> {
    head: Option<&'a str>,
    tail: Option<&'a str>,
    lenses: HashMap<&'a str, Lens<'a>>
}
impl<'a> LensBox<'a> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            lenses: HashMap::new()
        }
    }

    fn add_lens(&mut self, label: &'a str, val: i32) {
        if let Some(lens) = self.lenses.get_mut(label) {
            lens.val = val;
        } else {
            let new_lens = Lens::new(None, self.head, val);
            if let Some(old_head) = self.head.map(|head_label| self.lenses.get_mut(head_label)).flatten() {
                old_head.prev_label = Some(label);
            }
            self.lenses.insert(label, new_lens);
            if self.lenses.len()==1 {
                self.tail = Some(label);
            }
            self.head = Some(label);
        }
    }

    fn remove_lens(&mut self, label: &'a str) {
        if let Some(old_lens) = self.lenses.remove(label) {
            if self.head.map_or(false, |head_label| head_label==label) {
                self.head = old_lens.next_label;
            }
            if self.tail.map_or(false, |tail_label| tail_label==label) {
                self.tail = old_lens.prev_label;
            }
            if let Some(prev_label) = old_lens.prev_label {
                if let Some(prev_lens) = self.lenses.get_mut(prev_label) {
                    prev_lens.next_label = old_lens.next_label;
                }
            }
            if let Some(next_label) = old_lens.next_label {
                if let Some(next_lens) = self.lenses.get_mut(next_label) {
                    next_lens.prev_label = old_lens.prev_label;
                }
            }
        }
    }

    fn get_lenses(&self) -> Vec<i32> {
        let mut current = self.tail;
        let mut lenses = Vec::new();
        while let Some(lens_label) = current {
            if let Some(lens) = self.lenses.get(lens_label) {
                lenses.push(lens.val);
                current = lens.prev_label;
            }
        }
        lenses
    }
}

pub struct Solution;

impl Solution {
    pub fn get_aggregated_hash(line: &String) -> i32 {
        let mut result = 0;
        for part in line.split(",") {
            result += get_hashcode(part);
        }
        result
    }

    pub fn init_facility(line: &String) -> i32 {
        let mut boxes = Vec::with_capacity(256);
        for _ in 0..256 {
            boxes.push(LensBox::new());
        }
        for part in line.split(",") {
            if part.ends_with('-') {
                let label = part.trim_end_matches('-');
                let box_id = get_hashcode(label) as usize;
                boxes[box_id].remove_lens(label);
            } else if let Some((label, num)) = part.split_once('=') {
                let box_id = get_hashcode(label) as usize;
                boxes[box_id].add_lens(label, num.parse().unwrap())
            }
        }
        let mut result = 0;
        for (lens_box, box_id) in boxes.iter().zip(1..=256) {
            let lenses = lens_box.get_lenses();
            let lens_count = lenses.len() as i32;
            for (lens, slot_id) in lenses.into_iter().zip(1..=lens_count) {
                result += box_id*slot_id*lens;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_15.txt").await.unwrap();
        let result = Solution::get_aggregated_hash(&lines[0]);
        println!("{}", result);
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_15.txt").await.unwrap();
        let result = Solution::init_facility(&lines[0]);
        println!("{}", result);
    }
}