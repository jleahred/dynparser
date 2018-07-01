// -------------------------------------------------------------------------------------
//  T Y P E S

#[derive(Debug)]
pub(crate) enum NodeInfo {
    Val(String),
    Rule(String),
    EOF,
}

#[derive(Debug)]
pub struct Node {
    pub inf: NodeInfo,
    pub nodes: Vec<Node>,
}

//  T Y P E S
// -------------------------------------------------------------------------------------
