use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let start1 = Instant::now();
    let p1 = part1(file);
    let duration1 = start1.elapsed();
    println!("part1: {}, time {:?}", p1, duration1);

    let start2 = Instant::now();
    let p2 = part2(file);
    let duration2 = start2.elapsed();
    println!("part2: {}, time {:?}", p2, duration2);
}

#[derive(Debug, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    High,
    Low,
}

fn parse(file: &str) -> HashMap<String, (ModuleType, Vec<String>)> {
    let mut res = HashMap::new();

    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(" -> ").collect();
        let module_type: ModuleType;
        let module_name: String;
        if parts[0].starts_with("%") {
            module_type = ModuleType::FlipFlop;
            module_name = parts[0][1..parts[0].len()].to_owned();
        } else if parts[0].starts_with("&") {
            module_type = ModuleType::Conjunction;
            module_name = parts[0][1..parts[0].len()].to_owned();
        } else {
            module_type = ModuleType::Broadcaster;
            module_name = parts[0].to_owned();
        }
        let destinations: Vec<String> = parts[1].split(", ").map(|s| s.to_owned()).collect();

        res.insert(module_name, (module_type, destinations));
    }

    res
}

fn part1(file: &str) -> u32 {
    let modules = parse(file);

    let mut flip_flops: HashMap<String, bool> = modules
        .iter()
        .filter(|&(_, v)| v.0 == ModuleType::FlipFlop)
        .map(|(k, _)| (k.clone(), false))
        .collect();

    let mut conjunction_nodes: HashMap<String, HashMap<String, Pulse>> = HashMap::new();
    for node in modules
        .iter()
        .filter(|&(_, v)| v.0 == ModuleType::Conjunction)
    {
        conjunction_nodes.insert(
            node.0.clone(),
            modules
                .iter()
                .filter(|&(_, v)| v.1.contains(node.0))
                .map(|(k, _)| (k.clone(), Pulse::Low))
                .collect(),
        );
    }

    let mut pulse_counts: Vec<(u32, u32)> = vec![];
    loop {
        let mut low_pulses = 0;
        let mut high_pulses = 0;
        let mut nodes: VecDeque<(String, String, Pulse)> =
            VecDeque::from([("broadcaster".to_string(), "".to_string(), Pulse::Low)]);
        while !nodes.is_empty() {
            let (module_name, from_module, pulse) = nodes.pop_front().unwrap();
            if pulse == Pulse::Low {
                low_pulses += 1;
            } else {
                high_pulses += 1;
            }
            if let Some(module) = modules.get(&module_name) {
                let next_pulse: Option<Pulse> = match module.0 {
                    ModuleType::Broadcaster => Some(Pulse::Low),
                    ModuleType::Conjunction => {
                        let pulses = conjunction_nodes.get_mut(&module_name).unwrap();
                        pulses.insert(from_module, pulse);
                        if pulses.values().all(|p| *p == Pulse::High) {
                            Some(Pulse::Low)
                        } else {
                            Some(Pulse::High)
                        }
                    }
                    ModuleType::FlipFlop => {
                        if pulse == Pulse::Low {
                            let current_value = *flip_flops.get(&module_name).unwrap();
                            flip_flops.insert(module_name.clone(), !current_value);
                            if current_value {
                                Some(Pulse::Low)
                            } else {
                                Some(Pulse::High)
                            }
                        } else {
                            None
                        }
                    }
                };
                if let Some(p) = next_pulse {
                    for n in module.1.iter() {
                        nodes.push_back((n.to_owned(), module_name.clone(), p));
                    }
                }
            }
        }

        pulse_counts.push((low_pulses, high_pulses));
        if pulse_counts.len() == 1000 {
            break;
        }
    }
    let totals = pulse_counts
        .iter()
        .fold((0_u32, 0_u32), |acc, e| (acc.0 + e.0, acc.1 + e.1));

    1000 * totals.0 * 1000 * totals.1
}

fn part2(file: &str) -> u64 {
    let modules = parse(file);

    // this only works because there are very distinct subgraphs, thank you reddit hints and graphviz

    // find the last node that outputs to rx
    let last_node = modules
        .iter()
        .filter(|(_, v)| v.1.contains(&"rx".to_string()))
        .map(|(k, _)| k)
        .next()
        .unwrap();

    let mut subgraphs: Vec<HashSet<String>> = vec![];
    let mut penultimates: Vec<String> = vec![];
    let broadcast_outputs = &modules.get(&"broadcaster".to_string()).unwrap().1;
    for initial_out in broadcast_outputs {
        let mut subgraph_nodes: HashSet<String> = HashSet::new();
        let mut nodes: VecDeque<String> = VecDeque::from([initial_out.to_string()]);
        while !nodes.is_empty() {
            let node = nodes.pop_front().unwrap();
            if subgraph_nodes.contains(&node) || penultimates.contains(&node) {
                continue;
            }
            let module = modules.get(&node).unwrap();
            if module.1.contains(last_node) {
                penultimates.push(node);
            } else {
                subgraph_nodes.insert(node.clone());
                for next in module.1.iter() {
                    if next != last_node {
                        nodes.push_back(next.to_string());
                    }
                }
            }
        }
        subgraphs.push(subgraph_nodes);
    }

    let mut loop_indices = vec![0_u64; subgraphs.len()];

    let mut flip_flops: HashMap<String, bool> = modules
        .iter()
        .filter(|&(_, v)| v.0 == ModuleType::FlipFlop)
        .map(|(k, _)| (k.clone(), false))
        .collect();

    let mut conjunction_nodes: HashMap<String, HashMap<String, Pulse>> = HashMap::new();
    for node in modules
        .iter()
        .filter(|&(_, v)| v.0 == ModuleType::Conjunction)
    {
        conjunction_nodes.insert(
            node.0.clone(),
            modules
                .iter()
                .filter(|&(_, v)| v.1.contains(node.0))
                .map(|(k, _)| (k.clone(), Pulse::Low))
                .collect(),
        );
    }

    let mut iterations = 0;
    loop {
        let mut nodes: VecDeque<(String, String, Pulse)> =
            VecDeque::from([("broadcaster".to_string(), "".to_string(), Pulse::Low)]);
        while !nodes.is_empty() {
            let (module_name, from_module, pulse) = nodes.pop_front().unwrap();
            if let Some(module) = modules.get(&module_name) {
                let next_pulse: Option<Pulse> = match module.0 {
                    ModuleType::Broadcaster => Some(Pulse::Low),
                    ModuleType::Conjunction => {
                        let pulses = conjunction_nodes.get_mut(&module_name).unwrap();
                        pulses.insert(from_module, pulse);
                        if pulses.values().all(|p| *p == Pulse::High) {
                            Some(Pulse::Low)
                        } else {
                            Some(Pulse::High)
                        }
                    }
                    ModuleType::FlipFlop => {
                        if pulse == Pulse::Low {
                            let current_value = *flip_flops.get(&module_name).unwrap();
                            flip_flops.insert(module_name.clone(), !current_value);
                            if current_value {
                                Some(Pulse::Low)
                            } else {
                                Some(Pulse::High)
                            }
                        } else {
                            None
                        }
                    }
                };
                if let Some(p) = next_pulse {
                    for n in module.1.iter() {
                        nodes.push_back((n.to_owned(), module_name.clone(), p));
                    }
                }
            }
        }

        for i in 0..subgraphs.len() {
            if loop_indices[i] != 0 {
                continue;
            }
            let subgraph = &subgraphs[i];
            if subgraph.iter().all(|k| {
                let node_type = &modules.get(k).unwrap().0;
                match node_type {
                    ModuleType::Broadcaster => panic!("invalid node"),
                    ModuleType::Conjunction => conjunction_nodes
                        .get(k)
                        .unwrap()
                        .values()
                        .all(|&p| p == Pulse::Low),
                    ModuleType::FlipFlop => !flip_flops.get(k).unwrap(),
                }
            }) {
                loop_indices[i] = iterations + 1;
                break;
            }
        }

        if loop_indices.iter().all(|&v| v != 0) {
            break;
        }

        iterations += 1;
    }

    lcm(&loop_indices)
}

// once again, too annoyed by this problem to write my own lcm so I stole one from
// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
