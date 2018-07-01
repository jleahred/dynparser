//! Data information to build the AST

// -------------------------------------------------------------------------------------
//  T Y P E S

/// Information of a node
#[derive(Debug)]
pub enum NodeInfo {
    /// The node is terminal (atom) with a name
    Val(String),
    /// The node is not terminal (rule) with a name
    Rule(String),
    /// Reached end of filt
    EOF,
}

/// AST node tree
#[derive(Debug)]
pub struct Node {
    /// node information
    pub inf: NodeInfo,
    /// list of nodes under it
    pub nodes: Vec<Node>,
}

//  T Y P E S
// -------------------------------------------------------------------------------------
