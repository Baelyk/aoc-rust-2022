use std::error::Error;
use std::fmt;
use std::ops::{Index, IndexMut, Range};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Bits {
    bits: Vec<bool>,
}

impl fmt::Display for Bits {
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

impl Bits {
    pub fn new() -> Self {
        Self { bits: Vec::new() }
    }

    pub fn from_hex(hex: &str) -> Bits {
        let mut bits = Vec::new();
        hex.trim()
            .chars()
            .rev()
            .map(|c| {
                let mut bits: Bits = c.to_digit(16).unwrap().into();
                bits.pad(4);
                bits
            })
            .for_each(|b: Bits| bits.append(&mut b.bits.clone()));
        let bits = Bits::from(bits);
        bits
    }

    /// Gets Bits where start is the left-most bit wanted and len is the amount of bits
    pub fn get_from(&self, start: usize, len: usize) -> Self {
        let end = self.len() - start - 1;
        let start = 1 + end - len;
        self.bits[start..=end].into()
    }

    pub fn get(&self, index: usize) -> bool {
        let index = self.len() - 1 - index;
        self.bits[index]
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn pad(&mut self, pad_to: usize) {
        let padding = pad_to - self.len();
        if padding > 0 {
            self.bits.append(&mut vec![false; padding]);
        }
    }

    pub fn reverse_new(&self) -> Self {
        let mut bits = self.clone();
        bits.bits = bits.bits.iter().rev().copied().collect();
        bits
    }
}

impl From<Bits> for usize {
    fn from(bits: Bits) -> usize {
        bits.bits.iter().enumerate().fold(0, |acc, (n, &bit)| {
            if bit {
                acc + 2usize.pow(n.try_into().unwrap())
            } else {
                acc
            }
        })
    }
}

impl From<u32> for Bits {
    fn from(num: u32) -> Bits {
        let num = num as usize;
        Bits::from(num)
    }
}

impl From<usize> for Bits {
    fn from(num: usize) -> Bits {
        let mut num = num;
        let mut bits = Vec::new();
        let mut started = false;
        for n in (0..64 as u32).rev() {
            let power = 2usize.pow(n);
            if num >= power {
                bits.push(true);
                num -= power;
                started = true
            } else if started {
                bits.push(false);
            }
        }
        Bits::from(bits.iter().rev().copied().collect::<Vec<bool>>())
    }
}

impl From<Vec<bool>> for Bits {
    fn from(bitvec: Vec<bool>) -> Bits {
        let mut bits = Bits::new();
        bits.bits = bitvec;
        bits
    }
}

impl From<&[bool]> for Bits {
    fn from(bits: &[bool]) -> Bits {
        Bits::from(bits.to_vec())
    }
}

impl Index<usize> for Bits {
    type Output = bool;

    fn index(&self, bit: usize) -> &Self::Output {
        &self.bits[bit]
    }
}

impl IndexMut<usize> for Bits {
    fn index_mut(&mut self, bit: usize) -> &mut Self::Output {
        &mut self.bits[bit]
    }
}

impl FromStr for Bits {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.chars()
            .rev()
            .enumerate()
            .fold(Bits::new(), |acc, (i, d)| {
                let mut new_acc = acc;
                new_acc[i] = d == '1';
                new_acc
            }))
    }
}

impl IntoIterator for Bits {
    type Item = bool;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.bits.into_iter()
    }
}

#[cfg(test)]
mod bits_tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        let bits = Bits::from_hex("D2FE28");
        assert_eq!(bits.to_string(), "110100101111111000101000");
        let bits = Bits::from_hex("38006F45291200");
        assert_eq!(
            bits.to_string(),
            "00111000000000000110111101000101001010010001001000000000"
        );
    }
}
