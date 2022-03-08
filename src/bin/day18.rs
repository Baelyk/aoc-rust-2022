use aoc::*;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::ops::Add;
use std::str::FromStr;

const DAY: u8 = 18;

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

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq)]
enum Digit {
    Leaf(usize),
    Branch(Box<Number>),
}

impl Digit {
    fn new_branch(left: usize, right: usize) -> Self {
        Self::Branch(Box::new(Number {
            left: Digit::Leaf(left),
            right: Digit::Leaf(right),
        }))
    }

    fn try_explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        match self {
            Self::Branch(num) => {
                if depth == 4 {
                    if let (Digit::Leaf(left), Digit::Leaf(right)) =
                        (num.left.clone(), num.right.clone())
                    {
                        *self = Digit::Leaf(0);
                        return Some((left, right));
                    }
                }
                num.try_explode(depth)
            }
            _ => None,
        }
    }

    fn try_split(&mut self) -> bool {
        match self {
            Digit::Branch(num) => num.try_split(),
            Digit::Leaf(digit) => {
                if *digit >= 10 {
                    *self = Digit::new_branch(*digit / 2, (*digit + 1) / 2);
                    return true;
                }
                false
            }
        }
    }

    fn add_to(&mut self, dir: Direction, x: usize) -> bool {
        match self {
            Digit::Leaf(leaf) => {
                *leaf += x;
                true
            }
            Digit::Branch(branch) => match dir {
                Direction::Left => branch.left.add_to(dir, x),
                Direction::Right => branch.right.add_to(dir, x),
            },
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Digit::Leaf(digit) => *digit,
            Digit::Branch(branch) => branch.left.magnitude() * 3 + branch.right.magnitude() * 2,
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Digit::Leaf(num) => write!(f, "{}", num),
            Digit::Branch(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Number {
    left: Digit,
    right: Digit,
}

impl Number {
    fn try_explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        if let Some((left, right)) = self.left.try_explode(depth + 1) {
            if right != 0 {
                if self.right.add_to(Direction::Left, right) {
                    return Some((left, 0));
                }
            }
            return Some((left, right));
        } else if let Some((left, right)) = self.right.try_explode(depth + 1) {
            if left != 0 {
                if self.left.add_to(Direction::Right, left) {
                    return Some((0, right));
                }
            }
            return Some((left, right));
        }
        None
    }

    fn try_split(&mut self) -> bool {
        self.left.try_split() || self.right.try_split()
    }

    fn reduce(&mut self) {
        loop {
            if self.try_explode(0).is_some() {
                continue;
            }

            if self.try_split() {
                continue;
            }

            break;
        }
    }

    fn magnitude(&self) -> usize {
        Digit::Branch(Box::new(self.clone())).magnitude()
    }

    fn sum(numbers: &Vec<Number>) -> Number {
        let first = numbers[0].clone();
        numbers.iter().skip(1).cloned().fold(first, |sum, x| {
            let mut sum = sum + x;
            sum.reduce();
            sum
        })
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            left: Digit::Branch(Box::new(self)),
            right: Digit::Branch(Box::new(other)),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl FromStr for Number {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: VecDeque<char> = s.chars().collect();
        Ok(Number::from(chars))
    }
}

impl From<VecDeque<char>> for Number {
    fn from(vec: VecDeque<char>) -> Self {
        fn find_splitpoint(vec: &VecDeque<char>) -> usize {
            let mut i = 0;
            let mut opens = 0;
            let mut closes = 0;
            for c in vec {
                match c {
                    '[' => opens += 1,
                    ']' => closes += 1,
                    _ => {}
                };
                i += 1;
                if opens == closes {
                    break;
                }
            }
            i
        }

        let mut vec = vec.clone();
        // Remove starting and ending brackets
        vec.pop_front();
        vec.pop_back();

        let left = match vec[0] {
            '[' => {
                let i = find_splitpoint(&vec);
                let tmp = vec.split_off(i);
                let to_parse = vec;
                vec = tmp;
                Digit::Branch(Box::new(Number::from(to_parse)))
            }
            '0'..='9' => Digit::Leaf(vec.pop_front().unwrap().to_digit(10).unwrap() as usize),
            _ => panic!("Unexpected char: {}", vec[0]),
        };

        vec.pop_front();

        let right = match vec[0] {
            '[' => {
                let i = find_splitpoint(&vec);
                vec.truncate(i);
                let to_parse = vec;
                Digit::Branch(Box::new(Number::from(to_parse)))
            }
            '0'..='9' => Digit::Leaf(vec.pop_front().unwrap().to_digit(10).unwrap() as usize),
            _ => panic!("Unexpected char: {}", vec[0]),
        };

        Number { left, right }
    }
}

fn parse_input(input: String) -> Vec<Number> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part_1(input: &Vec<Number>) -> usize {
    Number::sum(input).magnitude()
}

fn part_2(input: &Vec<Number>) -> usize {
    let mut magnitude = 0;
    for a in input {
        for b in input {
            if a == b {
                continue;
            }
            let mut sum = a.clone() + b.clone();
            sum.reduce();
            magnitude = std::cmp::max(magnitude, sum.magnitude())
        }
    }
    magnitude
}

#[cfg(test)]
mod day_18_tests {
    use super::*;

    const TEST_INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn test_parse() {
        let to_parse = "[[1,2],3]";
        let parsed: Number = to_parse.parse().unwrap();
        assert_eq!(parsed.to_string(), to_parse);
        let to_parse = "[[1,2],3]";
        let parsed: Number = to_parse.parse().unwrap();
        assert_eq!(parsed.to_string(), to_parse);
    }
    #[test]
    fn test_add() {
        let a: Number = "[1,2]".parse().unwrap();
        let b: Number = "[[3,4],5]".parse().unwrap();
        assert_eq!((a + b).to_string(), "[[1,2],[[3,4],5]]");
    }
    #[test]
    fn test_explode() {
        let to_parse = "[[[[[9,8],1],2],3],4]";
        let mut parsed: Number = to_parse.parse().unwrap();
        dbg!(parsed.try_explode(0));
        assert_eq!(parsed.to_string(), "[[[[0,9],2],3],4]");

        let to_parse = "[7,[6,[5,[4,[3,2]]]]]";
        let mut parsed: Number = to_parse.parse().unwrap();
        dbg!(parsed.try_explode(0));
        assert_eq!(parsed.to_string(), "[7,[6,[5,[7,0]]]]");

        let to_parse = "[[6,[5,[4,[3,2]]]],1]";
        let mut parsed: Number = to_parse.parse().unwrap();
        dbg!(parsed.try_explode(0));
        assert_eq!(parsed.to_string(), "[[6,[5,[7,0]]],3]");

        let to_parse = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let mut parsed: Number = to_parse.parse().unwrap();
        dbg!(parsed.try_explode(0));
        assert_eq!(parsed.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }
    #[test]
    fn test_reduced_add() {
        let a: Number = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let b: Number = "[1,1]".parse().unwrap();
        let mut sum = a + b;
        sum.reduce();
        assert_eq!(sum.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }
    #[test]
    fn test_sum() {
        let number: Number = "[9,1]".parse().unwrap();
        let sum = Number::sum(&vec![number]);
        assert_eq!(sum.to_string(), "[9,1]");

        let numbers = "[1,1]
[2,2]
[3,3]
[4,4]";
        let numbers = parse_input(numbers.to_string());
        let sum = Number::sum(&numbers);
        assert_eq!(sum.to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");

        let numbers = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]";
        let numbers = parse_input(numbers.to_string());
        let sum = Number::sum(&numbers);
        assert_eq!(sum.to_string(), "[[[[3,0],[5,3]],[4,4]],[5,5]]");

        let numbers = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        let numbers = parse_input(numbers.to_string());
        let sum = Number::sum(&numbers);
        assert_eq!(
            sum.to_string(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );

        let numbers = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";
        let numbers = parse_input(numbers.to_string());
        let sum = Number::sum(&numbers);
        assert_eq!(sum.to_string(), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }
    #[test]
    fn test_magnitude() {
        let number: Number = "[9,1]".parse().unwrap();
        assert_eq!(number.magnitude(), 29);
        let number: Number = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap();
        assert_eq!(number.magnitude(), 3488);
    }
    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 4140);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 4435);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 3993);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 4802);
    }
}
