use aoc::bits::*;
use aoc::*;
use std::error::Error;

const DAY: u8 = 16;

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
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<Bits> for PacketType {
    type Error = &'static str;

    fn try_from(bits: Bits) -> Result<Self, Self::Error> {
        let num = usize::from(bits);
        num.try_into()
    }
}

impl TryFrom<usize> for PacketType {
    type Error = &'static str;

    fn try_from(num: usize) -> Result<Self, Self::Error> {
        match num {
            0 => Ok(PacketType::Sum),
            1 => Ok(PacketType::Product),
            2 => Ok(PacketType::Minimum),
            3 => Ok(PacketType::Maximum),
            4 => Ok(PacketType::Literal),
            5 => Ok(PacketType::GreaterThan),
            6 => Ok(PacketType::LessThan),
            7 => Ok(PacketType::EqualTo),
            _ => Err("Packet type id must be at most 3 binary digits"),
        }
    }
}

struct Packet {
    bits: Bits,
    subpackets: Vec<Packet>,
    typ: PacketType,
}

impl Packet {
    fn version(&self) -> usize {
        self.bits.get_from(0, 3).into()
    }

    fn version_sum(&self) -> usize {
        let version = self.version();
        version
            + self
                .subpackets
                .iter()
                .map(|packet| packet.version_sum())
                .sum::<usize>()
    }

    fn evaluate(&self) -> usize {
        let mut subpackets = self.subpackets.iter().map(|p| p.evaluate());
        match self.typ {
            PacketType::Sum => subpackets.sum(),
            PacketType::Product => subpackets.product(),
            PacketType::Minimum => subpackets.min().unwrap(),
            PacketType::Maximum => subpackets.max().unwrap(),
            PacketType::Literal => Bits::from(
                self.bits
                    .get_from(6, self.bits.len() - 6)
                    .into_iter()
                    .rev()
                    .enumerate()
                    .filter(|(i, _)| i % 5 != 0)
                    .rev()
                    .map(|(_, bit)| bit)
                    .collect::<Vec<bool>>(),
            )
            .into(),
            PacketType::GreaterThan => {
                if subpackets.next().unwrap() > subpackets.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            PacketType::LessThan => {
                if subpackets.next().unwrap() < subpackets.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            PacketType::EqualTo => {
                if subpackets.next().unwrap() == subpackets.next().unwrap() {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn parse(bits: &Bits, cursor: usize) -> (Packet, usize) {
        let mut cursor = cursor;

        let mut subpackets = Vec::new();

        let packet_start = cursor;
        cursor += 3;
        let typ = bits.get_from(cursor, 3);
        let typ = PacketType::try_from(bits.get_from(cursor, 3)).unwrap();
        cursor += 3;
        match typ {
            PacketType::Literal => {
                while bits.get(cursor) {
                    cursor += 5;
                }
                cursor += 5;
            }
            _ => {
                if bits.get(cursor) {
                    // Amount of packets specified
                    cursor += 1;
                    let amount = bits.get_from(cursor, 11);
                    let amount: usize = amount.into();
                    cursor += 11;
                    for _ in 0..amount {
                        let (subpacket, len) = Packet::parse(&bits, cursor);
                        subpackets.push(subpacket);
                        cursor += len;
                    }
                } else {
                    // Length of packets specified
                    cursor += 1;
                    let length: usize = bits.get_from(cursor, 15).into();
                    cursor += 15;
                    let end = cursor + length;
                    while cursor <= end {
                        if end - cursor < 64
                            && usize::from(bits.get_from(cursor, end - cursor)) == 0
                        {
                            break;
                        }
                        let (subpacket, length) = Packet::parse(&bits, cursor);
                        subpackets.push(subpacket);
                        cursor += length;
                    }
                };
            }
        }

        let length = cursor - packet_start;
        let bits = bits.get_from(packet_start, length);
        (
            Packet {
                bits: bits.clone(),
                subpackets,
                typ,
            },
            length,
        )
    }
}

impl From<Bits> for Packet {
    fn from(bits: Bits) -> Self {
        Packet::parse(&bits, 0).0
    }
}

fn parse_input(input: String) -> Packet {
    Packet::from(Bits::from_hex(&input))
}

fn part_1(input: &Packet) -> usize {
    input.version_sum()
}

fn part_2(input: &Packet) -> usize {
    input.evaluate()
}

#[cfg(test)]
mod day_16_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let packet = parse_input("D2FE28".into());
        assert_eq!(packet.version(), 6);
        println!("\n\n");
        let packet = parse_input("38006F45291200".into());
        assert_eq!(packet.version(), 1);
        println!("\n\n");
        let packet = parse_input("EE00D40C823060".into());
        assert_eq!(packet.version(), 7);
    }
    #[test]
    fn test_part_1() {
        let input = String::from("8A004A801A8002F478");
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 16);
        println!("\n\n");
        let input = String::from("620080001611562C8802118E34");
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 12);
        println!("\n\n");
        let input = String::from("C0015000016115A2E0802F182340");
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 23);
        println!("\n\n");
        let input = String::from("A0016C880162017C3686B18A3D4780");
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 31);
    }
    #[test]
    fn solution_part_1() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        assert_eq!(part_1(&parsed), 938);
    }
    #[test]
    fn test_part_2() {
        let input = String::from("D2FE28");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 2021);
        println!("\n\n");
        let input = String::from("C200B40A82");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 3);
        println!("\n\n");
        let input = String::from("04005AC33890");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 54);
        println!("\n\n");
        let input = String::from("880086C3E88112");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 7);
        println!("\n\n");
        let input = String::from("CE00C43D881120");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 9);
        println!("\n\n");
        let input = String::from("D8005AC2A8F0");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1);
        println!("\n\n");
        let input = String::from("F600BC2D8F");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 0);
        println!("\n\n");
        let input = String::from("9C005AC2F8F0");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 0);
        println!("\n\n");
        let input = String::from("9C0141080250320F1802104A08");
        let parsed = parse_input(input);
        assert_eq!(part_2(&parsed), 1);
        println!("\n\n");
    }
    #[test]
    fn solution_part_2() {
        let input = get_input(DAY);
        let parsed = parse_input(input);
        println!("Packet type {:?}", parsed.typ);
        assert_eq!(part_2(&parsed), 1495959086337);
    }
}
