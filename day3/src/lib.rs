mod domain;
use domain::{BinaryNumber, DiagnosticReport};


pub fn calculate_power_consumption(contents: &str) -> u32 {
    if let Some((first, rest)) = get_binary_numbers(contents).as_slice().split_first() {
        let report = rest.iter().fold(DiagnosticReport::new(first), |report, binary| report + binary);
        return report.gamma_rate().to_u32() * report.epsilon_rate().to_u32();
    }
    0
}

fn get_binary_numbers(contents: &str) -> Vec<BinaryNumber> {
    contents.lines()
        .filter_map(|line| line.parse::<BinaryNumber>().ok())
        .collect()
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