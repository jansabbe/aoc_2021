use crate::domain::{AmountOfFish, SchoolOFish};

mod domain;

pub fn number_of_fish(after_days: u32, contents: &str) -> AmountOfFish {
    let mut school = contents.parse::<SchoolOFish>().unwrap();
    (0..after_days).for_each(|_| school.simulate_day());
    school.total()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn number_of_fish_after_18() {
        let example = "3,4,3,1,2\n";
        let result = number_of_fish(18, example);
        assert_eq!(result, 26);
    }

    #[test]
    fn number_of_fish_after_80() {
        let example = "3,4,3,1,2\n";
        let result = number_of_fish(80, example);
        assert_eq!(result, 5934);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        let result = number_of_fish(80, input);
        assert_eq!(result, 352195);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        let result = number_of_fish(256, input);
        assert_eq!(result, 1600306001288);
    }
}
