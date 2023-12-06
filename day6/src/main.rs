use std::env;
use std::fs::read_to_string;
use std::iter::zip;
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

// this is basically just a quadratic inequality:
// x * (T - x) > D
// which works out to
// -x^2 + Tx - D > 0
// so a = -1, b = T, c = -D
// then add one to the distance since we want to go one unit further
fn quadratic_roots(race: &Race) -> (f64, f64) {
    let b24ac_root = f64::sqrt((race.time.pow(2) - (4 * (race.distance + 1))) as f64);
    let pos = ((-1.0 * race.time as f64) + b24ac_root) / -2.0;
    let neg = ((-1.0 * race.time as f64) - b24ac_root) / -2.0;
    (pos, neg)
}

fn part1(file: &str) -> u64 {
    let races = read_file(file, false);
    let mut mul: u64 = 1;
    for race in races {
        let (pos, neg) = quadratic_roots(&race);

        let options = (neg.floor() - pos.ceil() + 1.0) as u64;
        mul *= options;
    }
    mul
}

fn part2(file: &str) -> u64 {
    let races = read_file(file, true);
    let (pos, neg) = quadratic_roots(&races[0]);
    (neg.floor() - pos.ceil() + 1.0) as u64
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn read_file<P>(filename: P, smoosh: bool) -> Vec<Race>
where
    P: AsRef<Path>,
{
    let file = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file.lines().collect();

    if smoosh {
        let time_parts: Vec<&str> = lines[0].split_whitespace().skip(1).collect();
        let time = time_parts.join("").parse::<u64>().unwrap();
        let distance_parts: Vec<&str> = lines[1].split_whitespace().skip(1).collect();
        let distance = distance_parts.join("").parse::<u64>().unwrap();
        vec![Race {
            time: time,
            distance: distance,
        }]
    } else {
        let times = lines[0]
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap());
        let distances = lines[1]
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap());

        zip(times, distances)
            .map(|(t, d)| Race {
                time: t,
                distance: d,
            })
            .collect()
    }
}
