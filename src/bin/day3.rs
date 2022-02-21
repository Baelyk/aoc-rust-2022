use aoc::*;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

fn main() {
    println!("==== Day 3 ====");
    let input = get_input(3);
    println!("Input size: {}", input.len());
    let parsed_input = parse_input(input);
    let part_1 = part_1(&parsed_input);
    println!("Part 1: {}", part_1);
    let part_2 = part_2(&parsed_input);
    println!("Part 2: {}", part_2);
}

const BIT_STRING_LENGTH: usize = 12;
#[derive(Clone, Copy, Debug, PartialEq)]
struct BitString {
    bits: [bool; BIT_STRING_LENGTH],
}
impl fmt::Display for BitString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.bits.iter().fold(String::new(), |acc, &bit| {
                format!("{}{}", if bit { "1" } else { "0" }, acc)
            })
        )
    }
}

impl BitString {
    fn new() -> Self {
        Self {
            bits: [false; BIT_STRING_LENGTH],
        }
    }

    fn usize(self) -> usize {
        usize::from(self)
    }
}

#[derive(Debug)]
struct BitStringFromDecimalError {}
impl fmt::Display for BitStringFromDecimalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to create BitString from decimal number")
    }
}
impl TryFrom<usize> for BitString {
    type Error = BitStringFromDecimalError;
    fn try_from(decimal: usize) -> Result<Self, Self::Error> {
        if decimal > 2usize.pow((BIT_STRING_LENGTH - 1) as u32) - 1 {
            Err(BitStringFromDecimalError {})
        } else {
            Ok((0..=BIT_STRING_LENGTH)
                .rev()
                .fold((BitString::new(), decimal), |acc, n| {
                    if acc.1 >= 2usize.pow(n as u32) {
                        let mut bits = acc.0;
                        bits[n] = true;
                        (bits, acc.1 - 2usize.pow(n as u32))
                    } else {
                        acc
                    }
                })
                .0)
        }
    }
}

impl From<BitString> for usize {
    fn from(bits: BitString) -> usize {
        bits.bits.iter().enumerate().fold(0, |acc, (n, &bit)| {
            if bit {
                acc + 2usize.pow(n.try_into().unwrap())
            } else {
                acc
            }
        })
    }
}

impl Index<usize> for BitString {
    type Output = bool;

    fn index(&self, bit: usize) -> &Self::Output {
        &self.bits[bit]
    }
}

impl IndexMut<usize> for BitString {
    fn index_mut(&mut self, bit: usize) -> &mut Self::Output {
        &mut self.bits[bit]
    }
}

impl FromStr for BitString {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.chars()
            .rev()
            .enumerate()
            .fold(BitString::new(), |acc, (i, d)| {
                let mut new_acc = acc;
                new_acc[i] = d == '1';
                new_acc
            }))
    }
}

fn parse_input(input: String) -> Vec<BitString> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn gamma_rate(input: &Vec<BitString>) -> BitString {
    // (0s, 1s)
    let mut most_common = [(0, 0); BIT_STRING_LENGTH];
    input.iter().for_each(|bits| {
        bits.bits.iter().enumerate().for_each(|(i, &bit)| {
            if bit {
                most_common[i].1 += 1
            } else {
                most_common[i].0 += 1
            }
        })
    });
    most_common
        .iter()
        .enumerate()
        .fold(BitString::new(), |acc, (i, (zeros, ones))| {
            let mut new_acc = acc;
            if ones > zeros {
                new_acc[i] = true;
            }
            new_acc
        })
}

fn part_1(input: &Vec<BitString>) -> usize {
    let gamma = gamma_rate(input).usize();
    let epsilon = 2usize.pow(BIT_STRING_LENGTH as u32) - 1 - gamma;
    return gamma * epsilon;
}

fn most_common_nth_digit(input: &Vec<BitString>, n: usize) -> bool {
    let all = input.len();
    let mut ones = 0;
    input.iter().for_each(|bits| {
        if bits[n] {
            ones += 1;
        }
    });
    // Prefer ones
    return ones >= all - ones;
}

fn get_diagnostic_rating(input: &Vec<BitString>, most: bool) -> BitString {
    (0..12).rev().fold(input.clone(), |acc, n| {
        if acc.len() == 1 {
            return acc;
        } else if acc.len() <= 12 && n < 5 {
        }
        let digit = most_common_nth_digit(&acc, n);
        acc.iter()
            .filter(|bits| if bits[n] == digit { most } else { !most })
            .map(|bits| *bits)
            .collect()
    })[0]
}

fn part_2(input: &Vec<BitString>) -> usize {
    let o2 = get_diagnostic_rating(input, true);
    let co2 = get_diagnostic_rating(input, false);
    o2.usize() * co2.usize()
}

#[cfg(test)]
mod day_3_tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        let parsed = parse_input(input);
        assert_eq!(
            parsed
                .iter()
                .map(|bits| bits.usize())
                .collect::<Vec<usize>>(),
            vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10]
        );
    }
    #[test]
    fn test_part_1_gamma() {
        let input = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        let parsed = parse_input(input);
        let gamma = gamma_rate(&parsed);
        assert_eq!(gamma.usize(), 22);
    }
    #[test]
    fn test_part_1_epsilon() {
        let input = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        let parsed = parse_input(input);
        let gamma = gamma_rate(&parsed);
        // I have to manually calculate epsilon since my program assumes length 12
        let epsilon = 2usize.pow(5u32) - 1 - gamma.usize();
        assert_eq!(epsilon, 9);
    }
    #[test]
    fn test_part_1() {
        let input = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        let parsed = parse_input(input);
        let gamma = gamma_rate(&parsed).usize();
        // I have to manually calculate epsilon since my program assumes length 12
        let epsilon = 2usize.pow(5u32) - 1 - gamma;
        assert_eq!(gamma * epsilon, 198);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(3);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 1082324);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        let parsed = parse_input(input);

        // The shorter bit strings rear their ugly head here again, manually change to len 5
        fn get_diagnostic_rating(input: &Vec<BitString>, most: bool) -> BitString {
            (0..5).rev().fold(input.clone(), |acc, n| {
                if acc.len() == 1 {
                    return acc;
                } else if acc.len() <= 12 && n < 5 {
                }
                let digit = most_common_nth_digit(&acc, n);
                acc.iter()
                    .filter(|bits| if bits[n] == digit { most } else { !most })
                    .map(|bits| *bits)
                    .collect()
            })[0]
        }

        let o2 = get_diagnostic_rating(&parsed, true);
        let co2 = get_diagnostic_rating(&parsed, false);

        assert_eq!(o2.usize() * co2.usize(), 230);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(3);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1353024);
    }
}
