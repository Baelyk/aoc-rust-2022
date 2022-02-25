use aoc::*;
use std::collections::HashSet;
use std::collections::LinkedList;

const DAY: u8 = 9;

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

struct Grid {
    vec: Vec<usize>,
    size: usize,
}

fn neighbors(grid: &Grid, index: usize) -> LinkedList<usize> {
    let mut neighbors = LinkedList::new();
    // Left
    if index % grid.size != 0 {
        neighbors.push_back(index - 1);
    }
    // Right
    if index % grid.size < grid.size - 1 {
        neighbors.push_back(index + 1);
    }
    // Up
    if index >= grid.size {
        neighbors.push_back(index - grid.size);
    }
    // Down
    if index + grid.size < grid.vec.len() {
        neighbors.push_back(index + grid.size);
    }
    neighbors
}

fn is_low_point(grid: &Grid, index: usize) -> bool {
    // Neighbors: right, up, left, below
    neighbors(grid, index).iter().all(|&neighbor| {
        return grid.vec[index] < grid.vec[neighbor];
    })
}

fn basin_size(grid: &Grid, index: usize) -> usize {
    let mut queue = LinkedList::from([index]);
    let mut basin = HashSet::from([index]);

    while let Some(curr) = queue.pop_front() {
        let mut neighbors: LinkedList<usize> = neighbors(grid, curr)
            .iter()
            .filter(|&neighbor| {
                grid.vec[*neighbor] != 9
                    && grid.vec[*neighbor] > grid.vec[curr]
                    && basin.insert(*neighbor)
            })
            .copied()
            .collect();
        queue.append(&mut neighbors)
    }

    basin.len()
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
    input.vec.iter().enumerate().fold(0, |acc, (i, x)| {
        acc + if is_low_point(input, i) { x + 1 } else { 0 }
    })
}

fn part_2(input: &Grid) -> usize {
    let mut basins: Vec<usize> = input
        .vec
        .iter()
        .enumerate()
        .map(|(i, _)| {
            if is_low_point(input, i) {
                basin_size(input, i)
            } else {
                0
            }
        })
        .filter(|&size| size != 0)
        .collect();
    basins.sort();
    basins[basins.len() - 1] * basins[basins.len() - 2] * basins[basins.len() - 3]
}

#[cfg(test)]
mod day_9_tests {
    use super::*;

    const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 15);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 475);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1134);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 0);
    }
}
