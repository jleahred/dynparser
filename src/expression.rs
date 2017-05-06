use atom::Atom;
use parser::Parse;
use {parser, Error, error};


#[derive(Debug, PartialEq)]
pub enum Expression {
    Simple(Atom),
    Or(MultiExpr),
    And(MultiExpr),
    Not(Box<Expression>),
}



#[derive(Debug, PartialEq)]
pub struct MultiExpr(pub Vec<Expression>);





impl Parse for Expression {
    fn parse(&self,
             conf: &parser::Config,
             status: parser::Status)
             -> Result<parser::Status, Error> {
        match self {
            &Expression::Simple(ref atom) => atom.parse(conf, status),
            &Expression::Or(MultiExpr(ref exprs)) => parse_or(conf, exprs, status),
            &Expression::And(MultiExpr(ref exprs)) => parse_and(conf, exprs, status),
            &Expression::Not(ref exprs) => parse_negate(conf, exprs, status),
        }
    }
}


fn parse_or(conf: &parser::Config,
            exprs: &Vec<Expression>,
            status: parser::Status)
            -> Result<parser::Status, Error> {

    let mut deep_error: Option<Error> = None;
    for e in exprs {
        match e.parse(conf, status.clone()) {
            Ok(p) => return Ok(p),
            Err(error) => deep_error = Some(::deep_error(&deep_error, &error)),
        }
    }

    match deep_error {
        Some(err) => Err(err),
        None => Err(error(&status.pos, "emtpy or???")),
    }
}


fn parse_and(conf: &parser::Config,
             exprs: &Vec<Expression>,
             status: parser::Status)
             -> Result<parser::Status, Error> {
    let mut parst = status.clone();
    for e in exprs {
        parst = e.parse(conf, parst.clone())?;
    }
    Ok(parst)
}


fn parse_negate(conf: &parser::Config,
                expr: &Expression,
                status: parser::Status)
                -> Result<parser::Status, Error> {

    match expr.parse(conf, status.clone()) {
        Ok(result) => Err(error(&result.pos, "negation error")),
        Err(_) => Ok(status),
    }
}
