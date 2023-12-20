use std::collections::{HashMap, VecDeque};

pub struct Solution;

#[derive(PartialEq, Clone, Copy)]
enum FlipFlopState { On, Off }

#[derive(PartialEq, Clone, Copy)]
enum PulseType { High, Low }

#[derive(PartialEq, Clone)]
struct ConjunctionState {
    prev_signals: HashMap<String, PulseType>
}

#[derive(PartialEq, Clone)]
struct DeviceData {
    name: String,
    outputs: Vec<String>
}

#[derive(PartialEq, Clone)]
enum Device { Broadcaster(DeviceData), FlipFlop(DeviceData, FlipFlopState), Conjunction(DeviceData, ConjunctionState) }

fn parse_lines(lines: Vec<String>) -> Vec<Device> {
    fn build_device(line: String, inputs: &mut HashMap<String, Vec<String>>) -> Device {
        let (type_name, outputs) = line.split_once("->")
            .map(|(type_name,outputs)| (type_name.trim(), outputs.split(",").map(|t| String::from(t.trim())).collect::<Vec<_>>()))
            .unwrap();
        if type_name == "broadcaster" {
            Device::Broadcaster(DeviceData { name: String::from(type_name), outputs })
        } else {
            let (dev_type, name) = (&type_name[0..1], String::from(&type_name[1..]));
            for output in outputs.clone() {
                inputs.entry(output).or_insert_with(Vec::new).push(name.clone());
            }
            match dev_type {
                "%" => Device::FlipFlop(DeviceData { name, outputs }, FlipFlopState::Off),
                "&" => Device::Conjunction(DeviceData { name, outputs }, ConjunctionState { prev_signals: HashMap::new() }),
                _ => unreachable!()
            }
        }
    }
    
    let mut inputs = HashMap::new();
    let mut devices = Vec::with_capacity(lines.len());
    for line in lines {
        devices.push(build_device(line, &mut inputs));
    }

    for device in devices.iter_mut() {
        if let Device::Conjunction(DeviceData { name, ..}, state) = device {
            for input in inputs.get(name).into_iter().flatten() {
                state.prev_signals.insert(input.clone(), PulseType::Low);
            }
        }
    }

    devices
}

fn ping(devices: &mut HashMap<String, &mut Device>) -> (i64, i64) {
    let mut queue = VecDeque::new();
    let broadcaster = String::from("broadcaster");
    queue.push_back((String::from("button"), broadcaster, PulseType::Low));
    let (mut low_count, mut high_count) = (0, 0);
    while let Some((from, to, pulse)) = queue.pop_front() {
        if pulse==PulseType::Low {
            low_count += 1;
        } else {
            high_count += 1;
        }
        if let Some(device) = devices.get_mut(&to) {
            match device {
                Device::Broadcaster(DeviceData {ref name, ref outputs }) => {
                    for output in outputs {
                        queue.push_back((name.clone(), output.clone(), pulse));
                    }
                },
                Device::FlipFlop(DeviceData {ref name, ref outputs, ..}, ref mut state) if pulse==PulseType::Low => {
                    let next_pulse = if *state==FlipFlopState::Off {
                        *state=FlipFlopState::On;
                        PulseType::High
                    } else {
                        *state=FlipFlopState::Off;
                        PulseType::Low
                    };
                    for output in outputs {
                        queue.push_back((name.clone(), output.clone(), next_pulse));
                    }
                },
                Device::Conjunction(DeviceData {ref name, ref outputs, ..}, ConjunctionState { ref mut prev_signals }) => {
                    if let Some(saved_pulse) = prev_signals.get_mut(&from) {
                        *saved_pulse = pulse;
                    }
                    let next_pulse = if prev_signals.values().any(|&saved_pulse| saved_pulse==PulseType::Low) {
                        PulseType::High
                    } else {
                        PulseType::Low
                    };
                    for output in outputs {
                        queue.push_back((name.clone(), output.clone(), next_pulse));
                    }
                },
                _ => continue,
            }
        }
    }

    (low_count, high_count)
}

fn convert_to_map<'a>(devices: &'a mut Vec<Device>) -> HashMap<String, &'a mut Device> {
    let mut device_map = HashMap::new();
    for device in devices.iter_mut() {
        let name = match device {
            Device::Broadcaster(DeviceData { ref name, ..}) => name,
            Device::FlipFlop(DeviceData { ref name, ..}, _) => name,
            Device::Conjunction(DeviceData { ref name, ..}, _) => name,
        };
        device_map.insert(name.clone(), device);
    }
    device_map
}


impl Solution {
    pub fn get_pulse_count(lines: Vec<String>, requested_iterations: i64) -> i64 {
        let mut devices = parse_lines(lines);
        let mut devices2 = devices.clone();
        let mut device_map = convert_to_map(&mut devices);
        let mut device_map2 = convert_to_map(&mut devices2);

        let (mut low_count, mut high_count) = (0, 0);
        for iteration in 1..=requested_iterations {
            let (low, high) = ping(&mut device_map);
            low_count += low;
            high_count += high;
            _ = ping(&mut device_map2);
            _ = ping(&mut device_map2);
            if device_map==device_map2 {
                let mut cycle_len = 1;
                let (mut cycle_low_count, mut cycle_high_count) = (0, 0);
                while {
                    let (low, high) = ping(&mut device_map2);
                    cycle_low_count += low;
                    cycle_high_count += high;
                    device_map!=device_map2
                } {
                    cycle_len += 1;
                }
                let remained_iterations = requested_iterations - iteration;
                low_count += cycle_low_count*(remained_iterations / cycle_len);
                high_count += cycle_high_count*(remained_iterations / cycle_len);
                let remained_iterations = remained_iterations % cycle_len;
                for _ in 0..remained_iterations {
                    let (low, high) = ping(&mut device_map);
                    low_count += low;
                    high_count += high;
                }
                break;
            }
        }

        low_count * high_count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::load_lines;
    use tokio;

    #[tokio::test]
    async fn solve1() {
        let lines = load_lines("day_20.txt").await.unwrap();
        let result = Solution::get_pulse_count(lines, 1000000);
        println!("{result:?}");
    }
}