pub struct Solution;

#[derive(Copy, Clone)]
enum Param { X, M, A, S }
#[derive(Copy, Clone)]
enum Op { Greater, Less, GreaterOrEq, LessOrEq, True }
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

enum Action<'a> { Explore(&'a str, Vec<(Param, Op, i32)>), Restore }

fn invert_rule(rule: (Param, Op, i32)) -> (Param, Op, i32) {
    let (param, op, val) = rule;
    let op = match op { Op::Greater => Op::LessOrEq, Op::Less => Op::GreaterOrEq, _ => unreachable!() };
    (param, op, val)
}

impl Solution {
    pub fn find_acceptable_parts(lines: Vec<String>) -> i64 {
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
                    _ => {},
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

    pub fn find_number_of_acceptable_param_combinations(lines: Vec<String>) -> i64 {
        use std::collections::{HashMap};

        let mut flow_lines = Vec::new();
        for ln in lines {
            if ln.is_empty() {
                break;
            }
            flow_lines.push(ln);
        }
        let mut flow_graph = HashMap::with_capacity(flow_lines.len());
        let flows = parse_flows(flow_lines);
        for flow in flows.iter() {
            flow_graph.insert(flow.name.clone(), flow);
        }

        let mut accept_rules = Vec::new();
        let mut happy_path_rules = Vec::new();
        let mut stack = Vec::new();
        stack.push(Action::Explore("in", vec![]));

        while let Some(action) = stack.pop() {
            match action {
                Action::Explore(flow_name, rules) => {
                    stack.push(Action::Restore);
                    happy_path_rules.push(rules);
                    if flow_name=="A" {
                        accept_rules.push(happy_path_rules.iter().flatten().copied().collect::<Vec<_>>());
                        continue;
                    }
                    if flow_name=="R" {
                        continue;
                    }
                    let flow = flow_graph[flow_name];
                    let mut rule_set = Vec::new();
                    for rule in flow.rules.iter() {
                        if let Some(prev_rule) = rule_set.pop() {
                            rule_set.push(invert_rule(prev_rule));
                        }
                        if let Some(param) = rule.param {
                            rule_set.push((param, rule.op, rule.val));
                        }
                        stack.push(Action::Explore(&rule.target, rule_set.clone()))
                    }
                },
                Action::Restore => {
                    happy_path_rules.pop();
                }
            }
        }

        fn param_index(param: &Param) -> usize {
            match param { Param::X => 0, Param::M => 1, Param::A => 2, Param::S => 3, }
        }

        let mut total_possibilities = 0;

        for rule_set in accept_rules.iter() {
            let mut param_ranges = [[1,4001], [1,4001], [1,4001], [1,4001]];
            for &(param, op, val) in rule_set {
                let range = &mut param_ranges[param_index(&param)];
                let val = match op {
                    Op::Greater | Op::LessOrEq => val + 1,
                    Op::Less | Op::GreaterOrEq | Op::True => val,
                };
                match op {
                    Op::Greater | Op::GreaterOrEq => {
                        let lower_bound = val;
                        if lower_bound > range[0] {
                            range[0] = range[1].min(lower_bound);
                        }
                    },
                    Op::Less | Op::LessOrEq => {
                        let upper_bound = val;
                        if upper_bound < range[1] {
                            range[1] = range[0].max(upper_bound);
                        }
                    },
                    Op::True => {},
                }
            }
            total_possibilities += param_ranges.into_iter().map(|[start, end]| (end-start) as i64).reduce(|count, size| count*size).unwrap();
        }

        total_possibilities
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
        let result = Solution::find_acceptable_parts(lines);
        println!("{result:?}");
    }

    #[tokio::test]
    async fn solve2() {
        let lines = load_lines("day_19.txt").await.unwrap();
        let result = Solution::find_number_of_acceptable_param_combinations(lines);
        println!("{result:?}");
    }
}