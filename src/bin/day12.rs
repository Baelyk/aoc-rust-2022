use aoc::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

const DAY: u8 = 12;

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

fn is_small(cave: &String) -> bool {
    cave != "start" && cave != "end" && cave.chars().next().unwrap().is_ascii_lowercase()
}

struct Graph {
    matrix: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            matrix: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: &String, to: &String, add_reverse: bool) {
        match self.matrix.get_mut(from) {
            Some(edges) => {
                edges.insert(to.to_string());
            }
            None => {
                self.matrix
                    .insert(from.to_string(), HashSet::from([to.to_string()]));
            }
        }

        if add_reverse {
            self.add_edge(to, from, false);
        }
    }

    fn neighbors(&self, node: &String) -> &HashSet<String> {
        self.matrix.get(node).unwrap()
    }

    fn find_all_routes(&self, max_visits: usize) -> usize {
        self.find_path_to_end(
            max_visits,
            &HashMap::new(),
            &String::from("start"),
            &String::new(),
        )
    }

    fn find_path_to_end(
        &self,
        max_visits: usize,
        visited: &HashMap<&String, usize>,
        start: &String,
        path: &String,
    ) -> usize {
        if start == "end" {
            return 1;
        }

        return self
            .neighbors(start)
            .iter()
            .filter(|neighbor| *neighbor != "start")
            .map(|neighbor| {
                let mut visited = visited.clone();
                let visit_count = visited.entry(neighbor).or_insert(0);
                *visit_count += 1;
                if !is_small(neighbor) || *visit_count <= max_visits {
                    self.find_path_to_end(
                        max_visits,
                        &visited,
                        neighbor,
                        &format!("{} {}", path, neighbor),
                    )
                } else {
                    0
                }
            })
            .sum();
    }
}

impl FromStr for Graph {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph::new();
        s.lines().for_each(|line| {
            let parts: Vec<String> = line.split('-').map(|part| part.to_string()).collect();
            graph.add_edge(&parts[0], &parts[1], &parts[0] != "start");
        });
        Ok(graph)
    }
}

fn parse_input(input: String) -> Graph {
    input.parse().unwrap()
}

fn part_1(input: &Graph) -> usize {
    input.find_all_routes(1)
}

fn part_2(input: &Graph) -> usize {
    input.find_all_routes(2)
}

#[cfg(test)]
mod day_12_tests {
    use super::*;

    const TEST_INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part_1_1() {
        let input = String::from(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        );
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 10);
    }

    #[test]
    fn test_part_1_2() {
        let input = String::from(
            "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
        );
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 19);
    }

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 226);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 3563);
    }

    #[test]
    fn test_part_2_1() {
        let input = String::from(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        );
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 36);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 0);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 0);
    }
}
