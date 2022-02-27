use aoc::*;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

const DAY: u8 = 11;

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

#[derive(Clone)]
struct Grid {
    vec: Vec<usize>,
    size: usize,
}
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.vec
                .chunks(self.size)
                .fold(String::new(), |s, row| {
                    s + &row.iter().fold(String::new(), |s, &x| {
                        s + &(if x == 0 {
                            format!("\x1b[93m{}\x1b[0m", x)
                        } else {
                            x.to_string()
                        })
                    }) + "\n"
                })
                .trim()
        )
    }
}

fn neighbors(grid: &Grid, index: usize) -> VecDeque<usize> {
    let mut neighbors = VecDeque::new();
    let right = index % grid.size < grid.size - 1;
    let up = index >= grid.size;
    let left = index % grid.size != 0;
    let down = index + grid.size < grid.vec.len();

    // Neighbors positive around starting at the right
    if right {
        neighbors.push_back(index + 1);
    }
    if right && up {
        neighbors.push_back(index + 1 - grid.size);
    }
    if up {
        neighbors.push_back(index - grid.size);
    }
    if left && up {
        neighbors.push_back(index - 1 - grid.size);
    }
    if left {
        neighbors.push_back(index - 1);
    }
    if left && down {
        neighbors.push_back(index - 1 + grid.size);
    }
    if down {
        neighbors.push_back(index + grid.size);
    }
    if right && down {
        neighbors.push_back(index + 1 + grid.size);
    }

    neighbors
}

fn step(grid: &mut Grid) -> usize {
    // Increment energy level of all octopi
    grid.vec.iter_mut().for_each(|octopus| *octopus += 1);

    // Flash high energy octopi
    let mut queue: VecDeque<usize> = grid
        .vec
        .iter()
        .enumerate()
        .filter(|(_, &energy)| energy > 9)
        .map(|(i, _)| i)
        .collect();
    let mut flashed = HashSet::new();
    while let Some(octopus) = queue.pop_front() {
        if flashed.insert(octopus) {
            neighbors(grid, octopus).iter().for_each(|&neighbor| {
                let energy = &mut grid.vec[neighbor];
                *energy += 1;
                // This octo should flash if it has >9 energy. However, it can
                // only have >10 energy if it is already been queued to flash.
                if *energy == 10 {
                    queue.push_back(neighbor);
                }
            })
        }
    }

    // Reset flashed octopi's energy to zero
    flashed.iter().for_each(|&octopus| grid.vec[octopus] = 0);

    flashed.len()
}

fn parse_input(input: String) -> Grid {
    Grid {
        vec: input
            .replace("\n", "")
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
        size: input.lines().next().unwrap().len(),
    }
}

fn part_1(input: &Grid) -> usize {
    let mut grid = input.clone();
    (1..=100).fold(0, |flashes, _| flashes + step(&mut grid))
}

fn part_2(input: &Grid) -> usize {
    let all = input.vec.len();
    let mut grid = input.clone();
    (1..).find(|_| step(&mut grid) == all).unwrap()
}

#[cfg(test)]
mod day_11_tests {
    use super::*;

    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 1656);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 1661);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 195);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 334);
    }
}
