use aoc::*;
use std::error::Error;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

const DAY: u8 = 4;

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

#[derive(Clone, Copy, Debug, PartialEq)]
struct BingoBoard {
    board: [(usize, bool); 25],
}

impl BingoBoard {
    fn new() -> Self {
        Self {
            board: [(0, false); 25],
        }
    }

    fn row_of(&self, entry: usize) -> [(usize, bool); 5] {
        let row_start = 5 * entry.div_euclid(5);
        (0..5).fold([(0, false); 5], |acc, i| {
            let mut new_acc = acc;
            new_acc[i] = self.board[row_start + i];
            new_acc
        })
    }

    fn col_of(&self, entry: usize) -> [(usize, bool); 5] {
        let col_start = entry % 5;
        (0..5).fold([(0, false); 5], |acc, i| {
            let mut new_acc = acc;
            new_acc[i] = self.board[col_start + 5 * i];
            new_acc
        })
    }

    fn sum_unmarked(&self) -> usize {
        self.board
            .iter()
            .filter(|(_, marked)| !*marked)
            .map(|(num, _)| *num)
            .sum()
    }

    fn mark(&mut self, num_to_mark: usize) -> bool {
        match self
            .board
            .iter_mut()
            .enumerate()
            .find(|(_, (num, _))| *num == num_to_mark)
        {
            Some((i, (_, marked))) => {
                *marked = true;
                self.check_for_bingo(i)
            }
            _ => false,
        }
    }

    fn check_for_bingo(&self, entry: usize) -> bool {
        self.row_of(entry)
            .iter()
            .map(|(_, marked)| marked)
            .all(|marked| *marked)
            || self
                .col_of(entry)
                .iter()
                .map(|(_, marked)| marked)
                .all(|marked| *marked)
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.board.chunks(5).fold(String::new(), |acc, row| {
                format!(
                    "{}{}\n",
                    acc,
                    row.iter().fold(String::new(), |acc, (num, marked)| {
                        let marked = if *marked { "X" } else { " " };
                        format!("{} {:1}{:>2}", acc, marked, num)
                    })
                )
            })
        )
    }
}

impl Index<usize> for BingoBoard {
    type Output = usize;

    fn index(&self, entry: usize) -> &Self::Output {
        &self.board[entry].0
    }
}

impl IndexMut<usize> for BingoBoard {
    fn index_mut(&mut self, entry: usize) -> &mut Self::Output {
        &mut self.board[entry].0
    }
}

impl FromStr for BingoBoard {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_ascii_whitespace()
            .filter(|num| !num.is_empty())
            .enumerate()
            .fold(BingoBoard::new(), |acc, (i, d)| {
                let mut new_acc = acc;
                new_acc[i] = d.parse().unwrap();
                new_acc
            }))
    }
}

fn parse_input(input: String) -> (Vec<usize>, Vec<BingoBoard>) {
    let mut parts = input.split("\n\n");
    let numbers = parts
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    let boards = parts
        .map(|board| board.parse::<BingoBoard>().unwrap())
        .collect();
    (numbers, boards)
}

fn part_1((numbers, boards): &(Vec<usize>, Vec<BingoBoard>)) -> usize {
    let mut boards = boards.clone();
    numbers
        .iter()
        .find_map(|&num_to_mark| {
            boards.iter_mut().find_map(|board| {
                if board.mark(num_to_mark) {
                    Some(board.sum_unmarked() * num_to_mark)
                } else {
                    None
                }
            })
        })
        .unwrap()
}

fn part_2((numbers, bards): &(Vec<usize>, Vec<BingoBoard>)) -> usize {
    // TODO: Surely there is a better way to do this
    let mut boards = bards.clone();
    numbers
        .iter()
        .find_map(|&num_to_mark| {
            if boards.len() == 1 {
                let mut board = boards[0];
                if board.mark(num_to_mark) {
                    return Some(board.sum_unmarked() * num_to_mark);
                }
            } else {
                boards = boards
                    .iter_mut()
                    .filter_map(|board| {
                        if !(*board).mark(num_to_mark) {
                            Some(board)
                        } else {
                            None
                        }
                    })
                    .map(|board| *board)
                    .collect();
            }
            None
        })
        .unwrap()
}

#[cfg(test)]
mod day_4_tests {
    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_row_col_of() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        let board = parsed.1[1];
        let row = board.row_of(7).map(|(num, _)| num);
        let col = board.col_of(7).map(|(num, _)| num);
        assert_eq!(row, [9, 18, 13, 17, 5]);
        assert_eq!(col, [0, 13, 7, 10, 16]);
    }
    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        println!(
            "{:?}\n{}\n{}\n{}",
            parsed.0, parsed.1[0], parsed.1[1], parsed.1[2]
        );
        assert_eq!(part_1(&parsed), 4512);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 67716);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1924);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1830);
    }
}
