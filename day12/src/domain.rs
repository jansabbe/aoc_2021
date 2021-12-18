use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NodeId {
    Start,
    End,
    Lower([char; 2]),
    Upper([char; 2]),
}

impl FromStr for NodeId {
    type Err = ();

    fn from_str(node_str: &str) -> Result<Self, Self::Err> {
        match node_str {
            "start" => Ok(NodeId::Start),
            "end" => Ok(NodeId::End),
            val if val.to_uppercase() == val => {
                let first_two_chars: Vec<char> = val.chars().take(2).collect();
                Ok(NodeId::Upper([
                    *first_two_chars.get(0).unwrap_or(&'\0'),
                    *first_two_chars.get(1).unwrap_or(&'\0')
                ]))
            }
            val if val.to_lowercase() == val => {
                let first_two_chars: Vec<char> = val.chars().take(2).collect();
                Ok(NodeId::Lower([
                    *first_two_chars.get(0).unwrap_or(&'\0'),
                    *first_two_chars.get(1).unwrap_or(&'\0')
                ]))
            }
            _ => Err(())
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Connection(pub NodeId, pub NodeId);

impl FromStr for Connection {
    type Err = ();

    fn from_str(connection_str: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = connection_str.split_once("-") {
            let node_a: NodeId = a.parse()?;
            let node_b: NodeId = b.parse()?;
            return Ok(Connection(node_a, node_b));
        }
        return Err(());
    }
}


#[derive(Eq, PartialEq, Debug)]
pub struct Path(Vec<NodeId>);

impl Path {
    pub fn new() -> Self {
        Path(vec![NodeId::Start])
    }

    pub fn follow(&self, node: NodeId) -> Path {
        let mut new_path = self.0.clone();
        new_path.push(node.clone());
        Path(new_path)
    }

    pub fn contains(&self, node: NodeId) -> bool {
        self.0.contains(&node)
    }

    pub fn last_node(&self) -> NodeId {
        *self.0.last().expect("Should have atleast one node")
    }
}
