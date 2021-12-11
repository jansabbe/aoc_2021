use std::str::FromStr;

pub struct CrabPositions {
    positions: Vec<i32>,
}

impl CrabPositions {
    pub fn required_fuel_to_reach_simple(&self, target: i32) -> i32 {
        self.positions.iter().map(|&pos| (pos - target).abs()).sum()
    }

    pub fn required_fuel_to_reach_power(&self, target: i32) -> i32 {
        self.positions
            .iter()
            .map(|&pos| {
                let steps = (pos - target).abs();
                (steps * (steps + 1)) / 2 // Gauss formula for 1 + 2 + 3 + ... steps
            })
            .sum()
    }

    pub fn median(&self) -> i32 {
        statistical::median(&self.positions)
    }

    pub fn min(&self) -> i32 {
        *self.positions.iter().min().unwrap()
    }

    pub fn max(&self) -> i32 {
        *self.positions.iter().max().unwrap()
    }
}

impl FromStr for CrabPositions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .split(',')
            .filter_map(|p| p.trim().parse::<i32>().ok())
            .collect();
        Ok(CrabPositions { positions })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_power() {
        let positions = CrabPositions {
            positions: vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14],
        };
        assert_eq!(positions.required_fuel_to_reach_power(5), 168);
        assert_eq!(positions.required_fuel_to_reach_power(2), 206);
    }
}
