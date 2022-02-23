use aoc::*;

const DAY: u8 = 7;

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

fn parse_input(input: String) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|pos| pos.parse::<usize>().unwrap())
        .collect()
}

fn fuel_expended(input: &Vec<usize>, dest: usize) -> usize {
    input.iter().fold(0, |acc, &pos| {
        acc + if pos > dest { pos - dest } else { dest - pos }
    })
}

fn sum_1_to_n(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn fuel_expended_part_2(input: &Vec<usize>, dest: usize) -> usize {
    input.iter().fold(0, |acc, &pos| {
        acc + sum_1_to_n(if pos > dest { pos - dest } else { dest - pos })
    })
}

fn part_1(input: &Vec<usize>) -> usize {
    let max = *input.iter().max().unwrap();
    (0..=max)
        .map(|dest| fuel_expended(input, dest))
        .min()
        .unwrap()
}

fn part_2(input: &Vec<usize>) -> usize {
    let max = *input.iter().max().unwrap();
    (0..=max)
        .map(|dest| fuel_expended_part_2(input, dest))
        .min()
        .unwrap()
}

#[cfg(test)]
mod day_7_tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_fuel_expended() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(fuel_expended(&parsed, 2), 37);
        assert_eq!(fuel_expended(&parsed, 1), 41);
        assert_eq!(fuel_expended(&parsed, 3), 39);
        assert_eq!(fuel_expended(&parsed, 10), 71);
    }
    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        println!(
            "{} {}",
            parsed.iter().map(|x| x.pow(2)).sum::<usize>(),
            parsed.len()
        );
        assert_eq!(part_1(&parsed), 37);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 325528);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 0);
    }
    #[test]
    fn test_fuel_expended_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(fuel_expended_part_2(&parsed, 5), 168);
        assert_eq!(fuel_expended_part_2(&parsed, 2), 206);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 0);
    }
}
