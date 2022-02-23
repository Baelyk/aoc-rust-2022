use aoc::*;
use aoc::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

const DAY: u8 = 5;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_diag(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn get_points_along(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            let x = self.start.x;
            let mut ys = vec![self.start.y, self.end.y];
            ys.sort();
            (ys[0]..=ys[1]).map(|y| Point { x, y }).collect()
        } else if self.start.y == self.end.y {
            let y = self.start.y;
            let mut xs = vec![self.start.x, self.end.x];
            xs.sort();
            (xs[0]..=xs[1]).map(|x| Point { x, y }).collect()
        } else {
            // Line is diagonal
            let xsub = self.start.x > self.end.x;
            let ysub = self.start.y > self.end.y;
            let diff = if xsub {
                self.start.x - self.end.x
            } else {
                self.end.x - self.start.x
            };
            (0..=diff)
                .map(|i| {
                    let x = if xsub {
                        self.start.x - i
                    } else {
                        self.start.x + i
                    };
                    let y = if ysub {
                        self.start.y - i
                    } else {
                        self.start.y + i
                    };
                    Point { x, y }
                })
                .collect()
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Point> = s
            .split(" -> ")
            .map(|s| {
                let mut parts = s.split(',');
                Point {
                    x: parts.next().unwrap().parse().unwrap(),
                    y: parts.next().unwrap().parse().unwrap(),
                }
            })
            .collect();
        Ok(Line {
            start: points[0],
            end: points[1],
        })
    }
}

fn parse_input(input: String) -> Vec<Line> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

fn part_1(input: &Vec<Line>) -> usize {
    let mut vents = HashMap::new();
    input.iter().for_each(|line| {
        if !line.is_diag() {
            //println!("Adding line {}", line);
            line.get_points_along().iter().for_each(|&point| {
                //println!("  Adding point {}", point);
                let count = *vents.get(&point).or(Some(&0)).unwrap();
                vents.insert(point, count + 1);
            })
        }
    });
    vents.iter().filter(|(&key, &count)| count >= 2).count()
}

fn part_2(input: &Vec<Line>) -> usize {
    let mut vents = HashMap::new();
    input.iter().for_each(|line| {
        //println!("Adding line {}", line);
        line.get_points_along().iter().for_each(|&point| {
            //println!("  Adding point {}", point);
            let count = *vents.get(&point).or(Some(&0)).unwrap();
            vents.insert(point, count + 1);
        })
    });
    vents.iter().filter(|(&key, &count)| count >= 2).count()
}

#[cfg(test)]
mod day_0_tests {
    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 5);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 5169);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 12);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 22083);
    }
}
