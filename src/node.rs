use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub depth: usize,
    pub node_type: NodeType,
    pub path: PathBuf,
}

impl Node {
    pub fn new(name: String, depth: usize, node_type: NodeType) -> Self {
        Self {
            name,
            depth,
            node_type,
            path: PathBuf::new(),
        }
    }
}
