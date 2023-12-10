use std::collections::HashSet;
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

// return the grid, starting coord, and set of all points in the main loop
fn parse_grid(file: &str) -> (Vec<Vec<char>>, bool, HashSet<(usize, usize)>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (usize::MAX, usize::MAX);
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        if let Some(start_col) = line.find('S') {
            start = (grid.len(), start_col);
        }
        grid.push(line.chars().collect());
    }

    let candidate_next: Vec<(i32, i32, Vec<char>)> = vec![
        // north
        (-1, 0, vec!['|', '7', 'F']),
        // west
        (0, -1, vec!['-', 'L', 'F']),
        // east
        (0, 1, vec!['-', 'J', '7']),
        // south
        (1, 0, vec!['|', 'L', 'J']),
    ];

    let mut next_i: Vec<(usize, usize)> = Vec::new();
    for candidate in candidate_next {
        let candidate_row = candidate.0 + start.0 as i32;
        let candidate_col = candidate.1 + start.1 as i32;
        if candidate_row >= 0
            && (candidate_row as usize) < grid.len()
            && candidate_col >= 0
            && (candidate_col as usize) < grid[0].len()
            && candidate
                .2
                .contains(&grid[candidate_row as usize][candidate_col as usize])
        {
            next_i.push((candidate_row as usize, candidate_col as usize));
        }
    }

    let mut loop_points: HashSet<(usize, usize)> = HashSet::new();
    loop_points.insert(start);
    loop_points.insert(next_i[0]);
    loop_points.insert(next_i[1]);

    let start_is_corner = next_i == vec![(start.0 - 1, start.1), (start.0, start.1 - 1)]
        || next_i == vec![(start.0, start.1 + 1), (start.0 + 1, start.1)];

    let mut last_i = vec![start, start];

    while next_i[0] != next_i[1] {
        for i in 0..next_i.len() {
            let (row, col) = next_i[i];
            let grid_char = grid[row][col];
            let connected: ((usize, usize), (usize, usize)) = match grid_char {
                '|' => ((row - 1, col), (row + 1, col)),
                '-' => ((row, col - 1), (row, col + 1)),
                'L' => ((row - 1, col), (row, col + 1)),
                'J' => ((row - 1, col), (row, col - 1)),
                '7' => ((row, col - 1), (row + 1, col)),
                'F' => ((row, col + 1), (row + 1, col)),
                _ => panic!("unexpected char"),
            };
            let next_step = if last_i[i] == connected.0 {
                connected.1
            } else {
                connected.0
            };
            last_i[i] = next_i[i];
            next_i[i] = next_step;
            loop_points.insert(next_step);
        }
    }
    (grid, start_is_corner, loop_points)
}

fn part1(file: &str) -> u32 {
    let (_, _, loop_points) = parse_grid(file);

    loop_points.len() as u32 / 2
}

fn part2(file: &str) -> u32 {
    let (grid, start_is_corner, loop_points) = parse_grid(file);

    let mut irrelevant_edges = vec!['F', 'J'];
    if start_is_corner {
        // man screw this edge case
        irrelevant_edges.push('S');
    }

    // skip top left and bottom right corners since our ray will just
    // be glancing off those
    let relevant_edges: HashSet<(usize, usize)> = loop_points
        .iter()
        .filter(|&p| !irrelevant_edges.contains(&grid[p.0][p.1]))
        .map(|p| *p)
        .collect();

    // raycasting with diagonal lines because I wasted enough time trying to figure out
    // the special case of horizontal raycasting and running along an edge
    let mut row = 0;
    let mut col = 0;
    let mut count = 0;
    while !(row == grid.len() - 1 && col == grid[0].len() - 1) {
        let mut cur_row = row;
        let mut cur_col = col;
        let mut in_loop = false;
        // cast a ray up and to the right
        while cur_col < grid[0].len() {
            if relevant_edges.contains(&(cur_row, cur_col)) {
                in_loop = !in_loop;
                //println!("toggling loop: {}, {}, {}", cur_row, cur_col, in_loop);
            } else if in_loop && !loop_points.contains(&(cur_row, cur_col)) {
                //println!("in loop: {}, {}", cur_row, cur_col);
                count += 1;
            }

            if cur_row == 0 {
                break;
            }

            cur_row -= 1;
            cur_col += 1;
        }
        if row == grid.len() - 1 {
            col += 1;
        } else {
            row += 1;
        }
    }
    count
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
