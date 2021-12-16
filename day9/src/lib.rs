use crate::domain::{Height, HeightMap};

mod domain;


pub fn calculate_risk_level(input: &str) -> i32 {
    let height_map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line
            .split("")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect())
        .collect();

    let map = HeightMap::new(height_map.iter());

    let heights_low_points = map.iter_positions()
        .filter(|&pos| {
            let height = map[pos];
            let surrounding = map.adjacent_heights(pos);
            surrounding.iter().all(|&ad| ad > height)
        })
        .map(|pos| map[pos])
        .map(|Height(_, height)| height);


    heights_low_points
        .map(|height| height + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::calculate_risk_level;

    #[test]
    fn it_works() {
        let result = calculate_risk_level("\
            2199943210\n\
            3987894921\n\
            9856789892\n\
            8767896789\n\
            9899965678\n");
        assert_eq!(result, 15);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(calculate_risk_level(input), 548);

    }
}
