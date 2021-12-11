use std::fmt::Debug;
use std::num::ParseIntError;
use std::ops::{Index, Mul};
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

    pub fn create_number(&self, set_one_if: fn(usize, usize) -> bool) -> BinaryNumber {
        let indices: Vec<usize> = (0..self.nb_bits)
            .filter(|&index| {
                let (nb_ones, nb_zeros) = self.count_at(index);
                set_one_if(nb_ones, nb_zeros)
            })
            .collect();

        BinaryNumber::ones_at(&indices, self.nb_bits)
    }

    pub fn find(&self, keep_one_if: fn(usize, usize) -> bool) -> BinaryNumber {
        let mut list = self.clone();
        for index in (0..list.nb_bits).rev() {
            if list.numbers.len() == 1 {
                break;
            }
            let (nb_ones, nb_zeros) = list.count_at(index);
            if keep_one_if(nb_ones, nb_zeros) {
                list.numbers.retain(|number| number[index] == Bit::One);
            } else {
                list.numbers.retain(|number| number[index] == Bit::Zero);
            }
        }
        list.numbers[0]
    }

    fn count_at(&self, index: usize) -> (usize, usize) {
        let nb_ones = self
            .numbers
            .iter()
            .filter(|binary| binary[index] == Bit::One)
            .count();
        (nb_ones, self.numbers.len() - nb_ones)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BinaryNumber {
    pub nb_bits: usize,
    pub number: u32,
}

impl BinaryNumber {
    fn ones_at(indices: &[usize], nb_bits: usize) -> BinaryNumber {
        let number = indices.iter().map(|&index| 2_u32.pow(index as u32)).sum();
        BinaryNumber { number, nb_bits }
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

#[derive(Debug, Eq, PartialEq)]
pub enum Bit {
    Zero,
    One,
}

impl Index<usize> for BinaryNumber {
    type Output = Bit;

    fn index(&self, index: usize) -> &Bit {
        let mask = Self::ones_at(&[index], self.nb_bits);
        if self.number & mask.number > 0 {
            &Bit::One
        } else {
            &Bit::Zero
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_string_to_binary_number() {
        let result = BinaryNumber::from_str("01101").unwrap();
        assert_eq!(
            result,
            BinaryNumber {
                number: 0b1101,
                nb_bits: 5
            }
        )
    }

    #[test]
    fn can_create_binary_number_with_one_at_position() {
        let result = BinaryNumber::ones_at(&[3, 1], 4);
        assert_eq!(
            result,
            BinaryNumber {
                number: 0b1010,
                nb_bits: 4
            }
        )
    }

    #[test]
    fn can_check_if_binary_number_has_one_at_position() {
        let number = BinaryNumber {
            number: 0b1000,
            nb_bits: 4,
        };
        assert_eq!(number[3], Bit::One);
        assert_eq!(number[2], Bit::Zero);
    }

    #[test]
    fn can_create_most_significant_number() {
        let numbers = BinaryNumberList::new(vec![
            BinaryNumber::from_str("100").unwrap(),
            BinaryNumber::from_str("101").unwrap(),
            BinaryNumber::from_str("011").unwrap(),
        ]);

        let result = numbers.create_number(|nb_ones, nb_zeros| nb_ones > nb_zeros);

        assert_eq!(result, BinaryNumber::from_str("101").unwrap());
    }
}
