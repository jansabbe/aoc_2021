mod domain;

use std::str::FromStr;
use domain::{BinaryNumber};
use crate::domain::BinaryNumberList;


pub fn calculate_power_consumption(contents: &str) -> u32 {
    let list = get_binary_numbers(contents);
    let gamma_rate = list.most_common_bits();
    let epsilon_rate = list.least_common_bits();
    gamma_rate * epsilon_rate
}

pub fn calculate_life_support_rating(contents: &str) -> u32 {
    let list = get_binary_numbers(contents);
    oxygen_generator(list.clone()) * co2_scrubber(list)
}

fn co2_scrubber(list: BinaryNumberList) -> BinaryNumber {
    let result = (0..list.nb_bits).rev().fold(list, |list, position| {
        if list.numbers.len() == 1 {
            list
        } else if list.nb_ones_at(position) >= list.nb_zeros_at(position) {
            list.filter_with_zero_at_pos(position)
        } else {
            list.filter_with_one_at_pos(position)
        }
    });

    result.numbers[0]
}

fn oxygen_generator(list: BinaryNumberList) -> BinaryNumber {
    let result = (0..list.nb_bits).rev().fold(list, |list, position| {
        if list.numbers.len() == 1 {
            list
        } else if list.nb_ones_at(position) >= list.nb_zeros_at(position) {
            list.filter_with_one_at_pos(position)
        } else {
            list.filter_with_zero_at_pos(position)
        }
    });

    result.numbers[0]
}

fn get_binary_numbers(contents: &str) -> BinaryNumberList {
    BinaryNumberList::new(contents
        .lines()
        .filter_map(|line| line.parse::<BinaryNumber>().ok())
        .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_power_consumption() {
        let contents = "00100\n\
            11110\n\
            10110\n\
            10111\n\
            10101\n\
            01111\n\
            00111\n\
            11100\n\
            10000\n\
            11001\n\
            00010\n\
            01010\n";
        let result = calculate_power_consumption(contents);
        assert_eq!(result, 198);
    }

    #[test]
    fn can_calculate_life_support_rating() {
        let contents = "00100\n\
            11110\n\
            10110\n\
            10111\n\
            10101\n\
            01111\n\
            00111\n\
            11100\n\
            10000\n\
            11001\n\
            00010\n\
            01010\n";
        let result = calculate_life_support_rating(contents);
        assert_eq!(result, 230);
    }
}