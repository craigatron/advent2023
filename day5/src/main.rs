use std::cmp;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
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

#[derive(Clone, Debug)]
struct MappingRange {
    source_range: Range<u64>,
    source_range_start: u64,
    destination_range_start: u64,
}

impl MappingRange {
    fn map(&self, val: u64) -> Option<u64> {
        if self.source_range.contains(&val) {
            Some(self.destination_range_start + val - self.source_range_start)
        } else {
            None
        }
    }

    fn map_range(&self, range: &Range<u64>) -> (Vec<Range<u64>>, Option<Range<u64>>) {
        let mut unmapped: Vec<Range<u64>> = Vec::new();
        let mut mapped: Option<Range<u64>> = None;
        if range.end < self.source_range.start || range.start >= self.source_range.end {
            unmapped.push(range.clone());
        } else {
            if range.start < self.source_range.start {
                unmapped.push(Range {
                    start: range.start,
                    end: self.source_range.start,
                });
            }
            let start_src = cmp::max(range.start, self.source_range.start);
            let end_src = cmp::min(range.end, self.source_range.end);
            mapped = Some(Range {
                start: self.destination_range_start + start_src - self.source_range_start,
                end: self.destination_range_start + end_src - self.source_range_start,
            });
            if range.end > self.source_range.end {
                unmapped.push(Range {
                    start: self.source_range.end,
                    end: range.end,
                });
            }
        }
        (unmapped, mapped)
    }
}

#[derive(Clone, Debug)]
struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn map(&self, val: u64) -> Option<u64> {
        let mut mapping: Option<u64> = None;
        for r in &self.ranges {
            let m = r.map(val);
            if m.is_some() {
                mapping = m;
                break;
            }
        }
        mapping
    }

    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let mut v: Vec<Range<u64>> = Vec::new();
        let mut unmapped: Vec<Range<u64>> = Vec::new();
        unmapped.push(range.clone());
        for r in &self.ranges {
            let mut new_unmapped: Vec<Range<u64>> = Vec::new();
            for unmapped_r in unmapped {
                let (u, m) = r.map_range(&unmapped_r);
                new_unmapped.extend(u);
                if m.is_some() {
                    v.push(m.unwrap());
                }
            }
            unmapped = new_unmapped;
        }
        v.extend(unmapped.clone());
        v.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
        v
    }
}

fn part1(file: &str) -> u64 {
    let mut seeds: Vec<u64> = Vec::new();
    let mut current_ranges: Vec<MappingRange> = Vec::new();
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        if line == "" {
            if !current_ranges.is_empty() {
                let mapping = Mapping {
                    ranges: current_ranges.clone(),
                };
                // advance the seed values
                for i in 0..seeds.len() {
                    let seed_val = seeds[i];
                    seeds[i] = mapping.map(seed_val).unwrap_or(seed_val);
                }
                current_ranges.clear();
            }
            continue;
        }
        if line.starts_with("seeds:") {
            let seedstr = line[7..].to_string();
            seeds.extend(
                seedstr
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap()),
            );
            continue;
        }

        if line.contains(":") {
            // just taking advantage of all the steps being in order here
            continue;
        }

        let range_parts: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        current_ranges.push(MappingRange {
            source_range: Range {
                start: range_parts[1],
                end: range_parts[1] + range_parts[2],
            },
            destination_range_start: range_parts[0],
            source_range_start: range_parts[1],
        });
    }
    let mapping = Mapping {
        ranges: current_ranges.clone(),
    };
    // advance the seed values
    for i in 0..seeds.len() {
        let seed_val = seeds[i];
        seeds[i] = mapping.map(seed_val).unwrap_or(seed_val);
    }
    *seeds.iter().min().unwrap()
}

fn part2(file: &str) -> u64 {
    let mut seed_ranges: Vec<Range<u64>> = Vec::new();
    let mut current_ranges: Vec<MappingRange> = Vec::new();
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        if line == "" {
            if !current_ranges.is_empty() {
                let mut new_seed_ranges: Vec<Range<u64>> = Vec::new();
                let mapping = Mapping {
                    ranges: current_ranges.clone(),
                };
                // advance the seed values
                for i in 0..seed_ranges.len() {
                    let seed_val = &seed_ranges[i];
                    new_seed_ranges.extend(mapping.map_range(seed_val));
                }
                seed_ranges = new_seed_ranges;
                current_ranges.clear();
            }
            continue;
        }
        if line.starts_with("seeds:") {
            let seedstr = line[7..].to_string();
            let vec: Vec<u64> = seedstr
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            for i in (0..vec.len()).step_by(2) {
                seed_ranges.push(Range {
                    start: vec[i],
                    end: vec[i] + vec[i + 1],
                });
            }
            continue;
        }

        if line.contains(":") {
            // just taking advantage of all the steps being in order here
            continue;
        }

        let range_parts: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        current_ranges.push(MappingRange {
            source_range: Range {
                start: range_parts[1],
                end: range_parts[1] + range_parts[2],
            },
            destination_range_start: range_parts[0],
            source_range_start: range_parts[1],
        });
    }
    let mut new_seed_ranges: Vec<Range<u64>> = Vec::new();
    let mapping = Mapping {
        ranges: current_ranges.clone(),
    };
    // advance the seed values
    for i in 0..seed_ranges.len() {
        let seed_val = &seed_ranges[i];
        new_seed_ranges.extend(mapping.map_range(seed_val));
    }
    seed_ranges = new_seed_ranges;
    let mut min = seed_ranges[0].start;
    for i in 1..seed_ranges.len() {
        if seed_ranges[i].start < min {
            min = seed_ranges[i].start;
        }
    }
    min
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
