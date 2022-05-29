use aoc::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

const DAY: u8 = 21;

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

#[derive(Clone, PartialEq, Eq, Hash)]
struct Game {
    player_one: (usize, usize),
    player_two: (usize, usize),
    rolls: usize,
    die: usize,
    one_next: bool,
}
impl FromStr for Game {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Game {
            player_one: (
                lines
                    .next()
                    .unwrap()
                    .chars()
                    .last()
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize,
                0,
            ),
            player_two: (
                lines
                    .next()
                    .unwrap()
                    .chars()
                    .last()
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize,
                0,
            ),
            rolls: 0,
            die: 0,
            one_next: true,
        })
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Player 1 at {} with {}\nPlayer 2 at {} with {}\nRolls: {}\nOne next: {}",
            self.player_one.0,
            self.player_one.1,
            self.player_two.0,
            self.player_two.1,
            self.rolls,
            self.one_next
        )
    }
}
impl Game {
    fn det_roll(&mut self) -> usize {
        self.rolls += 1;
        if self.rolls % 100 == 0 {
            100
        } else {
            self.rolls % 100
        }
    }
    fn det_turn(&mut self) -> bool {
        let moves = self.det_roll() + self.det_roll() + self.det_roll();
        let mut player = if self.one_next {
            &mut self.player_one
        } else {
            &mut self.player_two
        };
        self.one_next = !self.one_next;

        let mut position = (player.0 + moves) % 10;
        if position == 0 {
            position = 10;
        }
        player.0 = position;
        player.1 += position;

        player.1 >= 1000
    }

    fn quantum_roll(&self) -> Vec<(Game, usize)> {
        let mut games = vec![];
        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
            .iter()
            .for_each(|(r, c)| {
                let mut game = self.clone();
                game.die = *r;
                games.push((game, *c));
            });
        games
    }

    fn quantum_turn(&mut self) -> Vec<(Game, usize)> {
        let mut player = if self.one_next {
            &mut self.player_one
        } else {
            &mut self.player_two
        };
        self.one_next = !self.one_next;

        let mut position = (player.0 + self.die) % 10;
        if position == 0 {
            position = 10;
        }
        player.0 = position;
        player.1 += position;

        if player.1 >= 21 {
            return Vec::new();
        } else {
            return self.quantum_roll();
        }
    }
}

fn parse_input(input: String) -> Game {
    input.parse().unwrap()
}

fn part_1(input: &Game) -> usize {
    let mut game = input.clone();
    //println!("{game}");
    while !game.det_turn() {
        //println!("\n{game}");
    }
    //println!("{game}");
    game.rolls * std::cmp::min(game.player_one.1, game.player_two.1)
}

fn part_2(input: &Game) -> usize {
    let mut wins = (0, 0);
    let mut game_queue: HashMap<Game, usize> = HashMap::new();
    input.quantum_roll().into_iter().for_each(|(g, c)| {
        game_queue.insert(g, c);
    });

    while !game_queue.is_empty() {
        game_queue.clone().iter_mut().for_each(|(game, count)| {
            game_queue.remove(game);
            let mut game = game.clone();
            let games = game.quantum_turn();
            if games.is_empty() {
                if game.player_one.1 >= 21 {
                    wins.0 += *count;
                } else {
                    wins.1 += *count;
                }
            } else {
                games.into_iter().for_each(|(game, new_count)| {
                    game_queue
                        .entry(game)
                        .and_modify(|c| *c += new_count * *count)
                        .or_insert(new_count * *count);
                });
            }
        });
    }

    std::cmp::max(wins.0, wins.1)
}

#[cfg(test)]
mod day_21_tests {
    use super::*;

    const TEST_INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 739785);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 752745);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 444356092776315);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 309196008717909);
    }
}
