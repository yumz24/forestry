use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    File,
    Directory,
    Symlink { target: String },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub depth: usize,
    pub node_type: NodeType,
    pub path: PathBuf,
}
