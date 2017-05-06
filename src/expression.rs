use atom::Atom;
use parser::Parse;
use {parser, Error, error};


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
             pars_conf: &parser::Config,
             pars_pos: parser::Possition)
             -> Result<parser::Possition, Error> {
        match self {
            &Expression::Simple(ref atom) => atom.parse(pars_conf, pars_pos),
            &Expression::Or(MultiExpr(ref exprs)) => parse_or(pars_conf, exprs, pars_pos),
            &Expression::And(MultiExpr(ref exprs)) => parse_and(pars_conf, exprs, pars_pos),
        }
    }
}


fn parse_or(pars_conf: &parser::Config,
            exprs: &Vec<Expression>,
            pars_pos: parser::Possition)
            -> Result<parser::Possition, Error> {

    let mut deep_error: Option<Error> = None;
    for e in exprs {
        match e.parse(pars_conf, pars_pos.clone()) {
            Ok(p) => return Ok(p),
            Err(error) => deep_error = ::deep_error(&deep_error, &error),
        }
    }

    Err(error(&pars_pos,
              "failed on or, pending getting best option for error message"))
}


fn parse_and(pars_conf: &parser::Config,
             exprs: &Vec<Expression>,
             pars_pos: parser::Possition)
             -> Result<parser::Possition, Error> {
    let mut parst = pars_pos.clone();
    for e in exprs {
        parst = e.parse(pars_conf, parst.clone())?;
    }
    Ok(parst)
}
