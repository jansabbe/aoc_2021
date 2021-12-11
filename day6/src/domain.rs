use std::str::FromStr;

pub type AmountOfFish = u64;
pub type FishLifetime = usize;

pub struct SchoolOFish {
    amount_of_fish_per_timer: [AmountOfFish; 9],
}

impl SchoolOFish {
    pub fn new(fishes: Vec<FishLifetime>) -> Self {
        let mut amount_of_fish_per_timer = [0; 9];
        for fish in fishes {
            amount_of_fish_per_timer[fish] += 1
        }
        SchoolOFish {
            amount_of_fish_per_timer
        }
    }

    pub fn simulate_day(&mut self) {
        let [amount_fish_with_zero_left, _xs @ ..] = self.amount_of_fish_per_timer;
        self.amount_of_fish_per_timer.rotate_left(1);
        self.amount_of_fish_per_timer[6] += amount_fish_with_zero_left;
    }

    pub fn total(&self) -> AmountOfFish {
        self.amount_of_fish_per_timer.iter().sum()
    }
}

impl FromStr for SchoolOFish {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let fish: Vec<usize> = line
            .split(',')
            .filter_map(|fish| fish.trim_end().parse::<FishLifetime>().ok())
            .collect();
        Ok(SchoolOFish::new(fish))
    }
}