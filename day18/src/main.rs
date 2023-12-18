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

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn parse(file: &str) -> Vec<(Direction, u8, String)> {
    let mut v = Vec::new();
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let d = match parts[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!("unexpected direction"),
        };
        v.push((
            d,
            parts[1].parse::<u8>().unwrap(),
            parts[2][2..parts[2].len() - 1].to_owned(),
        ));
    }
    v
}

fn area(instructions: Vec<(Direction, u32)>) -> u64 {
    let mut vertex: (i32, i32) = (0, 0);
    let mut s1: i64 = 0;
    let mut s2: i64 = 0;
    let mut border: u64 = 0;
    for i in 0..instructions.len() {
        let (dir, steps) = &instructions[i];
        let isteps = *steps as i32;
        border += *steps as u64;
        let new_point = match dir {
            Direction::Down => (vertex.0 + isteps, vertex.1),
            Direction::Up => (vertex.0 - isteps, vertex.1),
            Direction::Right => (vertex.0, vertex.1 + isteps),
            Direction::Left => (vertex.0, vertex.1 - isteps),
        };
        s1 += vertex.0 as i64 * new_point.1 as i64;
        s2 += vertex.1 as i64 * new_point.0 as i64;
        vertex = new_point;
    }
    // had to cheat and look up the border calculation and now I feel dumb
    ((s1 - s2).abs() as u64 / 2) + (border / 2) + 1
}

fn parse_hex(hex: &String) -> (Direction, u32) {
    let num = u32::from_str_radix(&hex[0..5], 16).unwrap();
    let dir = match &hex[5..] {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => panic!("unexpected char"),
    };
    (dir, num)
}

fn part1(file: &str) -> u64 {
    let v = parse(file);

    area(v.iter().map(|i| (i.0, i.1 as u32)).collect())
}

fn part2(file: &str) -> u64 {
    let v = parse(file);

    area(v.iter().map(|(_, _, hex)| parse_hex(hex)).collect())
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
