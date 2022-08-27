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
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
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

    fn size(&self) -> isize {
        if self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z {
            0
        } else {
            (1 + self.max.x - self.min.x)
                * (1 + self.max.y - self.min.y)
                * (1 + self.max.z - self.min.z)
        }
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

fn count_on(cuboids: &Vec<(Cuboid, isize)>) -> usize {
    cuboids
        .iter()
        .fold(0isize, |count, (cuboid, sign)| count + sign * cuboid.size())
        .try_into()
        .unwrap()
}

fn process_steps(steps: &Vec<RebootStep>) -> usize {
    let mut cuboids: Vec<(Cuboid, isize)> = vec![];

    steps.iter().for_each(|step| {
        let mut additions = vec![];
        if step.state {
            additions.push((step.cuboid, 1));
        }
        cuboids.iter().for_each(|(cuboid, cuboid_sign)| {
            let intersection = step.cuboid.intersection(cuboid);

            if intersection.size() > 0 {
                additions.push((intersection, -cuboid_sign));
            }
        });
        cuboids.append(&mut additions);
        //println!("After {}, we have {} on:", step, count_on(&cuboids));
        //cuboids
        //.iter()
        //.for_each(|(cuboid, sign)| println!("{} * {}", sign, cuboid));
    });

    count_on(&cuboids)
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
    fn test_cuboid_size() {
        assert_eq!(Cuboid::new(0, 0, 0, 5, 5, 5).size(), 6 * 6 * 6);
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
    fn test_part_1_big() {
        let input = String::from(
            "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
        );
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 474140);
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
        let input = String::from(
            "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
        );
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 2758514936282235);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1304385553084863);
    }
}
