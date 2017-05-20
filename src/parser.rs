use {Symbol, Rules, Text2Parse, Error, error, AST};
use parser;
use atom::parse_symbol;


pub struct Config<'a> {
    pub text2parse: &'a Text2Parse,
    pub rules: &'a Rules,
}




pub trait Parse {
    fn parse(&self,
             conf: &Config,
             status: parser::Status)
             -> Result<(parser::Status, AST::Node), Error>;
}



pub fn parse(conf: &Config,
             symbol: &Symbol,
             status: parser::Status)
             -> Result<(parser::Status, AST::Node), Error> {

    let final_status = parse_symbol(conf, symbol, status)?;

    match final_status.0.pos.n == conf.text2parse.0.len() {
        true => Ok(final_status),
        false => {
            Err(error(&final_status.0.pos,
                      &format!("unexpected >{}<",
                               conf.text2parse
                                   .0
                                   .chars()
                                   .skip(final_status.0.pos.n)
                                   .take(conf.text2parse.0.len() - final_status.0.pos.n)
                                   .collect::<String>()),
                      conf.text2parse))
        }
    }
}


#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Possition {
    pub n: usize,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Depth(pub u32);

#[derive(Debug, PartialEq, Clone)]
pub struct Status {
    pub pos: Possition,
    pub depth: Depth,
}


impl Status {
    pub fn new() -> Self {
        Status {
            pos: Possition::new(),
            depth: Depth(0),
        }
    }

    pub fn inc_depth(&self) -> Self {
        Status { depth: Depth(self.depth.0 + 1), ..self.clone() }
    }
}



impl Possition {
    pub fn new() -> Self {
        Possition {
            n: 0,
            col: 1,
            row: 1,
        }
    }
    fn inc_ch(&mut self, ch: char) -> &Self {
        match ch {
            '\n' => {
                self.n += 1;
                self.col = 0;
                self.row += 1;
            }
            _ => {
                self.n += 1;
                self.col += 1;
            }
        };
        self
    }
    pub fn inc_char(&mut self, text2parse: &Text2Parse) -> &Self {
        let n = self.n;
        self.inc_ch(text2parse.0.chars().nth(n).unwrap_or('?'))
    }
    pub fn inc_chars(&mut self, s: &str) -> &Self {
        for ch in s.chars() {
            self.inc_ch(ch);
        }
        self
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

    pub fn match_ch(chars: &str, ranges: Vec<(char, char)>) -> Expression {
        Expression::Simple(Atom::Match(chars.to_owned(), ranges))
    }

}