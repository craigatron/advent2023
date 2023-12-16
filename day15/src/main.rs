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

fn hash(s: &str, include_special: bool) -> u8 {
    let mut val = 0;
    for c in s.chars() {
        if !include_special && (c == '-' || c == '=') {
            break;
        }
        let mut tmp = val as u32;
        tmp += c as u32;
        tmp *= 17;
        val = (tmp % 256) as u8;
    }
    val
}

fn part1(file: &str) -> u32 {
    let l = read_lines(file).unwrap().next().unwrap().unwrap();
    let mut sum: u32 = 0;
    for step in l.split(',') {
        sum += hash(step, true) as u32;
    }
    sum
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: Option<u8>,
}

fn part2(file: &str) -> u32 {
    let l = read_lines(file).unwrap().next().unwrap().unwrap();
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    for step in l.split(',') {
        let chars: Vec<char> = step.chars().collect();
        let instruction = if step.ends_with('-') { '-' } else { '=' };
        let focal_length: Option<u8>;
        let label: String;
        if instruction == '-' {
            focal_length = None;
            label = chars[..chars.len() - 1].iter().collect();
        } else {
            focal_length = Some((chars[chars.len() - 1] as u32 - '0' as u32) as u8);
            label = chars[..chars.len() - 2].iter().collect();
        }
        let lens = Lens {
            label: label,
            focal_length: focal_length,
        };
        let box_num = hash(step, false);
        if instruction == '-' {
            if let Some(pos) = boxes[box_num as usize]
                .iter()
                .position(|v| v.label == lens.label)
            {
                boxes[box_num as usize].remove(pos);
            }
        } else {
            if let Some(pos) = boxes[box_num as usize]
                .iter()
                .position(|v| v.label == lens.label)
            {
                boxes[box_num as usize][pos] = lens;
            } else {
                boxes[box_num as usize].push(lens);
            }
        }
    }
    let mut sum: u32 = 0;
    for (box_num, b) in boxes.iter().enumerate() {
        for (i, l) in b.iter().enumerate() {
            sum += ((1 + box_num) * (1 + i) * l.focal_length.unwrap() as usize) as u32;
        }
    }
    sum
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
