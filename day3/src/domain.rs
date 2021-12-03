use std::fmt::Debug;
use std::num::ParseIntError;
use std::ops::Mul;
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

    pub fn retain<F>(&mut self, predicate: F) where F: FnMut(&BinaryNumber) -> bool {
        self.numbers.retain(predicate);
    }

    pub fn most_common_bits(&self) -> BinaryNumber {
        let positions: Vec<usize> = (0..self.nb_bits)
            .filter(|position| self.nb_ones_at(*position) > self.nb_zeros_at(*position))
            .collect();
        BinaryNumber::ones_at(&positions, self.nb_bits)
    }

    pub fn least_common_bits(&self) -> BinaryNumber {
        let positions: Vec<usize> = (0..self.nb_bits)
            .filter(|position| self.nb_ones_at(*position) < self.nb_zeros_at(*position))
            .collect();
        BinaryNumber::ones_at(&positions, self.nb_bits)
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

    pub fn ones_at(positions: &[usize], nb_bits: usize) -> BinaryNumber {
        let number = positions.iter().map(|&position| 2_u32.pow(position as u32)).sum();
        BinaryNumber { number, nb_bits }
    }

    pub fn has_one_at(&self, position: usize) -> bool {
        let mask = Self::ones_at(&[position], self.nb_bits);
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


impl Mul for BinaryNumber {
    type Output = u32;
    fn mul(self, rhs: Self) -> Self::Output {
        self.number * rhs.number
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
        let result = BinaryNumber::ones_at(&[3, 1], 4);
        assert_eq!(result, BinaryNumber::new(0b1010, 4))
    }

    #[test]
    fn can_check_if_binary_number_has_one_at_position() {
        let number = BinaryNumber::new(0b1000, 4);
        assert!(number.has_one_at(3));
        assert!(!number.has_one_at(2));
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