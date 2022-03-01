use aoc::*;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

const DAY: u8 = 13;

fn main() {
    println!("==== Day {} ====", DAY);
    let input = get_input(DAY);
    println!("Input size: {}", input.len());
    let parsed_input = parse_input(input);
    let part_1 = part_1(&parsed_input);
    println!("Part 1: {}", part_1);
    let part_2 = part_2(&parsed_input);
    println!("Part 2: \n{}", part_2);
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Dot {
    x: usize,
    y: usize,
}

impl fmt::Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl FromStr for Dot {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = s
            .split(',')
            .map(|part| part.parse::<usize>().unwrap())
            .collect();
        Ok(Dot {
            x: parts[0],
            y: parts[1],
        })
    }
}

#[derive(Clone, Copy)]
enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

impl FromStr for Fold {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('=').collect();
        let line = parts[1].parse().unwrap();
        Ok(match parts[0].contains('y') {
            true => Fold::Vertical(line),
            false => Fold::Horizontal(line),
        })
    }
}

#[derive(Clone)]
struct TransparentPaper {
    dots: Vec<Dot>,
    folds: Vec<Fold>,
    height: usize,
    width: usize,
}

impl fmt::Display for TransparentPaper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", {
            let mut matrix: Vec<char> = (" ".repeat(self.width) + "\n")
                .repeat(self.height)
                .chars()
                .collect();
            self.dots.iter().for_each(|Dot { x, y }| {
                matrix[x + y * (self.width + 1)] = '#';
            });

            matrix.iter().collect::<String>()
        })
    }
}

impl TransparentPaper {
    fn fold(&mut self) {
        let fold = self.folds.pop();
        if fold.is_none() {
            return;
        }
        let fold = fold.unwrap();
        match fold {
            Fold::Vertical(line) => {
                self.dots.iter_mut().for_each(|dot| {
                    if dot.y > line {
                        dot.y = line - (dot.y - line)
                    }
                });
                self.height = line;
            }
            Fold::Horizontal(line) => {
                self.dots.iter_mut().for_each(|dot| {
                    if dot.x > line {
                        dot.x = line - (dot.x - line)
                    }
                });
                self.width = line;
            }
        }
        let deduped: HashSet<Dot> = self.dots.drain(..).collect();
        self.dots.extend(deduped.iter());
    }
}

fn parse_input(input: String) -> TransparentPaper {
    let mut parts = input.split("\n\n");
    let dots = parts
        .next()
        .unwrap()
        .lines()
        .map(|dot| dot.parse().unwrap())
        .collect();
    let folds: Vec<Fold> = parts
        .next()
        .unwrap()
        .lines()
        .rev()
        .map(|fold| fold.parse().unwrap())
        .collect();
    let height = 1 + 2 * folds
        .iter()
        .find_map(|fold| match fold {
            Fold::Vertical(line) => Some(line),
            Fold::Horizontal(_) => None,
        })
        .unwrap();
    let width = 1 + 2 * folds
        .iter()
        .find_map(|fold| match fold {
            Fold::Vertical(_) => None,
            Fold::Horizontal(line) => Some(line),
        })
        .unwrap();
    TransparentPaper {
        dots,
        folds,
        height,
        width,
    }
}

fn part_1(input: &TransparentPaper) -> usize {
    let mut sheet = input.clone();
    sheet.fold();
    sheet.dots.len()
}

fn part_2(input: &TransparentPaper) -> String {
    let mut sheet = input.clone();
    while !sheet.folds.is_empty() {
        sheet.fold();
    }
    sheet.to_string()
}

#[cfg(test)]
mod day_13_tests {
    use super::*;

    const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 17);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 0);
    }
    #[test]
    fn solution_part_2() {
        // HECRZKPR
        // #  # ####  ##  ###  #### #  # ###  ###
        // #  # #    #  # #  #    # # #  #  # #  #
        // #### ###  #    #  #   #  ##   #  # #  #
        // #  # #    #    ###   #   # #  ###  ###
        // #  # #    #  # # #  #    # #  #    # #
        // #  # ####  ##  #  # #### #  # #    #  #
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed).len(), 246);
    }
}
