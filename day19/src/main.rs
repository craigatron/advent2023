use std::collections::{HashMap, VecDeque};
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

#[derive(Clone, Debug, PartialEq)]
enum Comparison {
    GreaterThan,
    LessThan,
}

#[derive(Debug, PartialEq)]
enum Result {
    Workflow(String),
    Accept,
    Reject,
}

#[derive(Clone, Debug)]
struct Constraint {
    category: char,
    comparison: Comparison,
    value: u32,
}

impl Constraint {
    fn inverse(&self) -> Constraint {
        Constraint {
            category: self.category,
            comparison: if self.comparison == Comparison::GreaterThan {
                Comparison::LessThan
            } else {
                Comparison::GreaterThan
            },
            value: if self.comparison == Comparison::GreaterThan {
                self.value + 1
            } else {
                self.value - 1
            },
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<(Constraint, Result)>,
    fail_result: Result,
}

fn parse(file: &str) -> (HashMap<String, Workflow>, Vec<HashMap<char, u32>>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<HashMap<char, u32>> = Vec::new();
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        if line == "" {
            continue;
        }

        if line.starts_with("{") {
            let mut part: HashMap<char, u32> = HashMap::new();
            let part_vals = line[1..line.len() - 1].split(",");
            for val in part_vals {
                let spec: Vec<&str> = val.split("=").collect();
                let char = spec[0].chars().next().unwrap();
                let value = spec[1].parse::<u32>().unwrap();
                part.insert(char, value);
            }
            parts.push(part);
        } else {
            // parse worfklow
            let rules_index = line.chars().position(|c| c == '{').unwrap();
            let workflow_name = line[0..rules_index].to_owned();
            // ignore brackets
            let rules_str = &line[rules_index + 1..line.len() - 1];
            let rule_parts: Vec<&str> = rules_str.split(",").collect();
            let mut rules: Vec<(Constraint, Result)> = Vec::new();
            for i in 0..rule_parts.len() - 1 {
                let rule_part = rule_parts[i];
                let mut rule_chars = rule_part.chars();
                let c = rule_chars.next().unwrap();
                let cmp_char = rule_chars.next().unwrap();
                let colon_i = rule_part.chars().position(|c| c == ':').unwrap();
                let value = rule_part[2..colon_i].parse::<u32>().unwrap();
                let result_str = &rule_part[colon_i + 1..];

                let result = match result_str {
                    "A" => Result::Accept,
                    "R" => Result::Reject,
                    _ => Result::Workflow(result_str.to_owned()),
                };
                rules.push((
                    Constraint {
                        category: c,
                        comparison: if cmp_char == '>' {
                            Comparison::GreaterThan
                        } else {
                            Comparison::LessThan
                        },
                        value: value,
                    },
                    result,
                ));
            }
            let fail_result = match rule_parts[rule_parts.len() - 1] {
                "A" => Result::Accept,
                "R" => Result::Reject,
                _ => Result::Workflow(rule_parts[rule_parts.len() - 1].to_owned()),
            };
            workflows.insert(
                workflow_name,
                Workflow {
                    rules: rules,
                    fail_result: fail_result,
                },
            );
        }
    }

    (workflows, parts)
}

fn part1(file: &str) -> u32 {
    let (workflows, parts) = parse(file);
    let mut sum = 0;
    for part in parts {
        let mut result = &Result::Workflow("in".to_owned());
        loop {
            match result {
                Result::Workflow(wf) => {
                    let workflow = workflows.get(wf).unwrap();
                    let mut wf_result: Option<&Result> = None;
                    for (constraint, res) in workflow.rules.iter() {
                        let part_value = *part.get(&constraint.category).unwrap();
                        let accepted = match constraint.comparison {
                            Comparison::GreaterThan => part_value > constraint.value,
                            Comparison::LessThan => part_value < constraint.value,
                        };
                        if accepted {
                            wf_result = Some(&res);
                            break;
                        }
                    }
                    if let Some(r) = wf_result {
                        result = r;
                    } else {
                        result = &workflow.fail_result;
                    }
                }
                Result::Accept => {
                    sum += part.values().sum::<u32>();
                    break;
                }
                Result::Reject => {
                    break;
                }
            }
        }
    }
    sum
}

fn part2(file: &str) -> u64 {
    let (workflows, _) = parse(file);

    let mut success_constraints: Vec<Vec<Constraint>> = vec![];
    let mut nodes: VecDeque<(&Workflow, Vec<Constraint>)> =
        VecDeque::from([(workflows.get("in").unwrap(), vec![])]);

    while !nodes.is_empty() {
        let (workflow, existing_constraints) = nodes.pop_front().unwrap();
        let mut workflow_constraints: Vec<Constraint> = vec![];
        for (constraint, result) in workflow.rules.iter() {
            let new_constraints = [
                existing_constraints.clone(),
                workflow_constraints.clone(),
                vec![constraint.clone()],
            ]
            .concat();
            match result {
                Result::Accept => {
                    success_constraints.push(new_constraints);
                }
                Result::Reject => {
                    // just ignore, nothing to do here
                }
                Result::Workflow(wf) => {
                    nodes.push_back((workflows.get(wf).unwrap(), new_constraints));
                }
            }
            workflow_constraints.push(constraint.inverse());
        }
        // handle the reject case
        let new_constraints = [existing_constraints.clone(), workflow_constraints.clone()].concat();
        match &workflow.fail_result {
            Result::Accept => {
                success_constraints.push(new_constraints);
            }
            Result::Reject => {
                // just ignore, nothing to do here
            }
            Result::Workflow(wf) => {
                nodes.push_back((workflows.get(wf).unwrap(), new_constraints));
            }
        }
    }

    let mut sum = 0;

    for constraints in success_constraints {
        let mut constraints_by_category: HashMap<char, (u32, u32)> = HashMap::from([
            ('x', (0, 4001)),
            ('m', (0, 4001)),
            ('a', (0, 4001)),
            ('s', (0, 4001)),
        ]);
        for constraint in constraints {
            let mut cat_constraint = *constraints_by_category.get(&constraint.category).unwrap();
            if constraint.comparison == Comparison::GreaterThan
                && constraint.value > cat_constraint.0
            {
                cat_constraint.0 = constraint.value;
                constraints_by_category.insert(constraint.category, cat_constraint);
            } else if constraint.comparison == Comparison::LessThan
                && constraint.value < cat_constraint.1
            {
                cat_constraint.1 = constraint.value;
                constraints_by_category.insert(constraint.category, cat_constraint);
            }
        }

        sum += constraints_by_category
            .values()
            .map(|(min, max)| (max - min - 1) as u64)
            .product::<u64>();
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
