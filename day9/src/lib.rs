use crate::domain::{Height, HeightMap};

mod domain;

pub fn calculate_risk_level(input: &str) -> i32 {
    let map: HeightMap = input.parse().expect("Cannot parse heightmap");

    map.low_points().map(|Height(_, height)| height + 1).sum()
}

pub fn calculate_size_basins(input: &str) -> i32 {
    let map: HeightMap = input.parse().expect("Cannot parse heightmap");

    let mut result: Vec<i32> = map
        .low_points()
        .map(|Height(pos, _)| map.flood(pos).len() as i32)
        .collect();
    result.sort();
    result.reverse();
    return result[0..3].iter().fold(1, |acc, el| acc * el);
}

#[cfg(test)]
mod tests {
    use crate::{calculate_risk_level, calculate_size_basins};

    #[test]
    fn it_works() {
        let result = calculate_risk_level(
            "\
            2199943210\n\
            3987894921\n\
            9856789892\n\
            8767896789\n\
            9899965678\n",
        );
        assert_eq!(result, 15);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(calculate_risk_level(input), 548);
    }

    #[test]
    fn it_works_part2() {
        let result = calculate_size_basins(
            "\
            2199943210\n\
            3987894921\n\
            9856789892\n\
            8767896789\n\
            9899965678\n",
        );
        assert_eq!(result, 1134);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(calculate_size_basins(input), 786048);
    }
}
