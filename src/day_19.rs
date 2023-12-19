pub struct Solution;

enum Param { X, M, A, S }
enum Op { Greater, Less, True }
struct Part { x:i32, m:i32, a:i32, s:i32 }

struct Rule { param: Option<Param>, op: Op, val: i32, target: String }
struct Flow {
    name: String,
    rules: Vec<Rule>
}

fn parse_parts(parts_lines: Vec<String>) -> Vec<Part> {
    let mut parts = Vec::with_capacity(parts_lines.len());
    for ln in parts_lines {
        let vals = ln.split(',').map(|val| val.split_once('=').unwrap().1.trim_end_matches('}').parse::<i32>().unwrap()).collect::<Vec<_>>();
        parts.push(Part{x:vals[0], m:vals[1], a:vals[2], s:vals[3]});
    }
    parts
}

fn parse_flows(flow_lines: Vec<String>) -> Vec<Flow> {
    let mut flows = Vec::with_capacity(flow_lines.len());
    for ln in flow_lines {
        let (name, rule_defs) = ln.trim_end_matches('}').split_once('{').unwrap();
        let mut rules = vec![];
        for rule_def in rule_defs.split(',') {
            let rule = if let Some((condition, target)) = rule_def.split_once(':') {
                let param = Some(match &condition[0..1] { "x" => Param::X, "m" => Param::M, "a" => Param::A, "s" => Param::S, _ => unreachable!() });
                let op = match &condition[1..2] { "<" => Op::Less, ">" => Op::Greater, _ => unreachable!() };
                let val = condition[2..].parse::<i32>().unwrap();
                Rule { param, op, val, target: String::from(target) }
            } else {
                Rule { param:None, op:Op::True, val:0, target:String::from(rule_def) }
            };
            rules.push(rule)
        }

        flows.push(Flow {name:String::from(name), rules});
    }
    flows
}

fn get_param_val(part: &Part, param: &Param) -> i32 {
    match param {
        Param::X => part.x,
        Param::M => part.m,
        Param::A => part.a,
        Param::S => part.s,
    }
}


impl Solution {
    pub fn solve1(lines: Vec<String>) -> i64 {
        use std::collections::{VecDeque, HashMap};

        let (part_lines, flow_lines) = lines.into_iter().fold((vec![], vec![]), |(mut first, second), ln| {
            if ln.is_empty() {
                (second, first)
            } else {
                first.push(ln);
                (first, second)
            }
        });
        let mut queue = VecDeque::with_capacity(part_lines.len());
        let parts = parse_parts(part_lines);
        for part in parts.iter() {
            queue.push_back((String::from("in"), part))
        }
        let mut flows = HashMap::with_capacity(flow_lines.len());
        for flow in parse_flows(flow_lines) {
            flows.insert(flow.name.clone(), flow);
        }
        let mut rating = 0;

        while let Some((flow_name, part)) = queue.pop_front() {
            let flow = &flows[&flow_name];
            for Rule { param, op, val, target} in flow.rules.iter() {
                match op {
                    Op::Greater => {
                        if *val >= get_param_val(part, param.as_ref().unwrap()) {
                            continue;
                        }
                    },
                    Op::Less => {
                        if *val <= get_param_val(part, param.as_ref().unwrap()) {
                            continue;
                        }
                    },
                    Op::True => {},
                };
                match target.as_str() {
                    "A" => {
                        rating += (part.x + part.m + part.a + part.s) as i64;
                    },
                    "R" => {},
                    _ => {
                        queue.push_back((target.clone(), part))
                    }
                }
                break;
            }
        }

        rating
    }

    
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_19.txt").await.unwrap();
        let result = Solution::solve1(lines);
        println!("{result:?}");
    }
}