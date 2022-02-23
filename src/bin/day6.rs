use aoc::*;

const DAY: u8 = 6;

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

type LanternfishPop = Vec<usize>;

fn parse_input(input: String) -> LanternfishPop {
    let mut lanternfish = Vec::from([0; 9]);
    input
        .trim()
        .split(',')
        .for_each(|timer| lanternfish[timer.parse::<usize>().unwrap()] += 1);
    lanternfish
}

fn part_1(input: &LanternfishPop) -> usize {
    (1..=80)
        .fold(input.clone(), |pop, _| {
            let mut pop = pop;
            let birthing = pop.remove(0);
            pop[6] += birthing;
            pop.push(birthing);
            pop
        })
        .iter()
        .sum()
}

fn part_2(input: &LanternfishPop) -> usize {
    (1..=256)
        .fold(input.clone(), |pop, _| {
            let mut pop = pop;
            let birthing = pop.remove(0);
            pop[6] += birthing;
            pop.push(birthing);
            pop
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod day_6_tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 5934);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 393019);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 26984457539);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1757714216975);
    }
}
