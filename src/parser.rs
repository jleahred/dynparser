#![allow(dead_code)]
// pending... remove

use std::collections::HashMap;

extern crate indentation_flattener;
// use indentation_flattener::flatter


use terminal::Terminal;


// fn parse(symbol: &Symbol, parsing: Parsing, rules: &Rules) -> Result<Parsing, String> {
//     let expr = rules.get(symbol).ok_or("undefined symbol")?;


//     let parsing = match expr {
//         &Expression::Terminal(ref term) => term.parse(parsing),
//         _ => Err("Pending implementation".to_owned()),
//     }?;

//     if parsing.position.n == parsing.parsing_text.0.len() {
//         Ok(parsing)
//     } else {
//         Err(format!("not consumed full input"))
//     }

// }


fn symbol(s: &str) -> Symbol {
    Symbol(s.to_owned())
}











type Rules = HashMap<Symbol, Expression>;


#[derive(Debug, PartialEq)]
enum Expression {
    NonTerminal(OrExpr),
    Terminal(Terminal),
}




#[derive(Debug, PartialEq)]
struct OrExpr {
    seq_expr: Vec<SeqExpr>,
}




#[derive(Debug, PartialEq)]
struct SeqExpr {
    par_expr: Vec<OrExpr>,
}







#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
struct Symbol(String);






#[derive(Debug, PartialEq, Default)]
pub struct ParsingPossition {
    pub n: usize,
    pub col: usize,
    pub row: usize,
}


#[derive(Debug, PartialEq, Default)]
pub struct ParsingText(String);
impl ParsingText {
    pub fn new(txt: &str) -> Self {
        ParsingText(txt.to_owned())
    }
    pub fn string(&self) -> String {
        self.0.clone()
    }
}


#[derive(Default, Debug, PartialEq)]
pub struct Parsing {
    pub position: ParsingPossition,
    pub parsing_text: ParsingText,
}


impl Parsing {
    pub fn new(s: &str) -> Self {
        Parsing { parsing_text: ParsingText(s.to_owned()), ..Parsing::default() }
    }


    // pub fn set_text(mut self, text: &str) -> Self {
    //     self.parsing_text = ParsingText(text.to_owned());
    //     self
    // }
}








// macro_rules! map(
//     { $($key:expr => $value:expr),+ } => {
//         {
//             let mut m = ::std::collections::HashMap::new();
//             $(
//                 m.insert($key, $value);
//             )+
//             m
//         }
//      };
// );


// macro_rules! parser(
//     { $($key:expr => $value:expr),+ } => {
//         {
//             let mut m = ::std::collections::HashMap::new();
//             $(
//                 m.insert(Symbol($key.to_owned()), $value);
//             )+
//             m
//         }
//     };
// );




// macro_rules! lit(
//     { $value:expr } => {
//         Expression::Terminal(Terminal::Literal($value.to_owned()))
//     };
// );


// macro_rules! dot(
//     { } => {
//         Expression::Terminal(Terminal::Dot)
//     };
// );






// #[test]
// fn macro_expr() {
//     let parser = map!{
//             Symbol("main".to_owned()) =>
//                 Expression::Terminal(
//                     Terminal::Literal("aaaa".to_owned()))};
//     let parser2 = parser!{
//             "main" =>
//                 Expression::Terminal(
//                     Terminal::Literal("aaaa".to_owned()))
//     };
//     let parser3 = parser!{
//             "main" => lit!("aaaa")
//     };
//     assert!(parser == parser2);
//     assert!(parser == parser3);
// }
