use std::str::FromStr;

pub struct Observation {
    signals: Vec<String>,
    pub outputs: Vec<String>,
}

impl FromStr for Observation {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (signals, outputs) = line.split_once(" | ").ok_or(())?;
        Ok(Observation {
            signals: signals.split_whitespace().map(String::from).collect(),
            outputs: outputs.split_whitespace().map(String::from).collect(),
        })
    }
}

pub fn count_uniques(ob: &Observation) -> usize {
    ob.outputs
        .iter()
        .filter(|p| p.len() == 2 || p.len() == 4 || p.len() == 3 || p.len() == 7)
        .count()
}