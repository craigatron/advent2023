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

type XYZ = (f64, f64, f64);

#[derive(Debug)]
struct Hailstone {
    start: XYZ,
    velocity: XYZ,
}

fn parse(file: &str) -> Vec<Hailstone> {
    read_lines(file)
        .unwrap()
        .map(|l| {
            let line = l.unwrap();
            let parts: Vec<&str> = line.split(" @ ").collect();
            let point_vec: Vec<f64> = parts[0]
                .split(", ")
                .map(|s| s.parse::<f64>().unwrap())
                .collect();
            let velocity_vec: Vec<f64> = parts[1]
                .split(", ")
                .map(|s| s.parse::<f64>().unwrap())
                .collect();
            Hailstone {
                start: (point_vec[0], point_vec[1], point_vec[2]),
                velocity: (velocity_vec[0], velocity_vec[1], velocity_vec[2]),
            }
        })
        .collect()
}

fn part1(file: &str) -> u32 {
    let hailstones = parse(file);

    let min: f64 = 200000000000000.0;
    let max: f64 = 400000000000000.0;

    let mut count = 0;
    for i in 0..hailstones.len() {
        let h1 = &hailstones[i];
        let h1m = h1.velocity.1 / h1.velocity.0;
        for j in i + 1..hailstones.len() {
            let h2 = &hailstones[j];
            let h2m = h2.velocity.1 / h2.velocity.0;
            if h1.start == h2.start {
                count += 1;
                continue;
            } else if h1m == h2m {
                continue;
            }
            let t1 = (h1.start.1 * h2.velocity.0 + h2.start.0 * h2.velocity.1
                - h2.start.1 * h2.velocity.0
                - h1.start.0 * h2.velocity.1)
                / (h1.velocity.0 * h2.velocity.1 - h1.velocity.1 * h2.velocity.0);
            let t2 = (h1.start.0 + h1.velocity.0 * t1 - h2.start.0) / h2.velocity.0;
            if t1 >= 0.0 && t2 >= 0.0 {
                let px = h1.start.0 + (h1.velocity.0 * t1);
                let py = h1.start.1 + (h1.velocity.1 * t1);
                if px >= min && px <= max && py >= min && py <= max {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(file: &str) -> u32 {
    // I spent basically all of my xmas eve waking hours trying to figure this out,
    // failed miserably, and ended up using this approach + WolframAlpha
    // https://www.reddit.com/r/adventofcode/comments/18q40he/2023_day_24_part_2_a_straightforward_nonsolver/

    // frickin hate linear algebra
    0
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
