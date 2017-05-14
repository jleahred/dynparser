// todo: ast
//  before parsing, check if rules are complete
//  no missing rules, no defined but not used rules
//  remove indentation reference???
//  update line and column when parsing
//  remove error not consume all input, let it error on pos...
//  let symbols with any char



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


fn add_descr_error(mut error: Error, descr: &str) -> Error {
    error.descr = format!("{} > {}", descr, error.descr);
    error.descr = truncate_error_msg(error.descr);
    error
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = write!(f,
                       "in pos: r:{}, c:{}, n:{}   -> ",
                       self.pos.row,
                       self.pos.col,
                       self.pos.n);

        for line in self.descr.lines() {
            if line.is_empty() == false {
                let _ = write!(f, "    {}\n", line);
            }
        }
        Ok(())
    }
}


//  pending remove...
pub use grammar::grammar;