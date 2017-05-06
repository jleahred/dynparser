use atom::Atom;
use parser::Parse;
use {parser, Text2Parse, Error, error};


#[derive(Debug, PartialEq)]
pub enum Expression {
    Simple(Atom),
    Or(MultiExpr),
    And(MultiExpr),
}



#[derive(Debug, PartialEq)]
pub struct MultiExpr(pub Vec<Expression>);





impl Parse for Expression {
    fn parse(&self,
             text2parse: &Text2Parse,
             pars_pos: parser::Possition)
             -> Result<parser::Possition, Error> {
        match self {
            &Expression::Simple(ref atom) => atom.parse(text2parse, pars_pos),
            &Expression::Or(MultiExpr(ref exprs)) => parse_or(text2parse, exprs, pars_pos),
            &Expression::And(MultiExpr(ref exprs)) => parse_and(text2parse, exprs, pars_pos),
        }
    }
}


fn parse_or(text2parse: &Text2Parse,
            exprs: &Vec<Expression>,
            pars_pos: parser::Possition)
            -> Result<parser::Possition, Error> {

    let mut deep_error: Option<Error> = None;
    for e in exprs {
        match e.parse(text2parse, pars_pos.clone()) {
            Ok(p) => return Ok(p),
            Err(error) => deep_error = ::deep_error(&deep_error, &error),
        }
    }

    Err(error(&pars_pos,
              "failed on or, pending getting best option for error message"))
}


fn parse_and(text2parse: &Text2Parse,
             exprs: &Vec<Expression>,
             pars_pos: parser::Possition)
             -> Result<parser::Possition, Error> {
    let mut parst = pars_pos.clone();
    for e in exprs {
        parst = e.parse(text2parse, parst.clone())?;
    }
    Ok(parst)
}
