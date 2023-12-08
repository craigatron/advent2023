use std::cmp::Ordering;
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [u32; 5],
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: [u32; 5], bid: u32) -> Hand {
        let mut counts: HashMap<u32, u8> = HashMap::new();
        let mut num_jokers: u8 = 0;
        for c in cards {
            if c == 1 {
                num_jokers += 1;
            } else {
                *counts.entry(c).or_insert(0) += 1;
            }
        }
        // I'm so sorry
        let mut hand_type = HandType::HighCard;
        if counts.len() == 1 {
            hand_type = HandType::FiveOfAKind;
        } else if counts.values().any(|&v| v == 4) {
            if num_jokers == 1 {
                hand_type = HandType::FiveOfAKind;
            } else {
                hand_type = HandType::FourOfAKind;
            }
        } else if counts.values().any(|&v| v == 3) && counts.values().any(|v| *v == 2) {
            // not possible for a joker to appear here
            hand_type = HandType::FullHouse;
        } else if counts.values().any(|&v| v == 3) {
            if num_jokers == 2 {
                hand_type = HandType::FiveOfAKind;
            } else if num_jokers == 1 {
                hand_type = HandType::FourOfAKind;
            } else {
                hand_type = HandType::ThreeOfAKind;
            }
        } else if counts.values().any(|&v| v == 2) {
            if num_jokers == 1 && counts.len() == 2 {
                hand_type = HandType::FullHouse;
            } else if num_jokers == 1 && counts.len() == 3 {
                hand_type = HandType::ThreeOfAKind;
            } else if num_jokers == 0 && counts.len() == 3 {
                hand_type = HandType::TwoPair;
            } else if num_jokers == 3 {
                hand_type = HandType::FiveOfAKind;
            } else if num_jokers == 2 {
                hand_type = HandType::FourOfAKind;
            } else if num_jokers == 1 {
                hand_type = HandType::ThreeOfAKind;
            } else {
                hand_type = HandType::OnePair;
            }
        } else if num_jokers == 5 {
            hand_type = HandType::FiveOfAKind;
        } else if num_jokers == 4 {
            hand_type = HandType::FiveOfAKind;
        } else if num_jokers == 3 {
            hand_type = HandType::FourOfAKind;
        } else if num_jokers == 2 {
            hand_type = HandType::ThreeOfAKind;
        } else if num_jokers == 1 {
            hand_type = HandType::OnePair;
        }

        Hand {
            cards: cards,
            bid: bid,
            hand_type: hand_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            let mut ordering = Ordering::Equal;
            for i in 0..self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    ordering = self.cards[i].cmp(&other.cards[i]);
                    break;
                }
            }
            ordering
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn hands_from_file(file: &str, use_jokers: bool) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut cards: [u32; 5] = [0, 0, 0, 0, 0];
        for (i, c) in parts[0].chars().enumerate() {
            cards[i] = match c {
                'T' => 10,
                'J' => {
                    if use_jokers {
                        1
                    } else {
                        11
                    }
                }
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => c.to_digit(10).unwrap(),
            }
        }
        hands.push(Hand::new(cards, parts[1].parse::<u32>().unwrap()));
    }
    hands
}

fn part1(file: &str) -> u32 {
    let mut hands = hands_from_file(file, false);
    hands.sort();

    let mut sum: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i + 1) as u32 * hand.bid;
    }
    sum
}

fn part2(file: &str) -> u32 {
    let mut hands = hands_from_file(file, true);
    hands.sort();

    let mut sum: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i + 1) as u32 * hand.bid;
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
