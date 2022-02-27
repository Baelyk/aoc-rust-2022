use aoc::*;
use std::collections::LinkedList;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

const DAY: u8 = 10;

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

#[derive(Copy, Clone, PartialEq)]
enum DelimType {
    Parenthesis,
    Bracket,
    Brace,
    Angle,
}

struct Delim {
    delim: DelimType,
    open: bool,
}

impl Delim {
    fn checker_score(&self) -> usize {
        match self.delim {
            DelimType::Parenthesis => 3,
            DelimType::Bracket => 57,
            DelimType::Brace => 1197,
            DelimType::Angle => 25137,
        }
    }
    fn autocomplete_score(&self) -> usize {
        match self.delim {
            DelimType::Parenthesis => 1,
            DelimType::Bracket => 2,
            DelimType::Brace => 3,
            DelimType::Angle => 4,
        }
    }
}
impl fmt::Display for Delim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self.delim {
            DelimType::Parenthesis => {
                if self.open {
                    "("
                } else {
                    ")"
                }
            }
            DelimType::Bracket => {
                if self.open {
                    "["
                } else {
                    "]"
                }
            }
            DelimType::Brace => {
                if self.open {
                    "{"
                } else {
                    "}"
                }
            }
            DelimType::Angle => {
                if self.open {
                    "<"
                } else {
                    ">"
                }
            }
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct UnknownDelimiter {
    c: char,
}
impl Error for UnknownDelimiter {}
impl fmt::Display for UnknownDelimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown delimiter: {}", self.c)
    }
}
impl TryFrom<char> for Delim {
    type Error = UnknownDelimiter;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Delim {
                delim: DelimType::Parenthesis,
                open: true,
            }),
            ')' => Ok(Delim {
                delim: DelimType::Parenthesis,
                open: false,
            }),
            '[' => Ok(Delim {
                delim: DelimType::Bracket,
                open: true,
            }),
            ']' => Ok(Delim {
                delim: DelimType::Bracket,
                open: false,
            }),
            '{' => Ok(Delim {
                delim: DelimType::Brace,
                open: true,
            }),
            '}' => Ok(Delim {
                delim: DelimType::Brace,
                open: false,
            }),
            '<' => Ok(Delim {
                delim: DelimType::Angle,
                open: true,
            }),
            '>' => Ok(Delim {
                delim: DelimType::Angle,
                open: false,
            }),
            _ => Err(UnknownDelimiter { c }),
        }
    }
}

enum LintResult {
    Ok,
    Incomplete(Vec<Delim>),
    Corrupted(Delim),
}

fn lint(line: &String) -> LintResult {
    let mut scope = LinkedList::new();
    for c in line.chars() {
        let delim = Delim::try_from(c).unwrap();
        if delim.open {
            scope.push_front(delim);
        } else {
            if delim.delim != scope.pop_front().unwrap().delim {
                return LintResult::Corrupted(delim);
            }
        }
    }
    if !scope.is_empty() {
        let completion = scope
            .iter()
            .map(|delim| Delim {
                delim: delim.delim,
                open: false,
            })
            .collect();
        return LintResult::Incomplete(completion);
    }
    return LintResult::Ok;
}

fn parse_input(input: String) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn part_1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| match lint(line) {
            LintResult::Corrupted(delim) => delim.checker_score(),
            _ => 0,
        })
        .sum()
}

fn part_2(input: &Vec<String>) -> usize {
    let mut scores: Vec<usize> = input
        .iter()
        .map(|line| match lint(line) {
            LintResult::Incomplete(delims) => delims
                .iter()
                .fold(0, |score, delim| score * 5 + delim.autocomplete_score()),
            _ => 0,
        })
        .filter(|&score| score > 0)
        .collect();
    scores.sort();
    let middle_index = scores.len().div_euclid(2);
    scores[middle_index]
}

#[cfg(test)]
mod day_10_tests {
    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 26397);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 436497);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 288957);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 2377613374);
    }
}
