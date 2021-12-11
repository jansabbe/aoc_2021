use crate::domain::CrabPositions;

mod domain;

pub fn calculate_part1(input: &str) -> i32 {
    let crab_positions: CrabPositions = input.parse().unwrap();
    crab_positions.required_fuel_to_reach_simple(crab_positions.median())
}

pub fn calculate_part2(input: &str) -> i32 {
    let crab_positions: CrabPositions = input.parse().unwrap();

    (crab_positions.min()..=crab_positions.max())
        .map(|potential_target| crab_positions.required_fuel_to_reach_power(potential_target))
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_calculate_1() {
        let distance = calculate_part1("16,1,2,0,4,2,7,1,2,14\n");
        assert_eq!(distance, 37)
    }

    #[test]
    fn test_real_input_1() {
        let input = include_str!("../input.txt");
        let distance = calculate_part1(input);
        assert_eq!(distance, 336120)
    }

    #[test]
    fn test_calculate_part2() {
        let positions = "16, 1, 2, 0, 4, 2, 7, 1, 2, 14\n";
        assert_eq!(calculate_part2(&positions), 168);
    }

    #[test]
    fn test_real_input_2() {
        let input = include_str!("../input.txt");
        let distance = calculate_part2(input);
        assert_eq!(distance, 96864235)
    }
}
