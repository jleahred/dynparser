extern crate indentation_flattener;
// use indentation_flattener::flatter;

use std::collections::HashMap;

use expression::Expression;
use parsing::{Parsing, ParsingText};

mod parser;
mod parsing;
mod atom;
mod expression;

#[cfg(test)]
mod tests;


// -------------------------------------------------------------------------------------
//  T Y P E S

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct Symbol(String);

pub fn symbol(s: &str) -> Symbol {
    Symbol(s.to_owned())
}


type Rules = HashMap<Symbol, Expression>;

//  T Y P E S
// -------------------------------------------------------------------------------------


// -------------------------------------------------------------------------------------
//  A P I

pub fn parse(symbol: &Symbol, text: ParsingText, rules: &Rules) -> Result<(), String> {
    let parsed = parser::parse(symbol, Parsing::new(&text.string()), rules);
    match parsed {
        Ok(_) => Ok(()),
        Err(s) => Err(s),
    }
}

//  A P I
// -------------------------------------------------------------------------------------









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
