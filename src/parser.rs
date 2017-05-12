use {Symbol, Rules, Text2Parse, Error, error};
use parser;


pub struct Config<'a> {
    pub text2parse: &'a Text2Parse,
    pub rules: &'a Rules,
}




pub trait Parse {
    fn parse(&self, conf: &Config, status: parser::Status) -> Result<parser::Status, Error>;
}



pub fn parse(conf: &Config, symbol: &Symbol, status: parser::Status) -> Result<Status, Error> {
    let status = conf.rules
        .get(symbol)
        .ok_or(error(&status.pos, &format!("undefined symbol {:?}", symbol)))?
        .parse(conf, status)?;

    match status.pos.n == conf.text2parse.0.len() {
        true => Ok(status),
        false => Err(error(&status.pos, "not consumed full input")),
    }
}


#[derive(Debug, PartialEq, Default, Clone, Eq, PartialOrd, Ord)]
pub struct Possition {
    pub n: usize,
    pub col: usize,
    pub row: usize,
}

#[derive(Debug, PartialEq, Default, Clone, PartialOrd)]
pub struct Depth(pub u32);

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Status {
    pub pos: Possition,
    pub depth: Depth,
    pub deep_error: Option<Error>,
}


impl Status {
    pub fn new() -> Self {
        Status {
            pos: Possition::new(),
            depth: Depth(0),
            deep_error: None,
        }
    }
}



impl Possition {
    pub fn new() -> Self {
        Possition { ..Possition::default() }
    }
}


pub mod tools {

    pub use atom::Atom;
    pub use expression::{Expression, MultiExpr, NRep};


    pub fn lit(s: &str) -> Expression {
        Expression::Simple(Atom::Literal(s.to_owned()))
    }

    pub fn dot() -> Expression {
        Expression::Simple(Atom::Dot)
    }

    // fn nothing() -> Expression {
    //     Expression::Simple(Atom::Nothing)
    // }

    pub fn or(exp_list: Vec<Expression>) -> Expression {
        Expression::Or(MultiExpr(exp_list))
    }

    pub fn and(exp_list: Vec<Expression>) -> Expression {
        Expression::And(MultiExpr(exp_list))
    }

    pub fn symref(s: &str) -> Expression {
        Expression::Simple(Atom::Symbol(s.to_owned()))
    }

    pub fn not(expr: Expression) -> Expression {
        Expression::Not(Box::new(expr))
    }

    pub fn repeat(expr: Expression, min: NRep, max: Option<NRep>) -> Expression {
        Expression::Repeat(Box::new(expr), min, max)
    }

}