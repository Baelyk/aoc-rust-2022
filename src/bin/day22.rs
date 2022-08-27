use aoc::*;
use std::cmp::{max, min};
use std::error::Error;
use std::fmt;
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
enum Axis {
    X,
    Y,
    Z,
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

    fn change_coord(&self, axis: Axis, coord: isize) -> Point {
        match axis {
            Axis::X => Point::new(coord, self.y, self.z),
            Axis::Y => Point::new(self.x, coord, self.z),
            Axis::Z => Point::new(self.x, self.y, coord),
        }
    }

    fn min(a: &Point, b: &Point) -> Point {
        Point {
            x: min(a.x, b.x),
            y: min(a.y, b.y),
            z: min(a.z, b.z),
        }
    }

    fn max(a: &Point, b: &Point) -> Point {
        Point {
            x: max(a.x, b.x),
            y: max(a.y, b.y),
            z: max(a.z, b.z),
        }
    }
}

//impl PartialOrd for Point {
//// Can be ordered if all components are e.g. <, so (1, 1, 1) cannot be
//// compared with (0, 1, 2)
//fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//if self.x < other.x && self.y < other.y && self.z < other.z {
//Some(Ordering::Less)
//} else if self == other {
//Some(Ordering::Equal)
//} else if self.x > other.x && self.y > other.y && self.z > other.z {
//Some(Ordering::Greater)
//} else {
//None
//}
//}
//}

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
        self.intersection(other) == *other
    }

    fn intersection(&self, other: &Self) -> Self {
        Cuboid {
            min: Point::max(&self.min, &other.min),
            max: Point::min(&self.max, &other.max),
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).size() > 0
    }

    fn size(&self) -> usize {
        if self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z {
            0
        } else {
            let size = (1 + self.max.x - self.min.x)
                * (1 + self.max.y - self.min.y)
                * (1 + self.max.z - self.min.z);
            size.try_into().unwrap()
        }
    }

    /// Split this cuboid in two along the plane at `plane` in the `axis`-axis,
    /// e.g. axis = Axis::X and plane = 1 splits along the x = 1 plane. The second component
    /// contains the plane.
    fn split_below(&self, axis: Axis, plane: isize) -> (Cuboid, Cuboid) {
        (
            Cuboid {
                min: self.min,
                max: self.max.change_coord(axis, plane - 1),
            },
            Cuboid {
                min: self.min.change_coord(axis, plane),
                max: self.max,
            },
        )
    }
    fn split_above(&self, axis: Axis, plane: isize) -> (Cuboid, Cuboid) {
        (
            Cuboid {
                min: self.min.change_coord(axis, plane + 1),
                max: self.max,
            },
            Cuboid {
                min: self.min,
                max: self.max.change_coord(axis, plane),
            },
        )
    }

    fn breakup(&self, hole: &Cuboid) -> Vec<Cuboid> {
        let mut parts = vec![];

        let (split_off, remnants) = self.split_below(Axis::X, hole.min.x);
        parts.push(split_off);
        let (split_off, remnants) = remnants.split_above(Axis::X, hole.max.x);
        parts.push(split_off);

        let (split_off, remnants) = remnants.split_below(Axis::Y, hole.min.y);
        parts.push(split_off);
        let (split_off, remnants) = remnants.split_above(Axis::Y, hole.max.y);
        parts.push(split_off);

        let (split_off, remnants) = remnants.split_below(Axis::Z, hole.min.z);
        parts.push(split_off);
        let (split_off, _) = remnants.split_above(Axis::Z, hole.max.z);
        parts.push(split_off);

        //parts.iter().for_each(|part| println!("{}", part));

        parts.into_iter().filter(|part| part.size() > 0).collect()
    }

    /// Split off the intersection of self and other, in the process breaking up what remains of
    /// each into at most 6 cuboids each. Returns (selfs, others, cuboids).
    fn split_off_intersection(&self, other: &Self) -> (Vec<Cuboid>, Vec<Cuboid>, Cuboid) {
        // In order to assume that self.min <= other.min, switch self and other if self.min >
        // other.min:
        //if self.min > other.min {
        //let (others, selfs, intersection) = other.split_off_intersection(self);
        //return (selfs, others, intersection);
        //}

        let intersection = self.intersection(other);
        let selfs = self.breakup(&intersection);
        let others = other.breakup(&intersection);

        //selfs.iter().for_each(|p| println!("{}", p));
        //println!("others:");
        //others.iter().for_each(|p| println!("{}", p));

        (selfs, others, intersection)
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "x={}..{},y={}..{},z={}..{}",
            self.min.x, self.max.x, self.min.y, self.max.y, self.min.z, self.max.z
        )
    }
}

#[derive(Clone)]
struct RebootStep {
    cuboid: Cuboid,
    state: bool,
}

impl RebootStep {
    fn new(cuboid: Cuboid, state: bool) -> Self {
        Self { cuboid, state }
    }
}

impl fmt::Display for RebootStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}",
            if self.state { "on" } else { "off" },
            self.cuboid
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

fn process_steps(steps: &Vec<RebootStep>) -> usize {
    let mut on_cuboids: Vec<Cuboid> = vec![];
    let mut steps = steps.clone();

    while !steps.is_empty() {
        let step = steps.pop().unwrap();
        //println!("\nStep {}", step);

        let cuboid = on_cuboids
            .iter()
            .enumerate()
            .find(|(_, cuboid)| cuboid.intersects(&step.cuboid));

        if let Some((i, cuboid)) = cuboid {
            //println!("{} intersects \n {}", step, cuboid);
            let (step_parts, mut cuboid_parts, intersection) =
                step.cuboid.split_off_intersection(&cuboid);

            // Turn the remaining parts of this step into more steps
            steps.append(
                &mut step_parts
                    .into_iter()
                    .map(|part| RebootStep::new(part, step.state))
                    .collect(),
            );

            // Add back the nonintersection parts of the on cuboid
            on_cuboids.remove(i);
            on_cuboids.append(&mut cuboid_parts);

            // Add back the intersection if this step turns things on
            if step.state {
                on_cuboids.push(intersection);
            }
        } else if step.state {
            on_cuboids.push(step.cuboid);
        } else {
            //println!("Step {} does not intersect any of", step);
            //on_cuboids
            //.iter()
            //.for_each(|cuboid| println!("  {}", cuboid));
        }

        //println!(
        //"After {}, {} cubes are on, {} steps remaining",
        //step,
        //on_cuboids.iter().map(|cuboid| cuboid.size()).sum::<usize>(),
        //steps.len()
        //);
    }
    on_cuboids.iter().map(|cuboid| cuboid.size()).sum()
}

fn parse_input(input: String) -> Vec<RebootStep> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: &Vec<RebootStep>) -> usize {
    let init_cuboid = Cuboid::new(-50, -50, -50, 50, 50, 50);
    //let mut on_cuboids = vec![];

    let steps: Vec<RebootStep> = input
        .iter()
        .filter(|step| init_cuboid.contains(&step.cuboid))
        .rev()
        .cloned()
        .collect();

    process_steps(&steps)
}

fn part_2(input: &Vec<RebootStep>) -> usize {
    let steps = input.clone();

    process_steps(&steps)
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
    fn test_split_off_intersection() {
        let a = "on x=-49..-5,y=-3..45,z=-29..18"
            .parse::<RebootStep>()
            .unwrap()
            .cuboid;
        let b = "on x=-41..9,y=-7..43,z=-33..15"
            .parse::<RebootStep>()
            .unwrap()
            .cuboid;
        //println!("{}", a);
        //println!("{}", b);
        assert_eq!(a.split_off_intersection(&b).0.len(), 3)
    }
    #[test]
    fn test_cuboid_size() {
        assert_eq!(Cuboid::new(0, 0, 0, 5, 5, 5).size(), 5 * 5 * 5);
        assert_eq!(Cuboid::new(6, 6, 6, 5, 5, 5).size(), 0);
    }

    #[test]
    fn test_part_1_smol() {
        let input = String::from(
            "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10",
        );
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 39);
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
        assert_eq!(part_1(&parsed), 537042);
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
