use atom::Atom;
use parser::Parse;
use {parser, Error, error};
use ast;


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
             -> Result<(parser::Status, ast::Node), Error> {
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
            -> Result<(parser::Status, ast::Node), Error> {
    let mut errs = vec![];
    for e in exprs {
        match e.parse(conf, status.clone()) {
            Ok(p) => return Ok(p),
            Err(perr) => errs.push(error(&perr.pos, &perr.descr, conf.text2parse)),
        }
    }

    let max_deep = errs.iter().fold(0, |acc, e| ::std::cmp::max(acc, e.pos.n));
    errs.retain(|ref e| e.pos.n == max_deep);

    if errs.len() == 1 {
        Err(errs[0].clone())    //  [0]  it's safe
    } else {
        let mut error = error(&status.pos, "", conf.text2parse);
        for e in errs {
            if e.pos.n == max_deep {
                error.descr = format!("{}  {}", error.descr, e.descr_indented());
                error.pos = e.pos;
            }
        }
        // error.descr = format!("{}end parsing or", error.descr);
        Err(error)
    }
}


fn parse_and(conf: &parser::Config,
             exprs: &Vec<Expression>,
             status: parser::Status)
             -> Result<(parser::Status, ast::Node), Error> {
    let ast = |ast_nodes| {
        ast::Node {
            kind: ast::K::EAnd,
            val: ast::V("".to_owned()),
            nodes: Box::new(ast_nodes),
        }
    };

    let mut parst = status.clone();
    let mut ast_nodes = vec![];
    for e in exprs {
        let (nw_st, ast) = e.parse(conf, parst.clone())?;
        parst = nw_st;
        ast_nodes.push(ast);
    }
    Ok((parst, ast(ast_nodes)))
}


fn parse_negate(conf: &parser::Config,
                expr: &Expression,
                status: parser::Status)
                -> Result<(parser::Status, ast::Node), Error> {

    match expr.parse(conf, status.clone()) {
        Ok(result) => Err(error(&result.0.pos, "negation error", conf.text2parse)),
        Err(_) => Ok((status, ast::Node::new_valstr(ast::K::ENot, ""))),
    }
}

fn parse_repeat(conf: &parser::Config,
                expr: &Expression,
                status: parser::Status,
                min: &NRep,
                omax: &Option<NRep>)
                -> Result<(parser::Status, ast::Node), Error> {
    let ast = |ast_nodes| {
        ast::Node {
            kind: ast::K::ERepeat,
            val: ast::V("".to_owned()),
            nodes: ast_nodes,
        }
    };
    let max_reached = |i| omax.as_ref().map_or(false, |ref m| i + 1 >= m.0);
    let last_ok_or =
        |lok: Option<parser::Status>, ref status| lok.as_ref().unwrap_or(&status).clone();

    let mut opt_lastokst = None;
    let mut opt_lasterror = None;
    let mut ast_nodes = Box::new(vec![]);
    for i in 0.. {
        let st = last_ok_or(opt_lastokst.clone(), status.clone());
        let last_result = expr.parse(conf, st);

        match last_result {
            Ok((st, ast_node)) => {
                opt_lastokst = Some(st);
                ast_nodes.push(ast_node);
            }
            Err(err) => opt_lasterror = Some(err),
        }

        match (i >= min.0, max_reached(i), opt_lasterror.clone(), opt_lastokst.clone()) {
            (false, _, Some::<Error>(err), _) => {
                return Err(error(&err.pos,
                                 &format!("trying repeat., {}", err.descr),
                                 conf.text2parse))
            }
            (false, _, None, _) => (),
            (true, _, Some(lerr), Some(lok)) => {
                return Ok((lok.update_deep_error(&lerr), ast(ast_nodes)))
            }
            (true, _, Some(lerr), None) => {
                return Ok((status.update_deep_error(&lerr), ast(ast_nodes)))
            }
            (true, true, None, Some(lok)) => return Ok((lok, ast(ast_nodes))),
            (true, true, None, None) => return Ok((status, ast(ast_nodes))),
            (true, false, None, _) => (),
        }
    }
    Err(error(&status.pos,
              "stupid line waitting for #37339",
              conf.text2parse))
}
