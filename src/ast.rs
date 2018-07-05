//! Data information to build the AST

// -------------------------------------------------------------------------------------
//  T Y P E S

/// Information of a node
#[derive(Debug)]
pub enum Node {
    /// The node is terminal (atom) with a name
    Val(String),
    /// The node is not terminal (rule)
    /// with a name and a vec of nodes
    Rule((String, Vec<Node>)),
    /// Reached end of file
    EOF,
}
