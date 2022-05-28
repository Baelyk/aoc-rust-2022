use aoc::*;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::ops::{Add, Index, IndexMut, Mul, Range, Sub};
use std::str::FromStr;

const DAY: u8 = 19;

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

#[derive(Clone, Debug, PartialEq)]
struct Scanner {
    num: usize,
    beacons: Vec<Point>,
}
impl FromStr for Scanner {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first = lines.next().unwrap();
        let num: usize = first
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let beacons = lines.map(|line| line.parse().unwrap()).collect();
        Ok(Scanner { num, beacons })
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<isize> = s.split(',').map(|x| x.parse().unwrap()).collect();
        Ok(Point(parts[0], parts[1], parts[2]))
    }
}

fn resolve_overlap(
    a: &Vec<Point>,
    a_table: &Vec<Vec<isize>>,
    b: &Vec<Point>,
) -> Option<(Point, Point, Point)> {
    // Take a as the correct scanner, and b as the scanner to find
    // First, find the coord and sign of the x coordinate
    let a_x: Vec<isize> = a.iter().map(|beacon| beacon[0]).collect();
    let mut overlapping = vec![];
    let mut x_coord = 0;
    let mut x_sign = 1;
    // i is the coord we are currently assuming is x
    for i in 0..=2 {
        let b_table = distance_table(b, i);
        b_table.iter().enumerate().for_each(|(i, b_dist)| {
            'sign: for sign in [1, -1] {
                // Find rows that share at least 12 distances
                for (j, a_dist) in a_table.iter().enumerate() {
                    let common = b_dist.iter().fold(0, |acc, x| {
                        if a_dist.contains(&(sign * x)) {
                            acc + 1
                        } else {
                            acc
                        }
                    });
                    if common >= 12 {
                        overlapping.push((i, j));
                        x_sign = sign;
                        break 'sign;
                    }
                }
            }
        });
        // If we have 12 we found the x coord, else try again
        if overlapping.len() < 12 {
            overlapping = vec![];
        } else {
            x_coord = i;
            break;
        }
    }

    // These scanners do not overlap with 12 beacons :(
    if overlapping.len() < 12 {
        return None;
    }

    // Now we have the the x coord and the beacons. Lets use this to get the y,z coords
    // TODO: in theory in special cases of the first two beacons this could cause issues
    let a1 = a[overlapping[0].1];
    let b1 = b[overlapping[0].0];
    let a2 = a[overlapping[1].1];
    let b2 = b[overlapping[1].0];
    let a_offset = a1 - a2;
    let b_offset = b1 - b2;
    // Get y
    let (y_coord, y_sign) = if a_offset.1.abs() == b_offset.0.abs() {
        (0, a_offset.1 / b_offset.0)
    } else if a_offset.1.abs() == b_offset.1.abs() {
        (1, a_offset.1 / b_offset.1)
    } else if a_offset.1.abs() == b_offset.2.abs() {
        (2, a_offset.1 / b_offset.2)
    } else {
        return None;
    };
    // Get z (stupid way)
    let (z_coord, z_sign) = if a_offset.2.abs() == b_offset.0.abs() {
        (0, a_offset.2 / b_offset.0)
    } else if a_offset.2.abs() == b_offset.1.abs() {
        (1, a_offset.2 / b_offset.1)
    } else if a_offset.2.abs() == b_offset.2.abs() {
        (2, a_offset.2 / b_offset.2)
    } else {
        return None;
    };
    // Unpermute the coords, so that all remains is the offset if scanner b
    let sign = Point(x_sign, y_sign, z_sign);
    let perm = Point(x_coord as isize, y_coord, z_coord);
    let b1_in_a = convert_coord(&b1, sign, perm, Point(0, 0, 0));

    // The location of scanner b in terms of scanner a
    Some((sign, perm, a1 - b1_in_a))
}

fn convert_coord(coord: &Point, sign: Point, perm: Point, offset: Point) -> Point {
    sign * coord.permute(perm) + offset
}

fn distance_table(beacons: &Vec<Point>, coord: usize) -> Vec<Vec<isize>> {
    let coords: Vec<isize> = beacons.iter().map(|b| b[coord]).collect();
    let len = coords.len();
    // TODO: This calculates the distance a to b AND b to a
    coords
        .iter()
        .map(|x_1| coords.iter().map(|x_2| x_2 - x_1).collect())
        .collect()
}

fn find_scanners_beacons(input: &Vec<Scanner>) -> (Vec<Point>, HashSet<Point>) {
    let mut scanners: Vec<Point> = vec![];
    let mut scanner_queue: VecDeque<Scanner> = input.clone().into();
    let mut found_scanners: VecDeque<(Scanner, Vec<Vec<isize>>)> = VecDeque::new();

    let primary = scanner_queue.pop_front().unwrap();
    let mut all_beacons: HashSet<Point> = primary.beacons.iter().copied().collect();
    let table = distance_table(&primary.beacons, 0);
    found_scanners.push_back((primary, table));

    while !scanner_queue.is_empty() {
        let (found_scanner, found_table) = found_scanners.pop_front().unwrap();

        for _ in 0..scanner_queue.len() {
            let scanner = scanner_queue.pop_front().unwrap();

            let mut readd = true;
            if let Some((sign, perm, offset)) =
                resolve_overlap(&found_scanner.beacons, &found_table, &scanner.beacons)
            {
                let mut beacons = vec![];
                scanner.beacons.iter().for_each(|beacon| {
                    let converted = convert_coord(&beacon, sign, perm, offset);
                    beacons.push(converted);
                    all_beacons.insert(converted);
                });
                let table = distance_table(&beacons, 0);
                found_scanners.push_back((
                    Scanner {
                        num: scanner.num,
                        beacons,
                    },
                    table,
                ));
                scanners.push(offset);
                readd = false;
            }
            if readd {
                scanner_queue.push_back(scanner);
            }
        }
    }

    (scanners, all_beacons)
}

fn manhattan_distance(a: &Point, b: &Point) -> usize {
    let disp = *a - *b;
    (disp.0.abs() + disp.1.abs() + disp.2.abs()) as usize
}

fn parse_input(input: String) -> Vec<Scanner> {
    input.split("\n\n").map(|s| s.parse().unwrap()).collect()
}

fn part_1(input: &Vec<Scanner>) -> usize {
    find_scanners_beacons(input).1.len()
}

fn part_2(input: &Vec<Scanner>) -> usize {
    let scanners = find_scanners_beacons(input).0;
    scanners
        .iter()
        .map(|a| {
            scanners
                .iter()
                .map(|b| manhattan_distance(a, b))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod day_19_tests {
    use super::*;

    const TEST_INPUT: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn test_parse() {
        let input = String::from(
            "--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
-1,-1,0
-5,0,0
-2,1,0",
        );
        let parsed = parse_input(input);
        assert_eq!(
            parsed,
            vec![
                Scanner {
                    num: 0,
                    beacons: vec![Point(0, 2, 0), Point(4, 1, 0), Point(3, 3, 0)]
                },
                Scanner {
                    num: 1,
                    beacons: vec![Point(-1, -1, 0), Point(-5, 0, 0), Point(-2, 1, 0)]
                }
            ]
        );
    }
    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 79);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 381);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 3621);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 12201);
    }
}
