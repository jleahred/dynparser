use atom::Atom;
use parser::Parse;
use {parser, Error, error};


#[derive(Debug)]
pub enum Expression {
    Simple(Atom),
    Or(MultiExpr),
    And(MultiExpr),
    Not(Box<Expression>),
    Repeat(Box<Expression>, NRep, Option<NRep>), //  min max
}


#[derive(Debug)]
pub struct NRep(pub u32);


#[derive(Debug)]
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
            &Expression::Repeat(ref exprs, ref min, ref max) => {
                parse_repeat(conf, exprs, status, min, max)
            }
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

fn parse_repeat(conf: &parser::Config,
                expr: &Expression,
                status: parser::Status,
                min: NRep,
                omax: Option<NRep>)
                -> Result<parser::Status, Error> {

    let parse_repeat_max = |max| 1;
    let parse_repeat_no_limit = || 1;

    let (repeated, end_status) = match omax {
        Some(max) => parse_repeat_max(max),
        None => parse_repeat_no_limit(),
    };

    match repeated.0 >= min.0 {
        true => Ok(status),
        false => Err(error(&status.pos, &format!("not enought repetitions {:?}", expr))),
    }
}
