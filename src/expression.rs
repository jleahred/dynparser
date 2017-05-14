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
    let mut errs = vec![];
    for e in exprs {
        match e.parse(conf, status.clone()) {
            Ok(p) => return Ok(p),
            Err(perr) => errs.push(error(&perr.pos, &format!("{}", perr.descr))),
        }
    }

    let mut error = error(&status.pos, "\nbegin parsing or");
    let max_deep = errs.iter().fold(0, |acc, e| ::std::cmp::max(acc, e.pos.n));

    for e in errs {
        if e.pos.n == max_deep {
            error.descr = format!("{}\n{}", error.descr, e);
        }
    }
    error.descr = format!("{}end parsing or", error.descr);

    Err(error)
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
                min: &NRep,
                omax: &Option<NRep>)
                -> Result<parser::Status, Error> {
    let max_reached = |i| omax.as_ref().map_or(false, |ref m| i + 1 >= m.0);
    let last_ok_or =
        |lok: Option<parser::Status>, ref status| lok.as_ref().unwrap_or(&status).clone();

    let mut opt_lastokst = None;
    for i in 0.. {
        let st = last_ok_or(opt_lastokst.clone(), status.clone());
        let last_result = expr.parse(conf, st);

        opt_lastokst = last_result.clone().ok().or(opt_lastokst);

        match (i >= min.0, max_reached(i), last_result, opt_lastokst.clone()) {
            (false, _, Err(err), _) => {
                return Err(error(&status.pos, &format!("not enougth repetitions. {}", err)))
            }
            (true, true, _, Some(lok)) => return Ok(lok),
            (true, true, _, None) => return Ok(status),
            (false, _, Ok(_), _) => (),
            (true, false, Ok(_), _) => (),
            (true, false, Err(_), Some(lok)) => return Ok(lok),
            (true, false, Err(_), None) => return Ok(status),
        }
    }
    Err(error(&status.pos, "stupid line waitting for #37339"))
}
