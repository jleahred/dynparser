// todo: ast
//  before parsing, check if rules are complete
//  no missing rules, no defined but not used rules
//  remove indentation reference???
//  update line and column when parsing
//  remove error not consume all input, let it error on pos...
//  let symbols with any char
//  ast on it's own file



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
    pub struct Kind(pub String);

    #[derive(Debug)]
    pub struct Val(pub String);

    #[derive(Debug)]
    pub struct Node {
        pub kind: Kind,
        pub val: Val,
        pub nodes: Box<Vec<Node>>,
    }

    impl Node {
        pub fn new(kind: Kind, val: Val) -> Self {
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
        // pub fn prune(mut self) -> Self {
        //     // match self.nodes.pop() {
        //     //     Some(mut child) => {
        //     //         if child.kind.0 == "" {
        //     //             match child.nodes.pop() {
        //     //                 Some(childchild) => self.nodes = childchild.nodes,
        //     //                 _ => (),
        //     //             }
        //     //         };
        //     //         child.prune();
        //     //     }
        //     //     _ => (),
        //     // };
        //     self
        // }
    }
    pub fn from_strs(k: &str, v: &str) -> Node {
        Node::new(Kind(k.to_owned()), Val(v.to_owned()))
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
        Ok((_, ast_node)) => Ok(ast_node),
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