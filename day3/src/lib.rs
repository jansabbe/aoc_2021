mod domain;
use domain::{BinaryNumber, BinaryNumberList};

pub fn calculate_power_consumption(contents: &str) -> u32 {
    let list = get_binary_numbers(contents);
    let gamma_rate = list.create_number(|nb_ones, nb_zeros| nb_ones > nb_zeros);
    let epsilon_rate = list.create_number(|nb_ones, nb_zeros| nb_ones < nb_zeros);
    gamma_rate * epsilon_rate
}

pub fn calculate_life_support_rating(contents: &str) -> u32 {
    let list = get_binary_numbers(contents);
    let oxygen_generator = list.find(|nb_ones, nb_zeros| nb_ones >= nb_zeros);
    let co2_scrubber = list.find(|nb_ones, nb_zeros| nb_ones < nb_zeros);
    oxygen_generator * co2_scrubber
}

fn get_binary_numbers(contents: &str) -> BinaryNumberList {
    BinaryNumberList::new(
        contents
            .lines()
            .filter_map(|line| line.parse::<BinaryNumber>().ok())
            .collect(),
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
