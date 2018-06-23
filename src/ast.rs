/// contains a char slice and a (char,char) slice
/// if char matches one in char slice -> OK
/// if char matches between tuple in elems slice -> OK
#[derive(Debug)]
pub struct MatchRules<'a>(&'a str, &'a [(char, char)]);
// -------------------------------------------------------------------------------------
//  T Y P E S

// #[derive(Debug, Clone, Copy)]
/// Kind of node
pub enum K {
    Root,
    Expression,
    Atom,
}

/// Non terminal symbols will match expresions
pub enum Expression {
    And,
    Not,
    Repeat,
}

/// terminal symbols will math atoms
pub enum Atom {
    Dot,
    Lit,
    Match,
    Symbref,
    Eof,
}

// #[derive(Debug)]
pub struct V(String);

// #[derive(Debug)]
pub struct Node {
    pub kind: K,
    pub val: V,
    pub nodes: Vec<Node>,
}

//  T Y P E S
// -------------------------------------------------------------------------------------
