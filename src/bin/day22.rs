use aoc::*;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::ops::{Add, Index, IndexMut, Mul, Range, Sub};
use std::str::FromStr;

const DAY: u8 = 22;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn compare(&self, other: &Self) -> (Ordering, Ordering, Ordering) {
        (
            self.x.cmp(&other.x),
            self.y.cmp(&other.y),
            self.z.cmp(&other.z),
        )
    }
}

impl PartialOrd for Point {
    // Can be ordered if all components are e.g. <, so (1, 1, 1) cannot be
    // compared with (0, 1, 2)
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x < other.x && self.y < other.y && self.z < other.z {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else if self.x > other.x && self.y > other.y && self.z > other.z {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Cuboid {
    min: Point,
    max: Point,
}

impl Cuboid {
    fn new(
        x_min: isize,
        y_min: isize,
        z_min: isize,
        x_max: isize,
        y_max: isize,
        z_max: isize,
    ) -> Self {
        let min = Point::new(x_min, y_min, z_min);
        let max = Point::new(x_max, y_max, z_max);
        Cuboid { min, max }
    }

    fn contains(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn intersects(&self, other: &Self) -> bool {
        self.min <= other.max && self.max >= other.min
    }

    /// Split the intersection of self and other off and of either. The first
    /// component is the remnants of self, and the second is the intersection
    /// and the remnants of other.
    fn split_off_intersection(&self, other: &Self) -> (Vec<Cuboid>, Vec<Cuboid>) {}
}

struct RebootStep {
    cuboid: Cuboid,
    state: bool,
}

impl fmt::Display for RebootStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} x={}..{},y={}..{},z={}..{})",
            if self.state { "on" } else { "off" },
            self.cuboid.min.x,
            self.cuboid.max.x,
            self.cuboid.min.y,
            self.cuboid.max.y,
            self.cuboid.min.z,
            self.cuboid.max.z
        )
    }
}

impl FromStr for RebootStep {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (state, region) = s.split_once(' ').unwrap();
        let state = state == "on";
        let mut region = region.split(',');
        let region_x = region
            .next()
            .unwrap()
            .get(2..)
            .unwrap()
            .split_once("..")
            .unwrap();
        let region_x = (region_x.0.parse().unwrap(), region_x.1.parse().unwrap());
        let region_y = region
            .next()
            .unwrap()
            .get(2..)
            .unwrap()
            .split_once("..")
            .unwrap();
        let region_y = (region_y.0.parse().unwrap(), region_y.1.parse().unwrap());
        let region_z = region
            .next()
            .unwrap()
            .get(2..)
            .unwrap()
            .split_once("..")
            .unwrap();
        let region_z = (region_z.0.parse().unwrap(), region_z.1.parse().unwrap());
        let cuboid = Cuboid {
            min: Point {
                x: region_x.0,
                y: region_y.0,
                z: region_z.0,
            },
            max: Point {
                x: region_x.1,
                y: region_y.1,
                z: region_z.1,
            },
        };
        Ok(RebootStep { state, cuboid })
    }
}

fn parse_input(input: String) -> Vec<RebootStep> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: &Vec<RebootStep>) -> usize {
    unimplemented!()
}

fn part_2(input: &Vec<RebootStep>) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod day_0_tests {
    use super::*;

    const TEST_INPUT: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn test_contains() {
        assert!(Cuboid::new(0, 0, 0, 5, 5, 5).contains(&Cuboid::new(1, 0, 1, 2, 3, 2)));
        assert!(Cuboid::new(0, 0, 0, 5, 5, 5).contains(&Cuboid::new(0, 0, 0, 5, 5, 5)));
        assert!(!Cuboid::new(0, 0, 0, 5, 5, 5).contains(&Cuboid::new(0, 0, 0, 5, 5, 6)));
        assert!(!Cuboid::new(0, 0, 0, 5, 5, 5).contains(&Cuboid::new(0, -1, 0, 5, 5, 5)));
        assert!(!Cuboid::new(0, 0, 0, 5, 5, 5).contains(&Cuboid::new(5, 5, 5, 8, 8, 8)));
    }
    #[test]
    fn test_intersects() {
        assert!(Cuboid::new(0, 0, 0, 5, 5, 5).intersects(&Cuboid::new(1, 1, 1, 2, 3, 2)));
        assert!(Cuboid::new(0, 0, 0, 5, 5, 5).intersects(&Cuboid::new(0, 0, 0, 5, 5, 5)));
        assert!(Cuboid::new(0, 0, 0, 5, 5, 5).intersects(&Cuboid::new(0, 0, 0, 5, 5, 6)));
        assert!(Cuboid::new(0, 0, 0, 5, 5, 5).intersects(&Cuboid::new(0, -1, 0, 5, 5, 5)));
        assert!(Cuboid::new(0, 0, 0, 5, 5, 5).intersects(&Cuboid::new(5, 5, 5, 8, 8, 8)));
        assert!(!Cuboid::new(0, 0, 0, 5, 5, 5).intersects(&Cuboid::new(6, 6, 6, 8, 8, 8)));
        assert!(!Cuboid::new(7, 7, 7, 8, 8, 8).intersects(&Cuboid::new(0, 0, 0, 5, 5, 5)));
        assert!(!Cuboid::new(0, 0, 0, 5, 5, 5).intersects(&Cuboid::new(6, 5, 6, 8, 8, 8)));
    }

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 590784);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 0);
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
