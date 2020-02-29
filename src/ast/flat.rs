//! Data information to build the Flat AST
//! And some functions to work with it
//!
//! The AST is a tree, because... it has to be during the parsing
//! and building.
//!
//! But once the input has been processed, we will want to follow
//! the tree in a specific order.
//!
//! It's not complicated visit the elements in a tree, but could
//! be easier if the AST has been "flattened"
//!
//! In order to flatten the tree, is necessary to add a new kind
//! of node. The end 'Rule'
//!

use crate::ast::{self, error, Error};
use idata::cont::IVec;
use std::result::Result;

// -------------------------------------------------------------------------------------
//  T Y P E S

/// Information of a node when ast has been flattened
#[derive(Debug, PartialEq)]
pub enum Node {
    /// The node is terminal (atom) with a name
    Val(String),
    /// Starts a rule
    BeginRule(String),
    /// Ends a rule
    EndRule(String),
    /// Reached end of file
    EOF,
}

impl ast::Node {
    /// Flattening tree
    ///
    /// It's very commont to visit nodes in order
    /// The grammar checked the consistency of input
    /// Flattening the AST, could be interesting in order to process the tree
    ///
    /// ```
    ///    use dynparser::ast::{self, flat};
    ///    
    ///    let ast_before_flatten = ast::Node::Rule((
    ///        "first".to_string(),
    ///        vec![
    ///            ast::Node::Rule((
    ///                "node1".to_string(),
    ///                vec![
    ///                    ast::Node::Val("hello".to_string()),
    ///                    ast::Node::Rule(("node1.1".to_string(), vec![ast::Node::Val(" ".to_string())])),
    ///                ],
    ///            )),
    ///            ast::Node::Rule((
    ///                "node2".to_string(),
    ///                vec![ast::Node::Val("world".to_string())],
    ///            )),
    ///        ],
    ///    ));
    ///
    ///    let vec_after_flatten =
    ///        vec![
    ///            flat::Node::BeginRule("first".to_string()),
    ///            flat::Node::BeginRule("node1".to_string()),
    ///            flat::Node::Val("hello".to_string()),
    ///            flat::Node::BeginRule("node1.1".to_string()),
    ///            flat::Node::Val(" ".to_string()),
    ///            flat::Node::EndRule("node1.1".to_string()),
    ///            flat::Node::EndRule("node1".to_string()),
    ///            flat::Node::BeginRule("node2".to_string()),
    ///            flat::Node::Val("world".to_string()),
    ///            flat::Node::EndRule("node2".to_string()),
    ///            flat::Node::EndRule("first".to_string()),
    ///        ];
    ///
    ///    assert!(ast_before_flatten.flatten() == vec_after_flatten)
    ///```
    pub fn flatten(&self) -> Vec<Node> {
        fn flatten_acc(acc: Vec<Node>, next: &ast::Node) -> Vec<Node> {
            match next {
                ast::Node::EOF => acc,
                ast::Node::Val(v) => acc.ipush(Node::Val(v.clone())),
                ast::Node::Rule((n, vn)) => {
                    let acc = acc.ipush(Node::BeginRule(n.to_string()));
                    let acc = vn.iter().fold(acc, |facc, n| flatten_acc(facc, n));
                    acc.ipush(Node::EndRule(n.to_string()))
                }
            }
        }

        flatten_acc(vec![], self)
    }
}

/// Consume a ast::flat::Node if it's a Rule kind with a specific value
/// and return the rest of nodes
///
///```
///     use dynparser::ast::flat;
///     let nodes = vec![
///                    flat::Node::BeginRule("hello".to_string()),
///                    flat::Node::Val("world".to_string()),
///                ];
///     
///     let nodes = flat::consume_node_start_rule_name("hello", &nodes).unwrap();
///
///     let (node, nodes)  = flat::consume_val(nodes).unwrap();
///     assert!(node == "world");
///     assert!(nodes.len() == 0);
///```
///
pub fn consume_node_start_rule_name<'a>(
    name: &str,
    nodes: &'a [Node],
) -> Result<&'a [Node], Error> {
    let (node, nodes) = split_first_nodes(nodes)?;
    let node_name = match node {
        Node::BeginRule(n) => Ok(n),
        _ => Err(error(&format!("expected begin rule for {}", name), None)),
    }?;
    if node_name == name {
        Ok(nodes)
    } else {
        Err(error(
            &format!("expected {} node, received {}", name, node_name),
            None,
        ))
    }
}

/// Consume a Node indicating the end of a rule,
/// if it's a Rule kind with a specific name
/// and return the rest of nodes
///
///```
///     use dynparser::ast::flat;
///     let nodes = vec![
///                 flat::Node::EndRule("hello".to_string()),
///                 flat::Node::EndRule("hi".to_string()),
///     ];
///     
///     let nodes = flat::consume_node_end_rule_name("hello", &nodes).unwrap();
///     let nodes = flat::consume_node_end_rule_name("hi", &nodes).unwrap();
///     assert!(nodes.len() == 0);
///```
///
pub fn consume_node_end_rule_name<'a>(name: &str, nodes: &'a [Node]) -> Result<&'a [Node], Error> {
    let (node, nodes) = split_first_nodes(nodes)?;
    let node_name = match node {
        Node::EndRule(n) => Ok(n),
        _ => Err(error(&format!("expected end rule for {}", name), None)),
    }?;
    if node_name == name {
        Ok(nodes)
    } else {
        Err(error(
            &format!("expected {} node, received {}", name, node_name),
            None,
        ))
    }
}

/// Given a list of nodes, return the first and the rest on a tuple
///
///```
///     use dynparser::ast::flat;
///     let nodes = vec![
///                 flat::Node::Val("hello".to_string()),
///                 flat::Node::Val("world".to_string()),
///                 flat::Node::Val(".".to_string()),
///     ];
///     
///     let (node, nodes) = flat::split_first_nodes(&nodes).unwrap();
///     assert!(flat::get_node_val(node).unwrap() == "hello");
///     assert!(nodes.len() == 2);
///
///     let (node, nodes) = flat::split_first_nodes(&nodes).unwrap();
///     assert!(flat::get_node_val(node).unwrap() == "world");
///     assert!(nodes.len() == 1);
///
///     let (node, nodes) = flat::split_first_nodes(&nodes).unwrap();
///     assert!(flat::get_node_val(node).unwrap() == ".");
///     assert!(nodes.len() == 0);
///```
///
pub fn split_first_nodes(nodes: &[Node]) -> Result<(&Node, &[Node]), Error> {
    nodes
        .split_first()
        .ok_or_else(|| error("trying get first element from nodes on empty slice", None))
}

/// Return a reference to first node
///
///```
///     use dynparser::ast::flat;
///     let nodes = vec![
///                    flat::Node::BeginRule("hello".to_string()),
///                    flat::Node::Val("world".to_string()),
///                ];
///     
///     let first = flat::peek_first_node(&nodes).unwrap();
///     assert!(first == &flat::Node::BeginRule("hello".to_string()));
///```
///
pub fn peek_first_node(nodes: &[Node]) -> Result<&Node, Error> {
    if nodes.is_empty() {
        Err(error("exptected node on peek_first_node", None))
    } else {
        Ok(&nodes[0])
    }
}

/// Get the value of the Node
/// If node is not a Node::Val, it will return an error
///```
///     use dynparser::ast::flat;
///     let node = flat::Node::Val("hello".to_string());
///     
///     let val = flat::get_node_val(&node).unwrap();
///     
///     assert!(val == "hello");
///```
pub fn get_node_val(node: &Node) -> Result<&str, Error> {
    match node {
        Node::Val(v) => Ok(v),
        _ => Err(error("expected node::Val", None)),
    }
}

/// Consume a node if it's a Val kind and the vaule is
/// equal to the provider one
///
///```
///     use dynparser::ast::flat;
///     let nodes = vec![
///                 flat::Node::Val("hello".to_string()),
///                 flat::Node::Val("world".to_string()),
///                 flat::Node::Val(".".to_string()),
///     ];
///     
///     let nodes = flat::consume_this_value("hello", &nodes).unwrap();
///     let nodes = flat::consume_this_value("world", &nodes).unwrap();
///```
///
pub fn consume_this_value<'a>(v: &str, nodes: &'a [Node]) -> Result<&'a [Node], Error> {
    let (node, nodes) = split_first_nodes(nodes)?;

    let nv = get_node_val(node)?;
    if nv == v {
        Ok(nodes)
    } else {
        Err(error(
            "trying get first element from nodes on empty slice",
            None,
        ))
    }
}

/// It will get the node name of a node if it's a rule one (begin or end).
/// In other case, it will return an error
///
/// ```
///    use dynparser::ast::flat;
///
///    let node = flat::Node::BeginRule("aaa".to_string());
///
///    let node_name = flat::get_nodename(&node).unwrap();
///
///    assert!(node_name == "aaa");
/// ```
pub fn get_nodename(node: &Node) -> Result<&str, Error> {
    match node {
        Node::BeginRule(nname) => Ok(nname),
        Node::EndRule(nname) => Ok(nname),
        _ => Err(error("expected node::Rule", None)),
    }
}

/// Given a slice of nodes, return the value (&str) of first
/// node if it is a Node::Val and return the rest of nodes
///
/// If it's not possible, it returns an error
///
///```
///     use dynparser::ast::flat;
///     let nodes = vec![
///                 flat::Node::Val("hello".to_string()),
///                 flat::Node::Val("world".to_string()),
///     ];
///     
///     let (val, nodes) = flat::consume_val(&nodes).unwrap();
///     assert!(val == "hello");
///     assert!(nodes.len() == 1);
///
///     let (val, nodes) = flat::consume_val(&nodes).unwrap();
///     assert!(val == "world");
///     assert!(nodes.len() == 0);
///```
///
pub fn consume_val(nodes: &[Node]) -> Result<(&str, &[Node]), Error> {
    let (node, nodes) = split_first_nodes(nodes)?;
    match node {
        Node::Val(v) => Ok((&v, nodes)),
        _ => Err(error("expected Val node", None)),
    }
}
