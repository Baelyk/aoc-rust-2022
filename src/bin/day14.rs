use aoc::*;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

const DAY: u8 = 14;

fn main() {
    println!("==== Day {} ====", DAY);
    let input = get_input(DAY);
    println!("Input size: {}", input.len());
    let parsed_input = parse_input(input);
    let part_1 = part_1(&parsed_input);
    println!("Part 1: {}", part_1);
    let part_2 = part_2(&parsed_input);
    println!("Part 2: {}", part_2);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pair(char, char);

impl Pair {
    fn new(first: char, second: char) -> Pair {
        Pair(first, second)
    }
}

impl FromStr for Pair {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.chars();
        Ok(Pair(parts.next().unwrap(), parts.next().unwrap()))
    }
}

#[derive(Clone)]
struct PolymerBuilder {
    polymer: HashMap<Pair, usize>,
    rules: HashMap<Pair, char>,
    count: HashMap<char, usize>,
}

impl PolymerBuilder {
    fn grow(&mut self) {
        let mut polymer = HashMap::new();
        self.polymer.iter().for_each(|(pair, n)| {
            let middle = *self.rules.get(pair).unwrap();
            self.count
                .entry(middle)
                .and_modify(|m| *m += n)
                .or_insert(*n);
            polymer
                .entry(Pair::new(pair.0, middle))
                .and_modify(|m| *m += n)
                .or_insert(*n);
            polymer
                .entry(Pair::new(middle, pair.1))
                .and_modify(|m| *m += n)
                .or_insert(*n);
        });
        self.polymer = polymer;
    }

    fn counts(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        self.polymer.iter().for_each(|(pair, n)| {
            counts.entry(pair.0).and_modify(|m| *m += n).or_insert(*n);
            counts.entry(pair.1).and_modify(|m| *m += n).or_insert(*n);
        });
        counts.iter_mut().for_each(|(_, n)| *n = (*n + 1) / 2);
        counts
    }

    fn len(&self) -> usize {
        self.polymer.values().sum::<usize>() + 1
    }

    fn count(&self) -> usize {
        let mut counts: Vec<usize> = self.count.values().cloned().collect();
        counts.sort();
        counts[counts.len() - 1] - counts[0]
    }
}

impl FromStr for PolymerBuilder {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("\n\n").collect();
        let polymer = parts[0]
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|pair| (format!("{}{}", pair[0], pair[1]).parse().unwrap(), 1))
            .collect();
        let rules = parts[1]
            .lines()
            .map(|line| {
                let mut parts = line.split(" -> ");
                (
                    parts.next().unwrap().to_owned().parse().unwrap(),
                    parts.next().unwrap().to_owned().chars().next().unwrap(),
                )
            })
            .collect();
        let mut count = HashMap::new();
        parts[0].chars().for_each(|c| {
            count.entry(c).and_modify(|m| *m += 1).or_insert(1);
        });
        Ok(PolymerBuilder {
            polymer,
            rules,
            count,
        })
    }
}

fn parse_input(input: String) -> PolymerBuilder {
    input.parse().unwrap()
}

fn part_1(input: &PolymerBuilder) -> usize {
    let mut polymer = input.clone();
    println!("{}: {} {:?}\n", 0, polymer.len(), polymer.count);
    (1..=10).for_each(|i| {
        polymer.grow();
        println!("{}: {} {:?}\n", i, polymer.len(), polymer.count);
    });
    polymer.count()
}

fn part_2(input: &PolymerBuilder) -> usize {
    let mut polymer = input.clone();
    (1..=40).for_each(|i| {
        polymer.grow();
        println!("{}: {} {:?}\n", i, polymer.len(), polymer.count);
    });
    polymer.count()
}

#[cfg(test)]
mod day_14_tests {
    use super::*;

    const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 1588);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 2874);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 2188189693529);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 0);
    }
}
