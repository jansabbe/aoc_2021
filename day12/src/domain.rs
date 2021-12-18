use std::ops::Sub;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum NodeId {
    Start,
    End,
    Lower(String),
    Upper(String),
}

impl FromStr for NodeId {
    type Err = ();

    fn from_str(node_str: &str) -> Result<Self, Self::Err> {
        match node_str {
            "start" => Ok(NodeId::Start),
            "end" => Ok(NodeId::End),
            val if val.to_uppercase() == val => Ok(NodeId::Upper(val.to_string())),
            val if val.to_lowercase() == val => Ok(NodeId::Lower(val.to_string())),
            _ => Err(())
        }
    }
}



#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Connection(pub NodeId, pub NodeId);

impl FromStr for Connection {
    type Err = ();

    fn from_str(connection_str: &str) -> Result<Self, Self::Err> {
        if let Some((a,b)) = connection_str.split_once("-") {
            let node_a: NodeId = a.parse()?;
            let node_b: NodeId = b.parse()?;
            return Ok(Connection(node_a, node_b));
        }
        return Err(())
    }
}


#[derive(Eq, PartialEq, Debug)]
pub struct Path(Vec<NodeId>);

impl Path {
    pub fn new() -> Self {
        Path(vec![NodeId::Start])
    }

    pub fn follow(&self, node: &NodeId) -> Path {
        let mut new_path = self.0.clone();
        new_path.push(node.clone());
        Path(new_path)
    }

    pub fn can_follow(&self, node: &NodeId) -> bool {
        match node {
            NodeId::Start => false,
            NodeId::End | NodeId::Upper(_) => true,
            lower => !self.0.contains(lower),
        }
    }

    pub fn last_node(&self) -> &NodeId {
        self.0.last().expect("Should have atleast one node")
    }
}
