fn calculate_part1(positions: &str) -> i32 {
    let positions: Vec<i32> = positions
        .split(',')
        .filter_map(|p| p.trim().parse::<i32>().ok())
        .collect();

    calculate_distance_part1(&positions, statistical::median(&positions))
}

fn calculate_distance_part1(positions: &[i32], target: i32) -> i32 {
    positions.iter().map(|&pos| (pos - target).abs()).sum()
}

fn calculate_part2(positions: &str) -> i32 {
    let positions: Vec<i32> = positions
        .split(',')
        .filter_map(|p| p.trim().parse::<i32>().ok())
        .collect();

    let &from = positions.iter().min().unwrap();
    let &to = positions.iter().max().unwrap();

    (from..=to)
        .map(|potential_target| calculate_distance_part2(&positions, potential_target))
        .min()
        .unwrap_or(0)
}

fn calculate_distance_part2(positions: &[i32], target: i32) -> i32 {
    positions
        .iter()
        .map(|&pos| {
            let steps = (pos - target).abs();
            (steps * (steps + 1)) / 2 // Gauss formula for 1 + 2 + 3 + ... steps
        })
        .sum()
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
    fn test_calculate_distance_part2() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(calculate_distance_part2(&positions, 5), 168);
        assert_eq!(calculate_distance_part2(&positions, 2), 206);
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
