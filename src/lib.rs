// todo: ast
//  ast on it's own file
//  before parsing, check if rules are complete
//  no missing rules, no defined but not used rules
//  remove indentation reference???
//  let symbols with any char
//  generate code for parsing (grammar.rs)
//  prune with a lambda
//  extend grammar to deal better with errors (error result)
//  add in status last deep error to deal with not consumed all input error
//      by example...  h=a (b
//  test and verify depp control
//  remove not necessary dependencies


const TRUNCATE_ERROR: usize = 2000;

// extern crate indentation_flattener;

use std::collections::HashMap;

use expression::Expression;
mod parser;
mod atom;
mod expression;
pub mod grammar;



#[cfg(test)]
mod tests;


// -------------------------------------------------------------------------------------
//  T Y P E S



#[allow(non_snake_case)]
pub mod AST {
    #[derive(Debug)]
    pub struct K(pub String);

    #[derive(Debug)]
    pub struct V(pub String);

    #[derive(Debug)]
    pub struct Node {
        pub kind: K,
        pub val: V,
        pub nodes: Box<Vec<Node>>,
    }

    impl Node {
        pub fn new(kind: K, val: V) -> Self {
            Node {
                kind: kind,
                val: val,
                nodes: Box::new(vec![]),
            }
        }
        pub fn merge(mut self, nwnode: Node) -> Self {
            self.nodes.push(nwnode);
            self
        }

        #[must_use]
        pub fn get_pruned(mut self) -> Self {
            fn to_be_pruned(node: &Node) -> bool {
                let prune_kind = match node.kind.0.as_ref() {
                    "repeat" => true,
                    "and" => true,
                    // "lit" => true,
                    "match" => true,
                    "atom" => true,
                    "compl_expr" => true,
                    _ => false,
                };
                let prune_val = match node.val.0.as_ref() {
                    "_" => true,
                    "or_expr" => true,
                    "and_expr" => true,
                    "compl_expr" => true,
                    _ => false,
                };
                prune_kind || prune_val
            };


            let mut located_prune = true;
            while located_prune {
                located_prune = false;
                let mut new_nodes = Box::new(vec![]);
                self.nodes.reverse();
                while let Some(mut node) = self.nodes.pop() {
                    match to_be_pruned(&node) {
                        true => {
                            located_prune = true;
                            node.nodes.reverse();
                            while let Some(n) = node.nodes.pop() {
                                new_nodes.push(n)
                            }
                        }
                        false => new_nodes.push(node.get_pruned()),
                    }
                }
                self.nodes = new_nodes;
            }

            self
        }
    }

    pub fn from_strs(k: &str, v: &str) -> Node {
        Node::new(K(k.to_owned()), V(v.to_owned()))
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct Symbol(pub String);

pub fn symbol(s: &str) -> Symbol {
    Symbol(s.to_owned())
}

#[derive(Debug, PartialEq, Default)]
pub struct Text2Parse(pub String);

pub fn text2parse(txt: &str) -> Text2Parse {
    Text2Parse(txt.to_owned())
}


type Rules = HashMap<Symbol, Expression>;

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub pos: parser::Possition,
    pub descr: String,
    pub line_text: String,
}




//  T Y P E S
// -------------------------------------------------------------------------------------


// -------------------------------------------------------------------------------------
//  A P I

pub fn parse(text2parse: &Text2Parse, symbol: &Symbol, rules: &Rules) -> Result<AST::Node, Error> {
    let config = parser::Config {
        text2parse: text2parse,
        rules: rules,
    };
    let parsed = parser::parse(&config, symbol, parser::Status::new());
    match parsed {
        Ok((_, ast_node)) => Ok(ast_node.get_pruned()),
        Err(s) => Err(s),
    }
}

//  A P I
// -------------------------------------------------------------------------------------



pub fn get_begin_line_pos(pos: &parser::Possition, text2parse: &Text2Parse) -> String {
    text2parse.0
        .chars()
        .take(pos.n)
        .collect::<String>()
        .chars()
        .rev()
        .take_while(|ch| *ch != '\n')
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}

//  pending
fn error(pos: &parser::Possition, descr: &str, text2parse: &Text2Parse) -> Error {
    Error {
        pos: pos.clone(),
        descr: descr.to_owned(),
        line_text: get_begin_line_pos(pos, text2parse),
    }
}



fn truncate_error_msg(mut err_msg: String) -> String {
    let result_len = err_msg.len();
    if result_len > TRUNCATE_ERROR {
        err_msg = format!("...{}",
                          err_msg.chars()
                              .skip(result_len - TRUNCATE_ERROR)
                              .take(TRUNCATE_ERROR)
                              .collect::<String>());
    };
    err_msg
}


fn add_descr_error(mut error: Error, descr: &str) -> Error {
    error.descr = format!("{} > {}", descr, error.descr);
    error.descr = truncate_error_msg(error.descr);
    error
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = write!(f,
                       "in pos: r:{}, c:{}, n:{}   >{}<  -> {}",
                       self.pos.row,
                       self.pos.col,
                       self.pos.n,
                       self.line_text,
                       self.descr);
        Ok(())
    }
}

impl Error {
    fn descr_indented(&self) -> String {
        let mut r = String::new();
        for line in self.descr.lines() {
            if line.is_empty() == false {
                r = format!("{}\n    {}", r, line);
            }
        }
        r
    }
}



//  pending remove...
pub use grammar::grammar;



#[test]
fn prune_emtpy_root() {
    use AST::{Node, K, V};
    let root = Node::new(K("root".to_owned()), V("".to_owned()));
    let pruned = root.get_pruned();

    assert!(format!("{:?}", pruned) == r#"Node { kind: K("root"), val: V(""), nodes: [] }"#);
}

#[test]
fn prune_nothing_to_prune() {
    use AST::{Node, K, V};
    let root = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("child".to_owned()),
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };
    let pruned = root.get_pruned();

    assert!(format!("{:?}", pruned) ==
        r#"Node { kind: K("root"), val: V(""), nodes: [Node { kind: K("child"), val: V("aaa"), nodes: [] }] }"#);
}


#[test]
fn prune_repeat_no_child() {
    use AST::{Node, K, V};
    let root = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("repeat".to_owned()),
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };
    let pruned = root.get_pruned();

    assert!(format!("{:?}", pruned) == r#"Node { kind: K("root"), val: V(""), nodes: [] }"#);
}


#[test]
fn prune_repeat_one_child() {
    use AST::{Node, K, V};
    let root = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("repeat".to_owned()),
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K("aaa".to_owned()),
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             }]),
    };
    let pruned = root.get_pruned();

    let result = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("aaa".to_owned()),
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}


#[test]
fn prune_repeat_two_child() {
    use AST::{Node, K, V};
    let root = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("repeat".to_owned()),
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K("aaa".to_owned()),
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      },
                                                      Node {
                                                          kind: K("bbb".to_owned()),
                                                          val: V("bbb".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             }]),
    };
    let pruned = root.get_pruned();

    let result = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("aaa".to_owned()),
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             },
                             Node {
                                 kind: K("bbb".to_owned()),
                                 val: V("bbb".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}

#[test]
fn prune_repeat_two_child_two_grandchild() {
    use AST::{Node, K, V};
    let root = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("repeat".to_owned()),
                                 val: V("".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K("aaa".to_owned()),
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![Node {
                                                          kind: K("ab".to_owned()),
                                                          val: V("ab".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      },
                                                      Node {
                                                          kind: K("bc".to_owned()),
                                                          val: V("bc".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                                                      },
                                                      Node {
                                                          kind: K("bbb".to_owned()),
                                                          val: V("bbb".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             }]),
    };
    let pruned = root.get_pruned();

    let result = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("aaa".to_owned()),
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![Node {
                                                          kind: K("ab".to_owned()),
                                                          val: V("ab".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      },
                                                      Node {
                                                          kind: K("bc".to_owned()),
                                                          val: V("bc".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                             },
                             Node {
                                 kind: K("bbb".to_owned()),
                                 val: V("bbb".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}


#[test]
fn prune_two_nested_repeat_one_child() {
    use AST::{Node, K, V};
    let root =
        Node {
            kind: K("root".to_owned()),
            val: V("".to_owned()),
            nodes: Box::new(vec![Node {
                                     kind: K("repeat".to_owned()),
                                     val: V("".to_owned()),
                                     nodes: Box::new(vec![Node {
                                                              kind: K("repeat".to_owned()),
                                                              val: V("".to_owned()),
                                                              nodes: Box::new(vec![Node {
                                                          kind: K("aaa".to_owned()),
                                                          val: V("aaa".to_owned()),
                                                          nodes: Box::new(vec![]),
                                                      }]),
                                                          }]),
                                 }]),
        };
    let pruned = root.get_pruned();

    let result = Node {
        kind: K("root".to_owned()),
        val: V("".to_owned()),
        nodes: Box::new(vec![Node {
                                 kind: K("aaa".to_owned()),
                                 val: V("aaa".to_owned()),
                                 nodes: Box::new(vec![]),
                             }]),
    };

    println!("{:?} ____________", pruned);
    assert!(format!("{:?}", pruned) == format!("{:?}", result));
}
