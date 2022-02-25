use aoc::*;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

const DAY: u8 = 8;

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

fn signals_equal(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.chars().all(|char| b.contains(a))
}

fn signal_sort(signal: String) -> String {
    let mut chars = signal.chars().collect::<Vec<char>>();
    chars.sort();
    chars.iter().collect()
}

struct LogEntry {
    signals: Vec<String>,
    output: Vec<String>,
}

impl LogEntry {
    fn get_1(&self) -> String {
        self.signals
            .iter()
            .find(|signal| signal.len() == 2)
            .unwrap()
            .to_string()
    }

    fn get_4(&self) -> String {
        self.signals
            .iter()
            .find(|signal| signal.len() == 4)
            .unwrap()
            .to_string()
    }

    fn get_7(&self) -> String {
        self.signals
            .iter()
            .find(|signal| signal.len() == 3)
            .unwrap()
            .to_string()
    }

    fn get_8(&self) -> String {
        self.signals
            .iter()
            .find(|signal| signal.len() == 7)
            .unwrap()
            .to_string()
    }

    fn get_6(&self, one: &str) -> String {
        let one = self.get_1();
        self.signals
            .iter()
            .find(|signal| {
                if signal.len() != 6 {
                    return false;
                }
                !one.chars().all(|char| signal.contains(char))
            })
            .unwrap()
            .to_string()
    }

    fn deduce_a(&self, one: &str, seven: &str) -> String {
        one.chars()
            .fold(seven.to_string(), |acc, char| acc.replace(char, ""))
            .to_string()
    }

    fn deduce_c(&self, six: &str) -> String {
        "abcdefg"
            .chars()
            .find(|&char| !six.contains(char))
            .unwrap()
            .to_string()
    }

    fn deduce_f(&self, one: &str, c: &str) -> String {
        one.chars()
            .find(|&char| !c.contains(char))
            .unwrap()
            .to_string()
    }

    fn get_2(&self, f: &str) -> String {
        self.signals
            .iter()
            .find(|signal| {
                if signal.len() != 5 {
                    return false;
                }
                !signal.contains(f)
            })
            .unwrap()
            .to_string()
    }

    fn get_3(&self, c: &str, f: &str) -> String {
        self.signals
            .iter()
            .find(|signal| {
                if signal.len() != 5 {
                    return false;
                }
                signal.contains(c) && signal.contains(f)
            })
            .unwrap()
            .to_string()
    }

    fn get_5(&self, c: &str) -> String {
        self.signals
            .iter()
            .find(|signal| {
                if signal.len() != 5 {
                    return false;
                }
                !signal.contains(c)
            })
            .unwrap()
            .to_string()
    }

    fn deduce_d(&self, two: &str, four: &str, five: &str) -> String {
        "abcdefg"
            .chars()
            .find(|&char| two.contains(char) && four.contains(char) && five.contains(char))
            .unwrap()
            .to_string()
    }

    fn get_0(&self, d: &str) -> String {
        self.signals
            .iter()
            .find(|signal| {
                if signal.len() != 6 {
                    return false;
                }
                !signal.contains(d)
            })
            .unwrap()
            .to_string()
    }

    fn get_9(&self, d: &str, zero: &str) -> String {
        self.signals
            .iter()
            .find(|signal| {
                if signal.len() != 6 {
                    return false;
                }
                signal.contains(d) && !zero.chars().all(|char| signal.contains(char))
            })
            .unwrap()
            .to_string()
    }

    fn get_output(&self) -> usize {
        let one = signal_sort(self.get_1());
        let four = signal_sort(self.get_4());
        let seven = signal_sort(self.get_7());
        let eight = signal_sort(self.get_8());
        let six = signal_sort(self.get_6(&one));

        let a = self.deduce_a(&one, &seven);
        let c = self.deduce_c(&six);
        let f = self.deduce_f(&one, &c);

        let two = signal_sort(self.get_2(&f));
        let three = signal_sort(self.get_3(&c, &f));
        let five = signal_sort(self.get_5(&c));

        let d = self.deduce_d(&two, &four, &five);

        let zero = signal_sort(self.get_0(&d));
        let nine = signal_sort(self.get_9(&d, &six));

        //println!(
        //"{} {} {} {} {} {} {} {} {} {}",
        //zero, one, two, three, four, five, six, seven, eight, nine
        //);

        self.output.iter().enumerate().fold(0, |num, (i, signal)| {
            //println!("{}", signal);
            let place = 10usize.pow(3 - i as u32);
            num + match signal_sort(signal.to_string()) {
                x if x == zero => 0,
                x if x == one => 1 * place,
                x if x == two => 2 * place,
                x if x == three => 3 * place,
                x if x == four => 4 * place,
                x if x == five => 5 * place,
                x if x == six => 6 * place,
                x if x == seven => 7 * place,
                x if x == eight => 8 * place,
                x if x == nine => 9 * place,
                _ => panic!("unknown signal"),
            }
        })
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} | {}",
            self.signals
                .iter()
                .fold(String::new(), |acc, x| format!("{} {}", acc, x)),
            self.output
                .iter()
                .fold(String::new(), |acc, x| format!("{} {}", acc, x))
        )
    }
}

impl FromStr for LogEntry {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" | ");
        Ok(LogEntry {
            signals: parts.next().unwrap().split(' ').map(String::from).collect(),
            output: parts.next().unwrap().split(' ').map(String::from).collect(),
        })
    }
}

fn parse_input(input: String) -> Vec<LogEntry> {
    input
        .lines()
        .map(|line| line.parse::<LogEntry>().unwrap())
        .collect()
}

fn part_1(input: &Vec<LogEntry>) -> usize {
    input.iter().fold(0, |acc, entry| {
        acc + entry
            .output
            .iter()
            .filter(|x| match x.len() {
                2 | 4 | 3 | 7 => true,
                _ => false,
            })
            .count()
    })
}

fn part_2(input: &Vec<LogEntry>) -> usize {
    input.iter().map(|entry| entry.get_output()).sum()
}

#[cfg(test)]
mod day_8_tests {
    use super::*;

    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part_1() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 26);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 349);
    }
    #[test]
    fn test_part_2() {
        let input = String::from(TEST_INPUT);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 61229);
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1070957);
    }
}
