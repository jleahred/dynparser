#![allow(dead_code)]
// pending... remove


extern crate indentation_flattener;
// use indentation_flattener::flatter;



fn parse(parsing: Parsing, text: &ParsingText, rules: &Rules) -> Result<Parsing, String> {
    let expr = rules.get(&parsing.symbol).ok_or("undefined symbol")?;


    let parsing = match expr {
            &Expression::Terminal(ref term) => term.parse(parsing, text),
            _ => Err("Pending implementation".to_owned()),
        }
        ?;

    if parsing.position.n == text.0.len() {
        Ok(parsing)
    } else {
        Err(format!("not consumed full input"))
    }

}








use std::collections::HashMap;




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




#[derive(Debug, PartialEq)]
enum Terminal {
    Literal(String),
    Match,
    Dot,
    Symbol,
}



impl Terminal {
    fn parse(&self, parsing: Parsing, text: &ParsingText) -> Result<Parsing, String> {
        match self {
            &Terminal::Literal(ref s) => parse_literal_string(s, parsing, text),
            _ => Err("pending implementation".to_owned()),
        }
    }
}

fn parse_literal_string(s: &str,
                        mut parsing: Parsing,
                        text: &ParsingText)
                        -> Result<Parsing, String> {
    let self_len = s.len();
    let in_text = text.0.chars().skip(parsing.position.n).take(self_len).collect::<String>();
    if s == in_text {
        parsing.position.n += self_len;
        parsing.position.row += self_len;
        Ok(parsing)
    } else {
        Err("pending implementation".to_owned())
    }
}




#[derive(Debug, PartialEq, Eq, Hash, Default)]
struct Symbol(String);






#[derive(Debug, PartialEq, Default)]
struct ParsPosition {
    n: usize,
    col: usize,
    row: usize,
}


#[derive(Debug, PartialEq, Default)]
struct ParsingText(String);
impl ParsingText {
    fn new(txt: &str) -> Self {
        ParsingText(txt.to_owned())
    }
}


#[derive(Default)]
struct Parsing {
    position: ParsPosition,
    symbol: Symbol,
    parsing_text: ParsingText,
}


impl Parsing {
    fn new(symbol: &str) -> Self {
        Parsing { symbol: Symbol(symbol.to_owned()), ..Parsing::default() }
    }
}








macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);


macro_rules! parser(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert(Symbol($key.to_owned()), $value);
            )+
            m
        }
     };
);


macro_rules! lit(
    { $value:expr } => {
        Expression::Terminal(Terminal::Literal($value.to_owned()))
     };
);






#[test]
fn macro_expr() {
    let parser = map!{ 
            Symbol("main".to_owned()) => 
                Expression::Terminal(
                    Terminal::Literal("aaaa".to_owned()))};
    let parser2 = parser!{ 
            "main" => 
                Expression::Terminal(
                    Terminal::Literal("aaaa".to_owned()))
    };
    let parser3 = parser!{ 
            "main" => lit!("aaaa")
    };
    assert!(parser == parser2);
    assert!(parser == parser3);
}




#[test]
fn validate_literal() {
    let grammar = &parser!{ 
            "main" => lit!("aaaa")
    };
    assert!(parse(Parsing::new("main"),
                       &ParsingText::new("aaaa"),
                       grammar).is_ok());
    assert!(parse(Parsing::new("main"),
                       &ParsingText::new("aaa"),
                       grammar).is_err());
    assert!(parse(Parsing::new("main"),
                       &ParsingText::new("bbbb"),
                       grammar).is_err());
    assert!(parse(Parsing::new("main"),
                       &ParsingText::new("aaaaa"),
                       grammar).is_err());
    assert!(parse(Parsing::new("main"),
                       &ParsingText::new(""),
                       grammar).is_err());
}
