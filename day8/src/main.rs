use std::collections::HashMap;
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

fn parse(file: &str) -> (HashMap<String, (String, String)>, Vec<char>) {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut directions: Vec<char> = Vec::new();
    for (i, l) in read_lines(file).unwrap().enumerate() {
        let line = l.unwrap();
        if i == 0 {
            directions = line.chars().collect();
        } else if !line.is_empty() {
            map.insert(
                line[0..3].to_string(),
                (line[7..10].to_string(), line[12..15].to_string()),
            );
        }
    }
    (map, directions)
}

fn part1(file: &str) -> u32 {
    let (map, directions) = parse(file);

    let mut node: &String = &"AAA".to_string();
    let mut steps: u32 = 0;
    let mut direction_index: usize = 0;
    while node != "ZZZ" {
        let dir = directions[direction_index];
        let map_node = map.get(node).unwrap();
        let next_node = if dir == 'L' { &map_node.0 } else { &map_node.1 };
        steps += 1;
        direction_index = if direction_index == directions.len() - 1 {
            0
        } else {
            direction_index + 1
        };
        node = &next_node;
    }

    steps
}

fn part2(file: &str) -> u64 {
    let (map, directions) = parse(file);
    let initial_nodes: Vec<&String> = map.keys().filter(|s| s.ends_with("A")).collect();
    let mut first_z: Vec<Option<u32>> = vec![None; initial_nodes.len()];

    let mut current_nodes = initial_nodes.clone();
    let mut steps: u32 = 0;
    let mut direction_index: usize = 0;
    while first_z.iter().any(|z| z.is_none()) {
        let dir = directions[direction_index];
        for i in 0..current_nodes.len() {
            let current_node = map.get(current_nodes[i]).unwrap();
            let next_node = if dir == 'L' {
                &current_node.0
            } else {
                &current_node.1
            };
            if first_z[i].is_none() && next_node.ends_with("Z") {
                first_z[i] = Some(steps + 1);
            }
            current_nodes[i] = next_node;
        }
        steps += 1;
        direction_index = if direction_index == directions.len() - 1 {
            0
        } else {
            direction_index + 1
        };
    }

    let unwrapped: Vec<u64> = first_z.iter().map(|z| z.unwrap() as u64).collect();
    lcm(&unwrapped)
}

// too annoyed by this problem to write my own lcm so I stole one from
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
