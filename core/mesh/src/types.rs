use std::fmt;

//& Logical identity of a peer/node in a mesh
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(String);

impl NodeId {
    //? Create a new NodeId from a string
    pub fn new(value: impl Into<String>) -> Self{
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

//? Implementation of Display for NodeID
impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

//? Implement From traits for easy conversion from string types to NodeId
impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for NodeId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

//? Transport level packet bytes
pub type RawPacket = Vec<u8>;