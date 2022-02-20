use aoc::*;
use std::error::Error;
use std::str::FromStr;

fn main() {
    println!("==== Day 2 ====");
    let input = get_input(2);
    println!("Input size: {}", input.len());
    let parsed_input = parse_input(input);
    let part_1 = part_1(&parsed_input);
    println!("Part 1: {}", part_1);
    let part_2 = part_2(&parsed_input);
    println!("Part 2: {}", part_2);
}

enum CourseCommand {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for CourseCommand {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let dist = parts[1].parse().unwrap();
        match parts[0] {
            "forward" => Ok(CourseCommand::Forward(dist)),
            "down" => Ok(CourseCommand::Down(dist)),
            "up" => Ok(CourseCommand::Up(dist)),
            _ => unimplemented!(),
        }
    }
}

fn parse_input(input: String) -> Vec<CourseCommand> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part_1(input: &Vec<CourseCommand>) -> usize {
    // (horizontal pos, depth)
    let change = input.iter().fold((0, 0), |acc, command| match command {
        CourseCommand::Forward(dist) => (acc.0 + dist, acc.1),
        CourseCommand::Down(dist) => (acc.0, acc.1 + dist),
        CourseCommand::Up(dist) => (acc.0, acc.1 - dist),
    });
    change.0 * change.1
}

fn part_2(input: &Vec<CourseCommand>) -> usize {
    // (horizontal pos, depth, aim)
    let change = input.iter().fold((0, 0, 0), |acc, command| match command {
        CourseCommand::Forward(dist) => (acc.0 + dist, acc.1 + acc.2 * dist, acc.2),
        CourseCommand::Down(dist) => (acc.0, acc.1, acc.2 + dist),
        CourseCommand::Up(dist) => (acc.0, acc.1, acc.2 - dist),
    });
    change.0 * change.1
}

#[cfg(test)]
mod day_2_tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 150);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(2);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 1383564);
    }
    #[test]
    fn test_part_2() {
        let input = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 900);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(2);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1488311643);
    }
}
