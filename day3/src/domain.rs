use std::fmt::{Debug, Display, Formatter};
use std::ops;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct DiagnosticReport {
    counts_of_ones: Vec<u32>,
    number_of_readings: u32,
}

impl DiagnosticReport {
    pub fn new(binary_number: &BinaryNumber) -> Self {
        DiagnosticReport {
            counts_of_ones: binary_number.number.iter().map(|is_one| if *is_one { 1 } else { 0 }).collect(),
            number_of_readings: 1,
        }
    }

    pub fn gamma_rate(&self) -> BinaryNumber {
        BinaryNumber {
            number: self.counts_of_ones.iter().map(|count| *count > (self.number_of_readings / 2)).collect()
        }
    }

    pub fn epsilon_rate(&self) -> BinaryNumber {
        BinaryNumber {
            number: self.counts_of_ones.iter().map(|count| *count < (self.number_of_readings / 2)).collect()
        }
    }
}

impl ops::Add<&BinaryNumber> for DiagnosticReport {
    type Output = DiagnosticReport;

    fn add(self, rhs: &BinaryNumber) -> Self::Output {
        DiagnosticReport {
            number_of_readings: self.number_of_readings + 1,
            counts_of_ones: self.counts_of_ones.iter()
                .zip(rhs.number.iter())
                .map(|(count, is_one)| if *is_one { count + 1 } else { *count })
                .collect(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct BinaryNumber {
    number: Vec<bool>,
}

impl BinaryNumber {
    pub fn to_u32(&self) -> u32 {
        u32::from_str_radix(&self.to_string(), 2).unwrap()
    }
}

impl FromStr for BinaryNumber {
    type Err = ();

    fn from_str(reading: &str) -> Result<Self, Self::Err> {
        Ok(BinaryNumber { number: reading.chars().map(|c| c == '1').collect() })
    }
}

impl Display for BinaryNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let binary_as_string = self.number.iter().map(|is_one| if *is_one { '1' } else { '0' }).collect::<String>();
        write!(f, "{}", binary_as_string)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_string_to_binary_number() {
        let result: BinaryNumber = "1101".parse().expect("Should parse");
        assert_eq!(result, BinaryNumber { number: vec![true, true, false, true] })
    }

    #[test]
    fn can_add_a_binary_number_to_a_report() {
        let binary_number: BinaryNumber = "100".parse().unwrap();
        let report = DiagnosticReport::new(&"110".parse().unwrap());
        let report = report + &binary_number;
        assert_eq!(report, DiagnosticReport {
            counts_of_ones: vec![2, 1, 0],
            number_of_readings: 2,
        })
    }

    #[test]
    fn can_get_gamma_and_epsilon_rate_from_report() {
        let report = DiagnosticReport {
            counts_of_ones: vec![7, 11, 2],
            number_of_readings: 12,
        };

        assert_eq!(report.gamma_rate(), BinaryNumber::from_str("110").unwrap());
        assert_eq!(report.epsilon_rate(), BinaryNumber::from_str("001").unwrap());
    }

    #[test]
    fn can_get_binary_number_as_int() {
        let binary_number = BinaryNumber::from_str("011").unwrap();

        assert_eq!(binary_number.to_u32(), 3);
    }
}