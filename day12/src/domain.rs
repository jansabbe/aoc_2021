use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Cave {
    Start,
    End,
    Small([char; 2]),
    Big([char; 2]),
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            val if val.to_uppercase() == val => {
                let first_two_chars: Vec<char> = val.chars().take(2).collect();
                Ok(Cave::Big([
                    *first_two_chars.get(0).unwrap_or(&'\0'),
                    *first_two_chars.get(1).unwrap_or(&'\0')
                ]))
            }
            val if val.to_lowercase() == val => {
                let first_two_chars: Vec<char> = val.chars().take(2).collect();
                Ok(Cave::Small([
                    *first_two_chars.get(0).unwrap_or(&'\0'),
                    *first_two_chars.get(1).unwrap_or(&'\0')
                ]))
            }
            _ => Err(())
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Connection(pub Cave, pub Cave);

impl FromStr for Connection {
    type Err = ();

    fn from_str(connection_str: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = connection_str.split_once("-") {
            let cave_a: Cave = a.parse()?;
            let cave_b: Cave = b.parse()?;
            return Ok(Connection(cave_a, cave_b));
        }
        return Err(());
    }
}


#[derive(Eq, PartialEq, Debug)]
pub struct Path {
    caves: Vec<Cave>,
    visited_small_cave_twice: bool,
}

impl Path {
    pub fn new() -> Self {
        Path {
            caves: vec![Cave::Start],
            visited_small_cave_twice: false,
        }
    }

    pub fn follow(&self, cave: Cave) -> Path {
        let will_visit_small_twice = matches!(cave, Cave::Small(_)) && self.caves.contains(&cave);
        let mut new_path = self.caves.clone();
        new_path.push(cave);

        Path {
            caves: new_path,
            visited_small_cave_twice: self.visited_small_cave_twice || will_visit_small_twice
        }
    }

    pub fn last_cave(&self) -> Cave {
        *self.caves.last().expect("Should have atleast one cave")
    }

    pub fn can_visit_lower(&self, cave: Cave) -> bool {
        !self.caves.contains(&cave) || !self.visited_small_cave_twice

    }
}
