use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::ops::{BitAnd, BitOr, BitXor, Mul, Not};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryNumberList {
    pub nb_bits: usize,
    pub numbers: Vec<BinaryNumber>,
}

impl BinaryNumberList {
    pub fn new(numbers: Vec<BinaryNumber>) -> Self {
        BinaryNumberList {
            nb_bits: numbers.first().unwrap().nb_bits,
            numbers,
        }
    }

    pub fn filter_with_one_at_pos(&self, position: usize) -> BinaryNumberList {
        BinaryNumberList {
            nb_bits: self.nb_bits,
            numbers: self.numbers.iter().filter(|b| b.has_one_at(position)).copied().collect(),
        }
    }

    pub fn filter_with_zero_at_pos(&self, position: usize) -> BinaryNumberList {
        BinaryNumberList {
            nb_bits: self.nb_bits,
            numbers: self.numbers.iter().filter(|b| b.has_zero_at(position)).copied().collect(),
        }
    }

    pub fn most_common_bits(&self) -> BinaryNumber {
        let zero = BinaryNumber::zeros(self.nb_bits);
        (0..self.nb_bits).fold(zero, |result, pos| {
            result | self.most_common_bit_at(pos)
        })
    }

    pub fn least_common_bits(&self) -> BinaryNumber {
        !self.most_common_bits()
    }

    fn most_common_bit_at(&self, position: usize) -> BinaryNumber {
        if self.nb_ones_at(position) > self.nb_zeros_at(position) {
            BinaryNumber::one_at_position(position, self.nb_bits)
        } else {
            BinaryNumber::zeros(self.nb_bits)
        }
    }

    pub fn nb_zeros_at(&self, position: usize) -> usize {
        self.numbers.len() - self.nb_ones_at(position)
    }

    pub fn nb_ones_at(&self, position: usize) -> usize {
        self.numbers.iter().filter(|b| b.has_one_at(position)).count()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BinaryNumber {
    pub nb_bits: usize,
    pub number: u32,
}

impl BinaryNumber {
    fn new(number: u32, nb_bits: usize) -> BinaryNumber {
        BinaryNumber { number, nb_bits }
    }

    pub fn ones(nb_bits: usize) -> BinaryNumber {
        let number = (0..nb_bits).fold(0, |sum, pos| sum + 2_u32.pow(pos as u32));
        BinaryNumber { number, nb_bits }
    }

    pub fn zeros(nb_bits: usize) -> BinaryNumber {
        !Self::ones(nb_bits)
    }

    pub fn one_at_position(position: usize, nb_bits: usize) -> BinaryNumber {
        BinaryNumber { number: 2_u32.pow(position as u32), nb_bits }
    }

    pub fn has_one_at(&self, position: usize) -> bool {
        let mask = Self::one_at_position(position, self.nb_bits);
        self.number & mask.number > 0
    }
    pub fn has_zero_at(&self, position: usize) -> bool {
        !self.has_one_at(position)
    }
}

impl FromStr for BinaryNumber {
    type Err = ParseIntError;

    fn from_str(reading: &str) -> Result<Self, Self::Err> {
        let number = u32::from_str_radix(reading, 2)?;

        Ok(BinaryNumber {
            nb_bits: reading.len(),
            number,
        })
    }
}

impl BitAnd for BinaryNumber {
    type Output = BinaryNumber;
    fn bitand(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.nb_bits, rhs.nb_bits);
        BinaryNumber { nb_bits: self.nb_bits, number: self.number & rhs.number }
    }
}

impl BitOr for BinaryNumber {
    type Output = BinaryNumber;
    fn bitor(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.nb_bits, rhs.nb_bits);
        BinaryNumber { nb_bits: self.nb_bits, number: self.number | rhs.number }
    }
}


impl BitXor for BinaryNumber {
    type Output = BinaryNumber;
    fn bitxor(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.nb_bits, rhs.nb_bits);
        BinaryNumber { nb_bits: self.nb_bits, number: self.number ^ rhs.number }
    }
}

impl Not for BinaryNumber {
    type Output = BinaryNumber;
    fn not(self) -> Self::Output {
        self ^ BinaryNumber::ones(self.nb_bits)
    }
}

impl Mul for BinaryNumber {
    type Output = u32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.number * rhs.number
    }
}


impl Display for BinaryNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0width$b}", self.number, width = self.nb_bits)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_string_to_binary_number() {
        let result = BinaryNumber::from_str("01101").unwrap();
        assert_eq!(result, BinaryNumber::new(0b1101, 5))
    }

    #[test]
    fn can_create_binary_number_with_one_at_position() {
        let result = BinaryNumber::one_at_position(3, 4);
        assert_eq!(result, BinaryNumber::new(0b1000, 4))
    }

    #[test]
    fn can_check_if_binary_number_has_one_at_position() {
        let number = BinaryNumber::new(0b1000, 4);
        assert!(number.has_one_at(3));
        assert!(!number.has_one_at(2));
    }

    #[test]
    fn can_use_operators_with_binary_numbers() {
        let first = BinaryNumber::from_str("1100").unwrap();
        let second = BinaryNumber::from_str("1010").unwrap();

        assert_eq!(first & second, BinaryNumber::from_str("1000").unwrap());
        assert_eq!(first | second, BinaryNumber::from_str("1110").unwrap());
        assert_eq!(first ^ second, BinaryNumber::from_str("0110").unwrap());
        assert_eq!(!first, BinaryNumber::from_str("0011").unwrap());
    }

    #[test]
    fn can_create_most_significant_number() {
        let numbers = BinaryNumberList::new(vec![
            BinaryNumber::from_str("100").unwrap(),
            BinaryNumber::from_str("101").unwrap(),
            BinaryNumber::from_str("011").unwrap(),
        ]);

        let result = numbers.most_common_bits();

        assert_eq!(result, BinaryNumber::from_str("101").unwrap());
    }
}