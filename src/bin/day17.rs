use aoc::*;
use std::error::Error;
use std::str::FromStr;

const DAY: u8 = 17;

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

#[derive(Debug, PartialEq, Eq)]
struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

fn sum_to_n(n: isize) -> isize {
    (n.pow(2) - n) / 2
}

impl Target {
    fn is_inside(&self, point: (isize, isize)) -> bool {
        point.0 >= self.x_min
            && point.0 <= self.x_max
            && point.1 >= self.y_min
            && point.1 <= self.y_max
    }

    fn is_past(&self, point: (isize, isize)) -> bool {
        point.0 > self.x_max || point.1 < self.y_min
    }
}

impl FromStr for Target {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().replace("target area: x=", "").replace(" y=", "");
        let mut parts = s.split(',');
        let mut x = parts.next().unwrap().split("..");
        let mut y = parts.next().unwrap().split("..");
        Ok(Target {
            x_min: x.next().unwrap().parse().unwrap(),
            x_max: x.next().unwrap().parse().unwrap(),
            y_min: y.next().unwrap().parse().unwrap(),
            y_max: y.next().unwrap().parse().unwrap(),
        })
    }
}

struct Launch {
    vx: isize,
    vy: isize,
}

impl Launch {
    fn position(&self, n: isize) -> (isize, isize) {
        // x stops changing when n >= vx
        let nx = std::cmp::min(n, self.vx);
        (nx * self.vx - sum_to_n(nx), n * self.vy - sum_to_n(n))
    }

    fn will_hit_target(&self, target: &Target) -> bool {
        for n in 0.. {
            let point = self.position(n);
            if target.is_inside(point) {
                return true;
            } else if target.is_past(point) {
                return false;
            }
        }
        // totally possible
        false
    }

    fn max_height(&self) -> isize {
        let mut max = 0;
        let mut prev = 0;
        for n in 0.. {
            let y = self.position(n).1;
            if y < prev {
                break;
            }
            max = y;
            prev = y;
        }
        max
    }
}

fn parse_input(input: String) -> Target {
    input.parse().unwrap()
}

fn part_1(input: &Target) -> isize {
    let mut max = 0;
    let min = (input.x_min as f64).sqrt() as isize;
    for vx in min..=input.x_max {
        for vy in min..=input.x_max {
            let launch = Launch { vx, vy };
            if launch.will_hit_target(input) {
                max = std::cmp::max(max, launch.max_height());
            }
        }
    }
    max
}

fn part_2(input: &Target) -> usize {
    let mut count = 0;
    let min = (input.x_min as f64).sqrt() as isize;
    for vx in min..=input.x_max {
        for vy in -input.x_max..=input.x_max {
            let launch = Launch { vx, vy };
            if launch.will_hit_target(input) {
                count += 1
            }
        }
    }
    count
}

#[cfg(test)]
mod day_17_tests {
    use super::*;

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_launch_position() {
        let launch = Launch { vx: 7, vy: 2 };
        let spots: Vec<(isize, isize)> = (1..=7).map(|n| launch.position(n)).collect();
        assert_eq!(
            spots,
            vec![
                (7, 2),
                (13, 3),
                (18, 3),
                (22, 2),
                (25, 0),
                (27, -3),
                (28, -7)
            ]
        );

        let launch = Launch { vx: 6, vy: 3 };
        let spots: Vec<(isize, isize)> = (1..=9).map(|n| launch.position(n)).collect();
        assert_eq!(
            spots,
            vec![
                (6, 3),
                (11, 5),
                (15, 6),
                (18, 6),
                (20, 5),
                (21, 3),
                (21, 0),
                (21, -4),
                (21, -9)
            ]
        );
    }
    #[test]
    fn test_parse() {
        let target: Target = "target area: x=20..30, y=-10..-5".parse().unwrap();
        assert_eq!(
            target,
            Target {
                x_min: 20,
                x_max: 30,
                y_min: -10,
                y_max: -5
            }
        );
    }
    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 45);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 3160);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 112);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 0);
    }
}
