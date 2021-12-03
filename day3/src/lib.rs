mod domain;

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
    let oxygen_generator = find_number(&list, |nb_ones, nb_zeros| nb_ones >= nb_zeros);
    let co2_scrubber = find_number(&list, |nb_ones, nb_zeros| nb_ones < nb_zeros);
    oxygen_generator * co2_scrubber
}

fn find_number(number_list: &BinaryNumberList, keep_one_if: fn(usize, usize) -> bool) -> BinaryNumber {
    let mut list = number_list.clone();
    for position in (0..list.nb_bits).rev() {
        if list.numbers.len() == 1 {
            break;
        }
        if keep_one_if(list.nb_ones_at(position), list.nb_zeros_at(position)) {
            list.retain(|number| number.has_one_at(position));
        } else {
            list.retain(|number| number.has_zero_at(position));
        }
    }
    list.numbers[0]
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