// error with description possition, expected values


extern crate indentation_flattener;
// use indentation_flattener::flatter;

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
pub struct Symbol(String);

pub fn symbol(s: &str) -> Symbol {
    Symbol(s.to_owned())
}

#[derive(Debug, PartialEq, Default)]
pub struct Text2Parse(String);

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
    let parsed = parser::parse(&text2parse, symbol, parser::Possition::new(), rules);
    match parsed {
        Ok(_) => Ok(()),
        Err(s) => Err(s),
    }
}

//  A P I
// -------------------------------------------------------------------------------------




impl Text2Parse {
    pub fn new(txt: &str) -> Self {
        Text2Parse(txt.to_owned())
    }
    pub fn string(&self) -> &String {
        &self.0
    }
}

fn error(pos: &parser::Possition, descr: &str) -> Error {
    Error {
        pos: pos.clone(),
        descr: descr.to_owned(),
    }
}


fn deep_error(err1: &Option<Error>, err2: &Error) -> Option<Error> {
    match err1 {
        &Some(ref error) => {
            match error.pos >= err2.pos {
                true => Some(error.clone()),
                false => Some(err2.clone()),
            }
        }
        &None => Some(err2.clone()),
    }
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
