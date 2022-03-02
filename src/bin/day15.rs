use aoc::grid::Grid;
use aoc::*;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

const DAY: u8 = 15;

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

#[derive(Copy, Clone, Eq, PartialEq)]
struct NodeCost {
    node: usize,
    cost: usize,
}

impl NodeCost {
    fn new(node: usize, cost: usize) -> NodeCost {
        NodeCost { node, cost }
    }
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> Ordering {
        // Switched LHS and RHS so smaller cost is better
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn pathfind(grid: &Grid) -> usize {
    let start = grid.first();
    let end = grid.last();
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut previous = HashMap::new();

    queue.push(NodeCost::new(start, 0));

    while let Some(NodeCost { node, cost }) = queue.pop() {
        visited.insert(node);

        for &neighbor in grid
            .neighbors(node)
            .iter()
            .filter(|neighbor| !visited.contains(neighbor))
        {
            let new_dist = cost + grid.get_index(neighbor);
            let dist = distances.entry(neighbor).or_insert(usize::MAX);
            if new_dist < *dist {
                *dist = new_dist;
                previous.insert(neighbor, node);
                if neighbor == end {
                    break;
                }
                queue.push(NodeCost::new(neighbor, *dist));
            }
        }
    }

    let mut cost = grid.get_index(end);
    let mut prev = end;
    while let Some(node) = previous.get(&prev) {
        cost += grid.get_index(*node);
        prev = *node;
    }

    cost - grid.get_index(start)
}

fn enlarge_grid(grid: &Grid) -> Grid {
    let width = grid.width();
    let height = grid.height();

    let big_grid_elements = vec![0; height * 5 * width * 5];
    let mut big_grid = Grid::new(big_grid_elements, width * 5);

    for x in 0..width {
        for y in 0..height {
            let mut entry = grid.get_coord(x, y);
            for i in 0..9 {
                if i < 5 {
                    for t in 0..=i {
                        *big_grid.get_mut_coord(x + width * t, y + height * (i - t)) = entry;
                    }
                } else {
                    let i = 8 - i;
                    for t in 0..=i {
                        *big_grid.get_mut_coord(x + width * (4 - (i - t)), y + height * (4 - t)) =
                            entry;
                    }
                }
                entry += 1;
                if entry > 9 {
                    entry = 1;
                }
            }
        }
    }

    big_grid
}

fn parse_input(input: String) -> Grid {
    input.parse().unwrap()
}

fn part_1(input: &Grid) -> usize {
    pathfind(input)
}

fn part_2(input: &Grid) -> usize {
    let big_grid = enlarge_grid(input);
    pathfind(&big_grid)
}

#[cfg(test)]
mod day_15_tests {
    use super::*;

    const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 40);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 415);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 315);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 0);
    }
}
