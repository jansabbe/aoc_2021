fn calculate(positions: &str) -> i32 {
    let positions: Vec<i32> = positions.split(',').filter_map(|p| p.trim().parse::<i32>().ok()).collect();
    calculate_distance(&positions, statistical::median(&positions))
}

fn calculate_distance(positions: &[i32], target: i32) -> i32 {
    positions.iter().map(|&pos| (pos - target).abs()).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_calculate_distance() {
        let distance = calculate("16,1,2,0,4,2,7,1,2,14\n");
        assert_eq!(distance, 37)
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../input.txt");
        let distance = calculate(input);
        assert_eq!(distance, 336120)
    }
}
