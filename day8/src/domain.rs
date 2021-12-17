use std::str::FromStr;

mod signal;

use signal::Signal;

trait SignalFinder {
    fn take<P>(&mut self, predicate: P) -> Signal
    where
        P: Fn(&Signal) -> bool;
}

impl SignalFinder for Vec<Signal> {
    fn take<P>(&mut self, predicate: P) -> Signal
    where
        P: Fn(&Signal) -> bool,
    {
        let index = self.iter().position(predicate).unwrap();
        let result = self[index];
        self.remove(index);
        return result;
    }
}

pub struct Observation {
    outputs: Vec<Signal>,
    signals: [Signal; 10],
}

impl Observation {
    fn new(mut signals: Vec<Signal>, outputs: Vec<Signal>) -> Self {
        let one = signals.take(|one| one.count_segments() == 2);
        let four = signals.take(|four| four.count_segments() == 4);
        let seven = signals.take(|seven| seven.count_segments() == 3);
        let eight = signals.take(|eight| eight.count_segments() == 7);
        let three = signals.take(|three| three.count_segments() == 5 && three.includes(seven));
        let nine = signals
            .take(|nine| nine.count_segments() == 6 && nine.includes(three) && nine.includes(four));
        let zero = signals.take(|zero| {
            zero.count_segments() == 6
                && zero.includes(seven)
                && zero.overlaps(nine).count_segments() == 5
        });
        let six = signals.take(|six| six.count_segments() == 6 && *six + one == eight);
        let two = signals.take(|two| two.count_segments() == 5 && *two + four == eight);
        let five = signals.take(|five| five.count_segments() == 5 && *five + one == nine);
        Observation {
            outputs,
            signals: [zero, one, two, three, four, five, six, seven, eight, nine],
        }
    }

    pub fn output(&self) -> usize {
        let numbers: Vec<String> = self
            .outputs
            .iter()
            .map(|&s| self.value_for_signal(s).to_string())
            .collect();
        numbers.join("").parse().unwrap()
    }

    fn value_for_signal(&self, signal: Signal) -> usize {
        self.signals.iter().position(|&s| s == signal).unwrap()
    }
}

impl FromStr for Observation {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (signals, outputs) = line.split_once(" | ").ok_or(())?;
        Ok(Observation::new(
            signals
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect(),
            outputs
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }
}

pub fn count_uniques(ob: &Observation) -> usize {
    ob.outputs
        .iter()
        .filter(|p| {
            p.count_segments() == 2
                || p.count_segments() == 4
                || p.count_segments() == 3
                || p.count_segments() == 7
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observation() {
        let observation: Observation = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\n".parse().unwrap();
        assert_eq!(observation.outputs[0], observation.signals[5]);
        assert_eq!(observation.outputs[1], observation.signals[3]);
        assert_eq!(observation.outputs[2], observation.signals[5]);
        assert_eq!(observation.outputs[3], observation.signals[3]);
    }

    #[test]
    fn test_observation_output() {
        let observation: Observation = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\n".parse().unwrap();
        assert_eq!(observation.output(), 5353);
    }

    #[test]
    fn test_first_observation_output() {
        let observation: Observation = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n".parse().unwrap();
        assert_eq!(observation.output(), 8394);
    }
}
