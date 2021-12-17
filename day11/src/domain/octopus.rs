use std::num::ParseIntError;
use std::str::FromStr;
use Octopus::{Flashed, GainingEnergy, WillFlash};

#[derive(Debug, PartialEq, Eq)]
pub enum Octopus {
    GainingEnergy(u8),
    WillFlash,
    Flashed,
}

impl Octopus {
    pub fn increase(&mut self) {
        *self = match self {
            GainingEnergy(value) => {
                if *value == 9 {
                    WillFlash
                } else {
                    GainingEnergy(*value + 1)
                }
            }
            WillFlash => Flashed,
            Flashed => Flashed,
        }
    }

    pub fn reset(&mut self) {
        if *self == Flashed {
            *self = GainingEnergy(0)
        }
    }
}

impl FromStr for Octopus {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u8 = s.parse()?;
        Ok(GainingEnergy(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_gain_energy() {
        let mut octopus = GainingEnergy(3);
        octopus.increase();
        assert_eq!(octopus, GainingEnergy(4))
    }

    #[test]
    fn can_be_primed_to_flash() {
        let mut octopus = GainingEnergy(9);
        octopus.increase();
        assert_eq!(octopus, WillFlash)
    }

    #[test]
    fn can_be_flash() {
        let mut octopus = WillFlash;
        octopus.increase();
        assert_eq!(octopus, Flashed)
    }

    #[test]
    fn cannot_overflash() {
        let mut octopus = Flashed;
        octopus.increase();
        assert_eq!(octopus, Flashed)
    }

    #[test]
    fn can_reset_flash() {
        let mut octopus = Flashed;
        octopus.reset();
        assert_eq!(octopus, GainingEnergy(0))
    }

    #[test]
    fn reset_does_nothing_else() {
        let mut octopus = GainingEnergy(3);
        octopus.reset();
        assert_eq!(octopus, GainingEnergy(3))
    }
}
