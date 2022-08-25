use aoc::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;
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
    //let part_1 = part_1(&parsed_input);
    //println!("Part 1: {}", part_1);
    let part_2 = part_2(&parsed_input);
    println!("Part 2: {}", part_2);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(isize, isize, isize);
impl Point {
    fn permute(&self, p: Point) -> Point {
        Point(
            self[p.0.try_into().unwrap()],
            self[p.1.try_into().unwrap()],
            self[p.2.try_into().unwrap()],
        )
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 < other.0 && self.1 < other.1 && self.2 < other.2 {
            Some(Ordering::Less)
        } else if self.0 > other.0 && self.1 > other.1 && self.2 > other.2 {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
// Componentwise multiplication
impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}
impl Index<usize> for Point {
    type Output = isize;

    fn index(&self, coord: usize) -> &Self::Output {
        match coord {
            0 => &self.0,
            1 => &self.1,
            _ => &self.2,
        }
    }
}
impl IndexMut<usize> for Point {
    fn index_mut(&mut self, coord: usize) -> &mut Self::Output {
        match coord {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => &mut self.2,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct ReactorInstruction {
    state: bool,
    region: Region,
}
impl FromStr for ReactorInstruction {
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
        let region = Region {
            min: Point(region_x.0, region_y.0, region_z.0),
            max: Point(region_x.1, region_y.1, region_z.1),
        };
        Ok(ReactorInstruction { state, region })
    }
}
impl ReactorInstruction {
    fn is_within_init_region(&self) -> bool {
        Point(-50, -50, -50) <= self.region.min && self.region.max <= Point(50, 50, 50)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Region {
    min: Point,
    max: Point,
}
impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

impl Region {
    fn contains_point(&self, point: &Point) -> bool {
        &self.min <= point && point <= &self.max
    }

    fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        for x in self.min.0..=self.max.0 {
            for y in self.min.1..=self.max.1 {
                for z in self.min.2..=self.max.2 {
                    points.push(Point(x, y, z));
                }
            }
        }
        points
    }

    fn contains(&self, other: &Region) -> bool {
        self.min <= other.min && other.max <= self.max
    }

    fn intersect(&self, other: &Region) -> Region {
        use std::cmp::max;
        use std::cmp::min;
        Region {
            min: Point(
                max(self.min.0, other.min.0),
                max(self.min.1, other.min.1),
                max(self.min.2, other.min.2),
            ),
            max: Point(
                min(self.max.0, other.max.0),
                min(self.max.1, other.max.1),
                min(self.max.2, other.max.2),
            ),
        }
    }

    // For when self contains other
    fn difference(&self, other: &Region) -> Vec<Region> {
        // (x-axis point right towards you)
        // Top
        // x_left (CENTER) x_right
        // Bottom
        // x_left/right extend for full x length
        //
        // (y-axis pointing right towards you)
        // Top
        // y_left (CENTER) y_right
        // Bottom
        // y_left/right extend for other y length only
        let mut regions = vec![];
        // Top
        regions.push(Region {
            min: Point(self.min.0, self.min.1, other.max.2 + 1),
            max: self.max,
        });
        // Bottom
        regions.push(Region {
            min: self.min,
            max: Point(self.max.0, self.max.1, other.min.2 - 1),
        });
        // x left
        regions.push(Region {
            min: Point(self.min.0, self.min.1, other.min.2),
            max: Point(self.max.0, other.min.1 - 1, other.max.2),
        });
        // x right
        regions.push(Region {
            min: Point(self.min.0, other.max.1 + 1, other.min.2),
            max: Point(self.max.0, self.max.1, other.max.2),
        });
        // y left
        regions.push(Region {
            min: Point(other.max.0 + 1, other.min.1, other.min.2),
            max: Point(self.max.0, other.max.1, other.max.2),
        });
        // y right
        regions.push(Region {
            min: Point(self.min.0, other.min.1, other.min.2),
            max: Point(other.min.0 - 1, other.max.1, other.max.2),
        });

        println!("{:?}", regions);

        regions.into_iter().filter(|r| r.size() > 0).collect()
    }

    fn size(&self) -> usize {
        if self.min < self.max {
            let disp = self.max - self.min;
            (disp.0 * disp.1 * disp.2).abs() as usize
        } else {
            0
        }
    }
}

fn parse_input(input: String) -> Vec<ReactorInstruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: &Vec<ReactorInstruction>) -> usize {
    let mut on_cubes = HashSet::new();
    input
        .iter()
        .filter(|i| i.is_within_init_region())
        .for_each(|i| {
            if i.state {
                i.region.points().into_iter().for_each(|p| {
                    on_cubes.insert(p);
                });
            } else {
                i.region.points().iter().for_each(|p| {
                    on_cubes.remove(p);
                });
            }
        });
    on_cubes.len()
}

fn part_2(input: &Vec<ReactorInstruction>) -> usize {
    let mut on_regions: HashSet<Region> = HashSet::new();
    let mut instruction_queue: VecDeque<ReactorInstruction> = input.clone().into_iter().collect();

    while !instruction_queue.is_empty() {
        println!("instructions: {}", instruction_queue.len());
        let i = instruction_queue.pop_front().unwrap();
        if i.region.size() == 0 {
            println!("too smol");
            continue;
        }
        let total = on_regions.len();
        let mut non_interact_count = 0;
        on_regions.clone().iter().for_each(|r| {
            // If on region contains off region, remove it but add back the difference
            if r.contains(&i.region) && !i.state {
                println!("1");
                on_regions.remove(r);
                r.difference(&i.region).into_iter().for_each(|region| {
                    instruction_queue.push_back(ReactorInstruction {
                        state: true,
                        region,
                    });
                });
            }
            // If off region contains on region, remove it
            else if i.region.contains(&r) {
                println!("2");
                on_regions.remove(r);
                if i.state {
                    instruction_queue.push_back(i);
                }
            }
            // If they intersect
            else if r.intersect(&i.region).size() > 0 {
                let intersection = r.intersect(&i.region);
                if i.state {
                    println!("3");
                    // Add non-intersecting i-part to queue
                    i.region
                        .difference(&intersection)
                        .into_iter()
                        .for_each(|region| {
                            instruction_queue.push_back(ReactorInstruction {
                                state: true,
                                region,
                            });
                        });
                } else {
                    println!("4");
                    // Remove r and add non-intersecting parts back to queue
                    on_regions.remove(r);
                    r.difference(&intersection).into_iter().for_each(|region| {
                        instruction_queue.push_back(ReactorInstruction {
                            state: true,
                            region,
                        });
                    });
                }
            }
            // If i interacts with NONE of the regions, add it if its on
            else {
                println!("5");
                non_interact_count += 1;
            }
        });

        if non_interact_count == total {
            on_regions.insert(i.region);
        }
    }

    println!("{}", on_regions.len());
    println!("{:?}", on_regions);
    on_regions.iter().fold(0, |acc, r| acc + r.size())
}

#[cfg(test)]
mod day_22_tests {
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
    fn intersection_difference() {
        let a = Region {
            min: Point(0, 0, 0),
            max: Point(5, 6, 7),
        };
        let b = Region {
            min: Point(-1, -1, -1),
            max: Point(1, 1, 1),
        };
        let inter_a = a.intersect(&b);
        let inter_b = b.intersect(&a);
        println!("{inter_a}");
        assert_eq!(inter_a, inter_b);
        let difference = b.difference(&inter_a);
        assert_eq!(difference.len(), 6);
        println!("{:?}", difference);
        assert!(difference.into_iter().all(|d| a.intersect(&d).size() == 0));
    }
    #[test]
    fn test_region_size() {
        let region = Region {
            min: Point(0, 0, 0),
            max: Point(5, 6, 7),
        };
        assert_eq!(region.size(), 210);
        let region = Region {
            min: Point(-5, -6, -7),
            max: Point(0, 0, 0),
        };
        assert_eq!(region.size(), 210);
        let region = Region {
            min: Point(-8, -7, -5),
            max: Point(-3, -1, 2),
        };
        assert_eq!(region.size(), 210);
        let region = Region {
            min: Point(-8, 0, -5),
            max: Point(-3, -1, 2),
        };
        assert_eq!(region.size(), 0);
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
