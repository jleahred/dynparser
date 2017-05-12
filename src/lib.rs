// todo: ast
//  before parsing, check if rules are complete
//  no missing rules, no defined but not used rules

const TRUNCATE_ERROR: usize = 100;

extern crate indentation_flattener;

use std::collections::HashMap;

use expression::Expression;
mod parser;
mod atom;
mod expression;

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

#[derive(Debug, PartialEq, Default, Clone)]
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

    let result_len = result.descr.len();
    if result_len > TRUNCATE_ERROR {
        result.descr = format!("...{}",
                               result.descr
                                   .chars()
                                   .skip(result_len - TRUNCATE_ERROR)
                                   .take(TRUNCATE_ERROR)
                                   .collect::<String>());
    };
    result
}

fn add_descr_error(mut error: Error, descr: &str) -> Error {
    error.descr = format!("{} > {}", descr, error.descr);
    error
}






// include!(concat!(env!("OUT_DIR"), "/dinpeg.rs"));






// #[test]
// fn validate_grammars() {
//     let validate = parse("main = aaaa");
//     assert!(validate == Ok(()));

//     let validate = parse(&flatter(r#"
//         main = def
//         def
//             = "def" _ func_name _ "(" _ params _ "):" _ eol+
//             |           body

//         func_name = id

//         id  = [A-Za-z][A-Za-z0-9_]*
//         eol = "\n"
//         _   = " "*

//         // body = $INDENT(stament*)

//         stament
//             = expr
//             / if

//         if
//             =  "if" _ expr _ ":" _
//             |        body
//             |  "else:" _
//             |        body
// "#)
//         .unwrap()
//         .0);
//     println!("{:?} ____________", validate);
//     assert!(validate == Ok(()));

// }
