mod domain;

use std::str::FromStr;
use domain::{Cave, Connection, Path};

struct Graph {
    connections: Vec<Connection>,
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Graph {
            connections: input
                .lines()
                .filter_map(|line| line.parse::<Connection>().ok())
                .collect()
        })
    }
}

impl Graph {
    fn caves_connected_to(&self, cave: Cave) -> impl Iterator<Item=Cave> + '_ {
        self.connections
            .iter()
            .filter_map(move |&Connection(a, b)| match (a, b) {
                (from, to) if to == cave => Some(from),
                (from, to) if from == cave => Some(to),
                _ => None
            })
    }

    fn paths_to_end(&self) -> i32 {
        let mut paths_to_end = 0;
        let mut paths_to_consider = vec![Path::new()];
        while !paths_to_consider.is_empty() {
            let path = paths_to_consider.pop().unwrap();

            for next_cave in self.caves_connected_to(path.last_cave()) {
                match next_cave {
                    Cave::Small(_) if path.can_visit_lower(next_cave) => {
                        paths_to_consider.push(path.follow(next_cave))
                    }
                    Cave::Big(_) => {
                        paths_to_consider.push(path.follow(next_cave))
                    }
                    Cave::End => {
                        paths_to_end += 1
                    }
                    _ => {}
                }
            }
        }

        return paths_to_end;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caves_connected_to() {
        let graph: Graph = "\
            start-A\n\
            start-b\n\
            A-c\n\
            A-b\n\
            b-d\n\
            A-end\n\
            b-end\n".parse().unwrap();

        assert_eq!(graph.caves_connected_to(Cave::Big(['A', '\0'])).collect::<Vec<Cave>>(), vec![
            Cave::Start,
            Cave::Small(['c', '\0']),
            Cave::Small(['b', '\0']),
            Cave::End,
        ])
    }

    #[test]
    fn paths_to_end() {
        let graph: Graph = "\
            start-A\n\
            start-b\n\
            A-c\n\
            A-b\n\
            b-d\n\
            A-end\n\
            b-end\n".parse().unwrap();

        assert_eq!(graph.paths_to_end(), 36)
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        let graph: Graph = input.parse().unwrap();
        assert_eq!(graph.paths_to_end(), 74222)
    }
}
