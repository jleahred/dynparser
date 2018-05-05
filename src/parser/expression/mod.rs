//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//
//  mod parser::expression
//
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

/// Here we have the parser for dependent kinds

use std::result;
use parser::{Error, Result, ResultPartial, Started, Status, atom::{self, Atom}};

#[cfg(test)]
mod test;

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  T Y P E S
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Expression<'a> {
    Simple(Atom<'a>),
    And(MultiExpr<'a>),
    Or(MultiExpr<'a>),
    Not(Box<Expression<'a>>),
    Repeat(RepInfo<'a>), //  min max
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct MultiExpr<'a>(&'a [Expression<'a>]);

#[derive(Debug)]
pub(crate) struct RepInfo<'a> {
    expression: Box<Expression<'a>>,
    min: NRep,
    max: Option<NRep>,
}

#[derive(Debug)]
pub(crate) struct NRep(pub usize);

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  A P I
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

#[allow(dead_code)]
pub(crate) fn parse<'a>(status: Status<'a>, expression: &'a Expression) -> Result<'a> {
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
    }
}

//-----------------------------------------------------------------------
fn parse_and<'a>(status: Status<'a>, multi_expr: &'a MultiExpr) -> ResultPartial<'a> {
    let init_tc: (_, &[Expression]) = (status, &(multi_expr.0));

    Ok(try_tail_call(init_tc, |acc| {
        if acc.1.len() == 0 {
            Ok(TailCall::Return(acc))
        } else {
            Ok(TailCall::Call((parse(acc.0, &acc.1[0])?.0, &acc.1[1..])))
        }
    })?.0)
}

//-----------------------------------------------------------------------
fn parse_or<'a>(status: Status<'a>, multi_expr: &'a MultiExpr) -> ResultPartial<'a> {
    let init_tc: (_, &[Expression]) = (status, &(multi_expr.0));

    Ok(tail_call(init_tc, |acc| {
        if acc.1.len() == 0 {
            TailCall::Return(Err(Error::from_status(&acc.0, "or")))
        } else {
            let try_parse = parse(acc.0.clone(), &acc.1[0]);
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
pub enum TailCall<T, R> {
    Call(T),
    Return(R),
}

pub fn try_tail_call<T, R, E, F>(seed: T, recursive_function: F) -> result::Result<R, E>
where
    F: Fn(T) -> result::Result<TailCall<T, R>, E>,
{
    let mut state = TailCall::Call(seed);
    loop {
        match state {
            TailCall::Call(arg) => {
                state = recursive_function(arg)?;
            }
            TailCall::Return(result) => {
                return Ok(result);
            }
        }
    }
}

pub fn tail_call<T, R, F>(seed: T, recursive_function: F) -> R
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
