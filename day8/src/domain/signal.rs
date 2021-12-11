use std::ops::Add;
use std::str::FromStr;

mod encoding {
    pub const A: u8 = 0b1000000;
    pub const B: u8 = 0b0100000;
    pub const C: u8 = 0b0010000;
    pub const D: u8 = 0b0001000;
    pub const E: u8 = 0b0000100;
    pub const F: u8 = 0b0000010;
    pub const G: u8 = 0b0000001;
    pub const ZERO: u8 = 0b0000000;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Signal(u8);

impl Signal {
    pub fn count_segments(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn includes(&self, value: Signal) -> bool {
        self.overlaps(value) == value
    }

    pub fn overlaps(&self, Signal(value): Signal) -> Signal {
        Signal(self.0 & value)
    }
}

impl Add for Signal {
    type Output = Signal;

    fn add(self, Signal(value): Self) -> Self::Output {
        Signal(self.0 | value)
    }
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Signal(s.chars().fold(encoding::ZERO as u8, |acc, c| {
            return acc | match c {
                'a' => encoding::A,
                'b' => encoding::B,
                'c' => encoding::C,
                'd' => encoding::D,
                'e' => encoding::E,
                'f' => encoding::F,
                'g' => encoding::G,
                _ => encoding::ZERO,
            };
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_includes() {
        let signal1 = Signal(0b1111000);
        let signal2 = Signal(0b0011000);
        assert!(signal1.includes(signal2));
        assert!(!signal2.includes(signal1));
    }

}