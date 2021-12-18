mod domain;

use std::str::FromStr;
use domain::{NodeId, Connection, Path};

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
    fn nodes_connected_to<'a>(&'a self, id: NodeId) -> impl Iterator<Item=NodeId> + 'a {
        self.connections
            .iter()
            .filter_map(move |&Connection(a, b)| match (a, b) {
                (from, to) if to == id => Some(from),
                (from, to) if from == id => Some(to),
                _ => None
            })
    }

    fn paths_to_end(&self) -> Vec<Path> {
        let mut paths_to_end = Vec::new();
        let mut paths_to_consider = vec![Path::new()];

        while !paths_to_consider.is_empty() {
            let current = paths_to_consider.pop().unwrap();
            self.nodes_connected_to(current.last_node()).for_each(|next_node| {
                match next_node {
                    NodeId::Lower(_) if !current.contains(next_node) => {
                        paths_to_consider.push(current.follow(next_node))
                    }
                    NodeId::Upper(_) => {
                        paths_to_consider.push(current.follow(next_node))
                    }
                    NodeId::End => {
                        paths_to_end.push(current.follow(next_node));
                    }
                    _ => {}
                }
            });
        }

        return paths_to_end;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodes_connected_to() {
        let graph: Graph = "\
            start-A\n\
            start-b\n\
            A-c\n\
            A-b\n\
            b-d\n\
            A-end\n\
            b-end\n".parse().unwrap();

        assert_eq!(graph.nodes_connected_to(NodeId::Upper(['A', '\0'])).collect::<Vec<NodeId>>(), vec![
            NodeId::Start,
            NodeId::Lower(['c', '\0']),
            NodeId::Lower(['b', '\0']),
            NodeId::End,
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

        assert_eq!(graph.paths_to_end().len(), 10)
    }

    #[test]
    fn paths_to_end_complex() {
        let graph: Graph = "\
        fs-end\n\
        he-DX\n\
        fs-he\n\
        start-DX\n\
        pj-DX\n\
        end-zg\n\
        zg-sl\n\
        zg-pj\n\
        pj-he\n\
        RW-he\n\
        fs-DX\n\
        pj-RW\n\
        zg-RW\n\
        start-pj\n\
        he-WI\n\
        zg-he\n\
        pj-fs\n\
        start-RW\n".parse().unwrap();
        assert_eq!(graph.paths_to_end().len(), 226)
    }

    #[test]
    fn huh() {
        let graph: Graph = "\
        dc-end\n\
        HN-start\n\
        start-kj\n\
        dc-start\n\
        dc-HN\n\
        LN-dc\n\
        HN-end\n\
        kj-sa\n\
        kj-HN\n\
        kj-dc\n\
        ".parse().unwrap();

        assert_eq!(graph.paths_to_end().len(), 19)
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        let graph: Graph = input.parse().unwrap();
        assert_eq!(graph.paths_to_end().len(), 3000)
    }
}
