use aoc::*;

fn main() {
    println!("==== Day 1 ====");
    let input = get_input(1);
    println!("Input size: {}", input.len());
    let parsed_input = parse_input(input);
    let part_1 = part_1(&parsed_input);
    println!("Part 1: {}", part_1);
    let part_2 = part_2(&parsed_input);
    println!("Part 2: {}", part_2);
}

fn parse_input(input: String) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part_1(input: &Vec<usize>) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(prev, curr)| curr > prev)
        .count()
}

fn part_2(input: &Vec<usize>) -> usize {
    let windows = input.windows(3).map(|w| w.iter().sum()).collect();
    part_1(&windows)
}

#[cfg(test)]
mod day_1_tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 7);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(1);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 1688);
    }
    #[test]
    fn test_part_2() {
        let input = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 5);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(1);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1728);
    }
}
