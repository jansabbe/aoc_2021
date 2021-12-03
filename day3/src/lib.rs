mod domain;

use domain::{BinaryNumber};
use crate::domain::BinaryNumberList;


pub fn calculate_power_consumption(contents: &str) -> u32 {
    let list = get_binary_numbers(contents);
    let gamma_rate = list.most_common_bits();
    let epsilon_rate = list.least_common_bits();
    gamma_rate * epsilon_rate
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
}