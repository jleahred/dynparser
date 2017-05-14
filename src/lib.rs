// todo: ast
//  before parsing, check if rules are complete
//  no missing rules, no defined but not used rules
//  remove indentation reference???



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
}




//  T Y P E S
// -------------------------------------------------------------------------------------


// -------------------------------------------------------------------------------------
//  A P I

pub fn parse(text2parse: &Text2Parse, symbol: &Symbol, rules: &Rules) -> Result<(), Error> {
    let config = parser::Config {
        text2parse: text2parse,
        rules: rules,
    };
    let parsed = parser::parse(&config, symbol, parser::Status::new());
    match parsed {
        Ok(_) => Ok(()),
        Err(s) => Err(s),
    }
}

//  A P I
// -------------------------------------------------------------------------------------





fn error(pos: &parser::Possition, descr: &str) -> Error {
    Error {
        pos: pos.clone(),
        descr: descr.to_owned(),
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

fn deep_error(err1: &Option<Error>, err2: &Error) -> Error {
    let mut result = match err1 {
        &Some(ref error) => {
            use std::cmp::Ordering::{Equal, Less, Greater};
            match error.pos.cmp(&err2.pos) {
                Equal => {
                    Error { descr: format!("{} {}", error.descr, err2.descr), ..error.clone() }
                }
                Greater => error.clone(),
                Less => err2.clone(),
            }
        }
        &None => err2.clone(),
    };

    result.descr = truncate_error_msg(result.descr);
    result
}

fn add_descr_error(mut error: Error, descr: &str) -> Error {
    error.descr = format!("{} > {}", descr, error.descr);
    error.descr = truncate_error_msg(error.descr);
    error
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = write!(f,
                             "in pos: r:{}, c:{}, n:{}   -> ",
                             self.pos.row,
                             self.pos.col,
                             self.pos.n);

        for line in self.descr.lines() {
            if line.is_empty() == false {
                res = write!(f, "    {}\n", line);
            }
        }
        res
    }
}


//  pending remove...
pub use grammar::grammar;