use aoc::grid::Grid;
use aoc::*;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

const DAY: u8 = 20;

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
enum Pixel {
    Light,
    Dark,
}
impl FromStr for Pixel {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Light),
            "." => Ok(Self::Dark),
            _ => Err("Unknown char"),
        }
    }
}
impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Light => "#",
                Self::Dark => ".",
            }
        )
    }
}

#[derive(Clone)]
struct Image {
    grid: Grid<Pixel>,
    background: Pixel,
}
impl Image {
    fn surrounding(&self, index: usize) -> Vec<Pixel> {
        let mut surrounding = Vec::new();
        let width = self.grid.width();
        let right = index % width < width - 1;
        let up = index >= width;
        let left = index % width != 0;
        let down = index + width < self.grid.size();

        if up && left {
            surrounding.push(self.grid.get_index(index - 1 - width));
        } else {
            surrounding.push(self.background);
        }
        if up {
            surrounding.push(self.grid.get_index(index - width));
        } else {
            surrounding.push(self.background);
        }
        if up && right {
            surrounding.push(self.grid.get_index(index + 1 - width));
        } else {
            surrounding.push(self.background);
        }
        if left {
            surrounding.push(self.grid.get_index(index - 1));
        } else {
            surrounding.push(self.background);
        }
        surrounding.push(self.grid.get_index(index));
        if right {
            surrounding.push(self.grid.get_index(index + 1));
        } else {
            surrounding.push(self.background);
        }
        if down && left {
            surrounding.push(self.grid.get_index(index - 1 + width));
        } else {
            surrounding.push(self.background);
        }
        if down {
            surrounding.push(self.grid.get_index(index + width));
        } else {
            surrounding.push(self.background);
        }
        if right && down {
            surrounding.push(self.grid.get_index(index + 1 + width));
        } else {
            surrounding.push(self.background);
        }

        surrounding
    }

    fn expand(&mut self) {
        let height = self.grid.height();
        let width = self.grid.width();
        let size = self.grid.size();
        let mut elements = self.grid.elements().clone();
        for i in (0..height).rev() {
            elements.insert(i * width, self.background);
            elements.insert(i * width, self.background);
        }
        elements.push(self.background);
        let elements = [
            self.background
                .to_string()
                .repeat(width + 1)
                .split("")
                .filter(|s| s.len() == 1)
                .map(|s| s.parse().unwrap())
                .collect(),
            elements,
            self.background
                .to_string()
                .repeat(width + 2)
                .split("")
                .filter(|s| s.len() == 1)
                .map(|s| s.parse().unwrap())
                .collect(),
        ]
        .concat();
        self.grid = Grid::new(elements, width + 2);
    }

    fn enhance(&mut self, algorithm: &Vec<Pixel>) {
        self.expand();
        let size = self.grid.size();
        let mut elements: Vec<Pixel> = Vec::with_capacity(size);
        for i in 0..size {
            let surrounding = self.surrounding(i);
            let index = pixels_to_num(&surrounding);
            let enhanced = algorithm[index];
            elements.push(enhanced);
        }
        self.grid = Grid::new(elements, self.grid.width());
        self.background = match self.background {
            Pixel::Light => algorithm[511],
            Pixel::Dark => algorithm[0],
        }
    }

    fn count_lit(&self) -> usize {
        self.grid.elements().iter().fold(0, |acc, p| {
            acc + match p {
                Pixel::Light => 1,
                Pixel::Dark => 0,
            }
        })
    }
}
impl FromStr for Image {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<Pixel> = s.parse().unwrap();
        let width = grid.width();
        Ok(Image {
            grid,
            background: Pixel::Dark,
        })
    }
}
impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

fn pixels_to_num(vec: &Vec<Pixel>) -> usize {
    let bits = vec.iter().fold(String::new(), |acc, x| {
        format!(
            "{}{}",
            acc,
            match x {
                Pixel::Light => 1,
                Pixel::Dark => 0,
            }
        )
    });
    usize::from_str_radix(&bits, 2).unwrap()
}

fn parse_input(input: String) -> (Vec<Pixel>, Image) {
    let mut parts = input.split("\n\n");
    let algorithm = parts
        .next()
        .unwrap()
        .trim()
        .split("")
        .filter(|s| s.len() == 1)
        .map(|s| s.parse().unwrap())
        .collect();
    let image = parts.next().unwrap().parse().unwrap();
    return (algorithm, image);
}

fn part_1(input: &(Vec<Pixel>, Image)) -> usize {
    let algorithm = &input.0;
    let mut image = input.1.clone();
    image.enhance(algorithm);
    image.enhance(algorithm);
    image.count_lit()
}

fn part_2(input: &(Vec<Pixel>, Image)) -> usize {
    let algorithm = &input.0;
    let mut image = input.1.clone();
    for _ in 0..50 {
        image.enhance(algorithm);
    }
    image.count_lit()
}

#[cfg(test)]
mod day_20_tests {
    use super::*;

    const TEST_INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 35);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 5268);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 3351);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 16875);
    }
}
