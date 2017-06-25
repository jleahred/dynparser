



const TRUNCATE_ERROR: usize = 2000;

// extern crate indentation_flattener;

use std::collections::HashMap;

mod parser;
mod atom;
mod expression;
pub mod ast;
pub mod grammar;


use expression::Expression;




#[cfg(test)]
mod tests;


// -------------------------------------------------------------------------------------
//  T Y P E S

//  see ast.rs

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

pub fn parse(text2parse: &Text2Parse, symbol: &Symbol, rules: &Rules) -> Result<ast::Node, Error> {
    // let check2prune = |kind: &ast::K, val: &str| {
    //     let prune_kind = match kind {
    //         &ast::K::ERepeat => true,
    //         &ast::K::EAnd => true,
    //         // &ast::K::a_lit => true,
    //         // &ast::K::AMatch => true,
    //         _ => false,
    //     };
    //     let prune_val = match val {
    //         // "_" => true,
    //         // "or_expr" => true,
    //         // "and_expr" => true,
    //         // "compl_expr" => true,
    //         _ => false,
    //     };
    //     prune_kind || prune_val

    // };
    let config = parser::Config {
        text2parse: text2parse,
        rules: rules,
    };
    let parsed = parser::parse(&config, symbol, parser::Status::new());
    match parsed {
        // Ok((_, ast_node)) => Ok(ast_node.get_pruned(&check2prune)),
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
