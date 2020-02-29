//! Data information to build the AST
//! And some functions to work with AST
//!

use idata::cont::IVec;
use std::result::Result;

pub mod flat;

// -------------------------------------------------------------------------------------
//  T Y P E S

/// Context information about an error manipulanting the ast
/// You will have an String with the description, and the node
/// wich produced the error
/// It will have the error description and the string of node
/// info
#[derive(Debug, PartialEq)]
pub struct Error(pub String, pub Option<String>);

/// Helper to create an ast::Error
/// ```
///    use dynparser::ast;
///
///    let error_a = ast::error("testing", None);
///    let error_b = ast::Error("testing".to_string(), None);
///
///    assert!(error_a == error_b)
/// ```

pub fn error(desc: &str, ast_context: Option<&str>) -> Error {
    Error(desc.to_string(), ast_context.map(|a| a.to_string()))
}

/// Information of a node
#[derive(Debug, PartialEq)]
pub enum Node {
    /// The node is terminal (atom) with a name
    Val(String),
    /// The node is not terminal (rule)
    /// with a name and a vec of nodes
    Rule((String, Vec<Node>)),
    /// Reached end of file
    EOF,
}

impl Node {
    /// Remove nodes with one of the names in the list.
    /// It will remove the childs
    /// ```
    ///    use dynparser::ast;
    ///
    ///    let ast_before_prune: ast::Node = ast::Node::Rule((
    ///        "root".to_string(),
    ///        vec![ast::Node::Rule((
    ///            "a".to_string(),
    ///            vec![
    ///                ast::Node::Rule(("_1".to_string(), vec![])),
    ///                ast::Node::Rule(("_2".to_string(), vec![])),
    ///            ],
    ///        ))],
    ///    ));
    ///
    ///    let ast_after_prune = ast::Node::Rule((
    ///        "root".to_string(),
    ///        vec![ast::Node::Rule(("a".to_string(), vec![]))],
    ///    ));
    ///
    ///    assert!(ast_before_prune.prune(&vec!["_1", "_2"]) == ast_after_prune)
    /// ```

    pub fn prune(&self, nodes2prune: &[&str]) -> Self {
        let nname2prune = |nname: &str| nodes2prune.iter().find(|n| *n == &nname);
        let node2prune = |node: &Node| match node {
            Node::Rule((nname, _)) => nname2prune(nname).is_some(),
            _ => false,
        };
        let prune_vn = |vnodes: &[Node]| {
            vnodes.iter().fold(vec![], |acc, n| {
                if !node2prune(n) {
                    acc.ipush(n.prune(nodes2prune))
                } else {
                    acc
                }
            })
        };
        match self {
            Node::EOF => Node::EOF,
            Node::Val(v) => Node::Val(v.clone()),
            Node::Rule((n, vn)) => Node::Rule((n.clone(), prune_vn(vn))),
        }
    }

    /// Remove nodes excepting names in the list.
    /// Childs will be connected to the parent node removed
    /// ```
    ///
    /// use dynparser::ast;
    ///
    /// let ast_before_pass_through: ast::Node = ast::Node::Rule((
    ///     "root".to_string(),
    ///     vec![ast::Node::Rule((
    ///         "a".to_string(),
    ///         vec![ast::Node::Rule((
    ///             "_1".to_string(),
    ///             vec![ast::Node::Rule(("_2".to_string(), vec![]))],
    ///         ))],
    ///     ))],
    /// ));
    ///
    /// let ast_after_pass_through: ast::Node = ast::Node::Rule((
    ///     "root".to_string(),
    ///     vec![ast::Node::Rule((
    ///         "a".to_string(),
    ///         vec![ast::Node::Rule(("_2".to_string(), vec![]))],
    ///     ))],
    /// ));
    ///
    /// assert!(ast_before_pass_through.pass_through_except(&vec!["root", "a", "_2"]) == ast_after_pass_through)
    /// ```

    pub fn pass_through_except(&self, nodes2keep: &[&str]) -> Self {
        fn pthr_vn(vnodes: &[Node], nodes2keep: &[&str]) -> Vec<Node> {
            let nname2keep = |nname: &str| nodes2keep.iter().find(|n| *n == &nname);
            let node2keep = |node: &Node| match node {
                Node::Rule((nname, _)) => nname2keep(nname).is_some(),
                _ => true,
            };
            vnodes.iter().fold(vec![], |acc, n| {
                if node2keep(n) {
                    acc.ipush(n.pass_through_except(nodes2keep))
                } else {
                    match n {
                        Node::Rule((_, new_nodes)) => acc.iappend(pthr_vn(new_nodes, nodes2keep)),
                        _ => acc.ipush(n.pass_through_except(nodes2keep)),
                    }
                }
            })
        };
        match self {
            Node::EOF => Node::EOF,
            Node::Val(v) => Node::Val(v.clone()),
            Node::Rule((n, vn)) => Node::Rule((n.clone(), pthr_vn(vn, nodes2keep))),
        }
    }

    /// Concat consecutive Val nodes
    /// ```
    ///    use dynparser::ast;
    ///    
    ///    let ast_before_compact: ast::Node = ast::Node::Rule((
    ///        "root".to_string(),
    ///        vec![ast::Node::Rule((
    ///            "node".to_string(),
    ///            vec![
    ///                ast::Node::Val("hello".to_string()),
    ///                ast::Node::Val(" ".to_string()),
    ///                ast::Node::Val("world".to_string()),
    ///            ],
    ///        ))],
    ///    ));
    ///
    ///    let ast_after_compact = ast::Node::Rule((
    ///        "root".to_string(),
    ///        vec![ast::Node::Rule((
    ///            "node".to_string(),
    ///            vec![ast::Node::Val("hello world".to_string())],
    ///        ))],
    ///    ));
    ///
    ///    assert!(ast_before_compact.compact() == ast_after_compact)
    ///```
    pub fn compact(&self) -> Self {
        fn concat_nodes(nodes: Vec<Node>, n: &Node) -> Vec<Node> {
            let get_val = |nodes: &Vec<Node>| match nodes.last() {
                Some(Node::Val(ref v)) => Some(v.to_string()),
                _ => None,
            };
            let concat_v = |v: &String, prev_v: &Option<String>| match (v, prev_v) {
                (v, Some(pv)) => Some(format!("{}{}", pv, v)),
                _ => None,
            };

            match (n, get_val(&nodes)) {
                (Node::EOF, _) => nodes.ipush(Node::EOF),
                (Node::Val(ref v), ref prev_v) => match concat_v(v, prev_v) {
                    Some(c) => {
                        let (_, nodes) = nodes.ipop();
                        nodes.ipush(Node::Val(c.clone()))
                    }
                    _ => nodes.ipush(Node::Val(v.clone())),
                },
                (Node::Rule((ref n, ref vn)), _) => {
                    nodes.ipush(Node::Rule((n.clone(), compact_nodes(vn))))
                }
            }
        };
        fn compact_nodes(nodes: &[Node]) -> Vec<Node> {
            nodes
                .iter()
                .fold(vec![], |acc: Vec<Node>, n| (concat_nodes(acc, n)))
        };
        match self {
            Node::EOF => Node::EOF,
            Node::Val(v) => Node::Val(v.clone()),
            Node::Rule((n, vn)) => Node::Rule((n.clone(), compact_nodes(vn))),
        }
    }
}

/// It will get the node name and a slice to the nodes contained by the node
/// ```
///    use dynparser::ast::{self, get_nodename_and_nodes, Node};
///
///    let ast: Node = Node::Rule((
///        "root".to_string(),
///        vec![Node::Val("hello".to_string())],
///    ));
///
///    let (node_name, nodes) = get_nodename_and_nodes(&ast).unwrap();
///
///    assert!(node_name == "root");
///    assert!(nodes[0] == ast::Node::Val("hello".to_string()),)
/// ```
pub fn get_nodename_and_nodes(node: &Node) -> Result<(&str, &[Node]), Error> {
    match node {
        Node::Rule((nname, nodes)) => Ok((nname, nodes)),
        _ => Err(error("expected node::Rule", None)),
    }
}

/// Get the value of the Node
/// If node is not a Node::Val, it will return an error
///```
///     use dynparser::ast::{self, get_node_val};
///     let ast = ast::Node::Val("hello".to_string());
///     
///     let val = get_node_val(&ast).unwrap();
///     
///     assert!(val == "hello");
///```
pub fn get_node_val(node: &Node) -> Result<&str, Error> {
    match node {
        Node::Val(v) => Ok(v),
        _ => Err(error("expected node::Val", None)),
    }
}

/// Sometimes, processing the ast, you will exptect to have an unique
/// child, and it will have to be a simple Node::Val
/// This function will return the val, or error in other case
///
///```
///     use dynparser::ast;
///     let nodes = vec![ast::Node::Val("hello".to_string())];
///     
///     let val = ast::get_nodes_unique_val(&nodes).unwrap();
///     
///     assert!(val == "hello");
///```
///
/// If you pass an slice with more than one element, it will return
/// an error
///
///```
///     use dynparser::ast;
///     let nodes = vec![ast::Node::Val("hello".to_string()),
///                      ast::Node::Val("world".to_string())];
///     
///     assert!(ast::get_nodes_unique_val(&nodes).is_err());
///```
pub fn get_nodes_unique_val(nodes: &[Node]) -> Result<&str, Error> {
    match (nodes.first(), nodes.len()) {
        (Some(n), 1) => get_node_val(n),
        _ => Err(error("expected only one value in nodes", None)),
    }
}

/// Given a slice of nodes, return the value (&str) of first
/// node if it is a Node::Rule and return the rest of nodes
///
/// If it's not possible, return an error
///
///```
///     use dynparser::ast;
///     let nodes = vec![
///                 ast::Node::Val("hello".to_string()),
///                 ast::Node::Val("world".to_string()),
///     ];
///     
///     let (val, nodes) = ast::consume_val(&nodes).unwrap();
///     assert!(val == "hello");
///     assert!(nodes.len() == 1);
///
///     let (val, nodes) = ast::consume_val(&nodes).unwrap();
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

/// Given a list of nodes, return the first and the rest on a tuple
///
///```
///     use dynparser::ast;
///     let nodes = vec![
///                 ast::Node::Val("hello".to_string()),
///                 ast::Node::Val("world".to_string()),
///                 ast::Node::Val(".".to_string()),
///     ];
///     
///     let (node, nodes) = ast::split_first_nodes(&nodes).unwrap();
///     assert!(ast::get_node_val(node).unwrap() == "hello");
///     assert!(nodes.len() == 2);
///
///     let (node, nodes) = ast::split_first_nodes(&nodes).unwrap();
///     assert!(ast::get_node_val(node).unwrap() == "world");
///     assert!(nodes.len() == 1);

///     let (node, nodes) = ast::split_first_nodes(&nodes).unwrap();
///     assert!(ast::get_node_val(node).unwrap() == ".");
///     assert!(nodes.len() == 0);
///```
///
pub fn split_first_nodes(nodes: &[Node]) -> Result<(&Node, &[Node]), Error> {
    nodes
        .split_first()
        .ok_or_else(|| error("trying get first element from nodes on empty slice", None))
}

/// Consume a node if it's a Val kind and the vaule is
/// equal to the provider one
///
///```
///     use dynparser::ast;
///     let nodes = vec![
///                 ast::Node::Val("hello".to_string()),
///                 ast::Node::Val("world".to_string()),
///                 ast::Node::Val(".".to_string()),
///     ];
///     
///     let nodes = ast::consume_this_value("hello", &nodes).unwrap();
///     let nodes = ast::consume_this_value("world", &nodes).unwrap();
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

/// Consume a node if it's a Rule kind with a specific value
/// and return the rest of nodes and the sub_nodes for the consumed node
///
///```
///     use dynparser::ast;
///     let nodes = vec![
///                 ast::Node::Rule(("hello".to_string(), vec![ast::Node::Val("world".to_string())])),
///     ];
///     
///     let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("hello", &nodes).unwrap();
///     assert!(nodes.len() == 0);
///     let nodes = ast::consume_this_value("world", &sub_nodes).unwrap();
///```
///
pub fn consume_node_get_subnodes_for_rule_name_is<'a>(
    name: &str,
    nodes: &'a [Node],
) -> Result<(&'a [Node], &'a [Node]), Error> {
    let (node, nodes) = split_first_nodes(nodes)?;
    match node {
        Node::Rule((n, sub_nodes)) => {
            if n == name {
                Ok((nodes, sub_nodes))
            } else {
                Err(error(
                    &format!("expected {} node, received {}", name, n),
                    None,
                ))
            }
        }
        unknown => Err(error(
            &format!("expected {} Node::Rule, received {:?}", name, unknown),
            None,
        )),
    }
}

/// Consume a node if it's a Rule kind with a specific value
/// and return the rest of nodes and the sub_nodes for the consumed node
///
///```
///     use dynparser::ast;
///     let nodes = vec![];
///     
///     assert!(ast::check_empty_nodes(&nodes).is_ok());
///```
///
pub fn check_empty_nodes(nodes: &[Node]) -> Result<(), Error> {
    if nodes.is_empty() {
        Ok(())
    } else {
        Err(error("not consumed full nodes", None))
    }
}

/// Return a reference to first node
///
///```
///     use dynparser::ast;
///     let nodes = vec![
///                 ast::Node::Rule(("hello".to_string(), vec![])),
///                 ast::Node::Val("world".to_string())];
///     
///     let first = ast::peek_first_node(&nodes).unwrap();
///     assert!(first == &ast::Node::Rule(("hello".to_string(), vec![])));
///```
///
pub fn peek_first_node(nodes: &[Node]) -> Result<&Node, Error> {
    if nodes.is_empty() {
        Err(error("exptected node on peek_first_node", None))
    } else {
        Ok(&nodes[0])
    }
}
