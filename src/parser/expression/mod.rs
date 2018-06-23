//! Here we have the parser for non atomic things

use parser::atom::Atom;
use parser::{atom, Error, Result, ResultPartial, Started, Status};
use std::collections::HashMap;

#[cfg(test)]
mod test;

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  T Y P E S
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

/// The set of rules to be parsed
/// Any rule has a name
/// A rule can be registered just once
/// The starting rule is main
#[derive(Debug)]
pub struct SetOfRules<'a>(pub(crate) HashMap<String, Expression<'a>>);

impl<'a> SetOfRules<'a> {
    pub fn new(mrules: HashMap<String, Expression<'a>>) -> Self {
        SetOfRules(mrules)
    }
}

#[allow(missing_docs)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum Expression<'a> {
    Simple(Atom<'a>),
    And(MultiExpr<'a>),
    Or(MultiExpr<'a>),
    Not(Box<Expression<'a>>),
    Repeat(RepInfo<'a>),
    RuleName(String),
}

/// Opaque type to manage multiple expressions
/// #[allow(dead_code)]
#[derive(Debug)]
pub struct MultiExpr<'a>(pub(crate) Vec<Expression<'a>>);

impl<'a> MultiExpr<'a> {
    pub fn new(v: Vec<Expression<'a>>) -> Self {
        MultiExpr(v)
    }
}

/// Opaque type to manage repetition subexpression
#[derive(Debug)]
pub struct RepInfo<'a> {
    pub(crate) expression: Box<Expression<'a>>,
    pub(crate) min: NRep,
    pub(crate) max: Option<NRep>,
}

impl<'a> RepInfo<'a> {
    pub fn new(expression: Box<Expression<'a>>, min: usize, max: Option<usize>) -> Self {
        RepInfo {
            expression,
            min: NRep(min),
            max: max.map(|m| NRep(m)),
        }
    }
}

/// Number of repetitions of rule
#[derive(Debug)]
pub(crate) struct NRep(pub(crate) usize);

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  A P I
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

#[allow(dead_code)]
pub(crate) fn parse_expr<'a>(status: Status<'a>, expression: &'a Expression) -> Result<'a> {
    let started = Started(status.pos.n);
    Ok((parse_partial(status, expression)?, started))
}

//-----------------------------------------------------------------------
#[allow(dead_code)]
pub(crate) fn parse_partial<'a>(
    status: Status<'a>,
    expression: &'a Expression,
) -> ResultPartial<'a> {
    match expression {
        &Expression::Simple(ref val) => atom::parse(status, &val),
        &Expression::And(ref val) => parse_and(status, &val),
        &Expression::Or(ref val) => parse_or(status, &val),
        &Expression::Not(ref val) => parse_not(status, &val),
        &Expression::Repeat(ref val) => parse_repeat(status, &val),
        &Expression::RuleName(ref val) => parse_rule_name(status, &val),
    }
}

//-----------------------------------------------------------------------
pub(crate) fn parse_rule_name<'a>(status: Status<'a>, rule_name: &str) -> ResultPartial<'a> {
    let rules = &status.rules.0;
    let expression = rules.get(rule_name).ok_or(Error::from_status(
        &status,
        &format!("Missing rule: {}", rule_name),
    ))?;
    parse_partial(status, &expression)
}

//-----------------------------------------------------------------------
fn parse_and<'a>(status: Status<'a>, multi_expr: &'a MultiExpr) -> ResultPartial<'a> {
    let init_tc: (_, &[Expression]) = (status, &(multi_expr.0));

    tail_call(init_tc, |acc| {
        if acc.1.len() == 0 {
            TailCall::Return(Ok(acc.0))
        } else {
            let result_parse = parse_expr(acc.0, &acc.1[0]);
            match result_parse {
                Ok((st, _)) => TailCall::Call((st, &acc.1[1..])),
                Err(err) => TailCall::Return(Err(err)),
            }
        }
    })
}

//-----------------------------------------------------------------------
fn parse_or<'a>(status: Status<'a>, multi_expr: &'a MultiExpr) -> ResultPartial<'a> {
    let init_tc: (_, &[Expression]) = (status, &(multi_expr.0));

    Ok(tail_call(init_tc, |acc| {
        if acc.1.len() == 0 {
            TailCall::Return(Err(Error::from_status(&acc.0, "or")))
        } else {
            let try_parse = parse_expr(acc.0.clone(), &acc.1[0]);
            match try_parse {
                Ok(result) => TailCall::Return(Ok(result)),
                Err(_) => TailCall::Call((acc.0, &acc.1[1..])),
            }
        }
    })?.0)
}

//-----------------------------------------------------------------------
fn parse_not<'a>(status: Status<'a>, expression: &'a Expression) -> ResultPartial<'a> {
    match parse_partial(status.clone(), expression) {
        Ok(_) => Err(Error::from_status(&status, "not")),
        Err(_) => Ok(status),
    }
}

//-----------------------------------------------------------------------
fn parse_repeat<'a>(status: Status<'a>, rep_info: &'a RepInfo) -> ResultPartial<'a> {
    let big_min_bound = |counter| counter >= rep_info.min.0;
    let touch_max_bound = |counter: usize| match rep_info.max {
        Some(ref m) => counter + 1 == m.0,
        None => false,
    };

    Ok(tail_call((status, 0), |acc| {
        let try_parse = parse_partial(acc.0.clone(), &rep_info.expression);
        match (try_parse, big_min_bound(acc.1), touch_max_bound(acc.1)) {
            (Err(_), true, _) => TailCall::Return(Ok(acc.0)),
            (Err(_), false, _) => TailCall::Return(Err(Error::from_status(&acc.0, "repeat"))),
            (Ok(st), _, false) => TailCall::Call((st, acc.1 + 1)),
            (Ok(st), _, true) => TailCall::Return(Ok(st)),
        }
    })?)
}

//-----------------------------------------------------------------------
//  TailCall
//-----------------------------------------------------------------------
enum TailCall<T, R> {
    Call(T),
    Return(R),
}

fn tail_call<T, R, F>(seed: T, recursive_function: F) -> R
where
    F: Fn(T) -> TailCall<T, R>,
{
    let mut state = TailCall::Call(seed);
    loop {
        match state {
            TailCall::Call(arg) => {
                state = recursive_function(arg);
            }
            TailCall::Return(result) => {
                return result;
            }
        }
    }
}
