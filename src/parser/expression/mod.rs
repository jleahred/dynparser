#![warn(missing_docs)]
//! Here we have the parser for non atomic things

use ast;
use parser::{atom, atom::Atom, Error, Result, Status};
use std::collections::HashMap;
use std::result;

#[cfg(test)]
mod test;

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  T Y P E S
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub(crate) struct Started(usize);

pub(crate) type ResultExpr<'a> = result::Result<(Status<'a>, Vec<ast::Node>), Error>;

/// The set of rules to be parsed
/// Any rule has a name
/// A rule can be registered just once
/// The starting rule is main
#[derive(Debug)]
pub struct SetOfRules<'a>(pub(crate) HashMap<String, Expression<'a>>);

impl<'a> SetOfRules<'a> {
    /// Initialize a set of rules with a hashmap of <String, Expression>
    /// In general, is better to use the ```rules!``` macro
    pub fn new(mrules: HashMap<String, Expression<'a>>) -> Self {
        SetOfRules(mrules)
    }

    /// As this is a dynamic parser, it is necessary to add rules on
    /// runtime.
    ///
    /// This method, will take the owner ship, and will return itself
    ///
    /// In this way, you don't need to declare mutable vars.
    /// You could need recursion in some cases
    ///
    /// To add several rules at once, look for merge
    ///
    /// ```
    /// #[macro_use]  extern crate dynparser;
    /// use dynparser::parse;
    ///
    /// fn main() {
    ///     let rules = rules!{
    ///        "main"   =>  and!{
    ///                         rep!(lit!("a"), 1, 5),
    ///                         rule!("rule2")
    ///                     }
    ///     };
    ///
    ///     let rules = rules.add("rule2", lit!("bcd"));
    ///
    ///     assert!(parse("aabcd", &rules).is_ok())
    /// }
    /// ```
    pub fn add(mut self, name: &str, expr: Expression<'a>) -> Self {
        self.0.insert(name.to_owned(), expr);
        self
    }

    /// As this is a dynamic parser, it is necessary to add rules on
    /// runtime.
    ///
    /// This method, will take the owner ship, and will return itself
    ///
    /// In this way, you don't need to declare mutable vars.
    /// You could need recursion in some cases
    ///
    /// It will add the rules from the parameter
    ///
    /// ```
    /// #[macro_use]  extern crate dynparser;
    /// use dynparser::parse;
    ///
    /// fn main() {
    ///     let rules = rules!{
    ///        "main"   =>  and!{
    ///                         rep!(lit!("a"), 1, 5),
    ///                         rule!("rule2")
    ///                     }
    ///     };
    ///
    ///     let rules = rules.merge(rules!{"rule2" => lit!("bcd")});
    ///
    ///     assert!(parse("aabcd", &rules).is_ok())
    /// }
    /// ```
    pub fn merge(self, rules2merge: Self) -> Self {
        SetOfRules(rules2merge.0.into_iter().chain(self.0).collect())
    }
}

#[allow(missing_docs)]
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
#[derive(Debug)]
pub struct MultiExpr<'a>(pub(crate) Vec<Expression<'a>>);

impl<'a> MultiExpr<'a> {
    /// Creates a new instance of ```MultiExpr``` from a vector
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
    /// Creates a Repeticion Info for an expression with min and
    /// optionally max values to repeat
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

//-----------------------------------------------------------------------
pub(crate) fn parse<'a>(status: Status<'a>) -> Result<'a> {
    parse_rule_name(status, "main")
}

//-----------------------------------------------------------------------
//  SUPPORT

//-----------------------------------------------------------------------
fn parse_rule_name<'a>(status: Status<'a>, rule_name: &str) -> Result<'a> {
    let rules = &status.rules.0;
    let expression = rules.get(rule_name).ok_or(Error::from_status(
        &status,
        &format!("Missing rule: {}", rule_name),
    ))?;
    let (st, vnodes) = parse_expr(status, &expression)?;
    Ok((st, ast::Node::Rule((rule_name.to_owned(), vnodes))))
}

fn parse_atom_as_expr<'a>(status: Status<'a>, a: &'a Atom) -> ResultExpr<'a> {
    let (st, node) = atom::parse(status, a)?;
    Ok((st, vec![node]))
}

fn parse_rule_name_as_expr<'a>(status: Status<'a>, rule_name: &str) -> ResultExpr<'a> {
    let (st, ast) = parse_rule_name(status, rule_name)?;
    Ok((st, vec![ast]))
}

fn parse_expr<'a>(status: Status<'a>, expression: &'a Expression) -> ResultExpr<'a> {
    match expression {
        &Expression::Simple(ref val) => parse_atom_as_expr(status, &val),
        &Expression::And(ref val) => parse_and(status, &val),
        &Expression::Or(ref val) => parse_or(status, &val),
        &Expression::Not(ref val) => parse_not(status, &val),
        &Expression::Repeat(ref val) => parse_repeat(status, &val),
        &Expression::RuleName(ref val) => parse_rule_name_as_expr(status, &val),
    }
}

//-----------------------------------------------------------------------
fn parse_and<'a>(status: Status<'a>, multi_expr: &'a MultiExpr) -> ResultExpr<'a> {
    let vec_concat = |mut v1: Vec<_>, v2: Vec<_>| {
        v1.extend(v2);
        v1
    };

    let init_tc: (_, &[Expression], Vec<ast::Node>) = (status, &(multi_expr.0), vec![]);

    tail_call(init_tc, |acc| {
        if acc.1.len() == 0 {
            TailCall::Return(Ok((acc.0, acc.2)))
        } else {
            let result_parse = parse_expr(acc.0, &acc.1[0]);
            match result_parse {
                Ok((status, vnodes)) => {
                    TailCall::Call((status, &acc.1[1..], vec_concat(acc.2, vnodes)))
                }
                Err(err) => TailCall::Return(Err(err)),
            }
        }
    })
}

//-----------------------------------------------------------------------
fn parse_or<'a>(status: Status<'a>, multi_expr: &'a MultiExpr) -> ResultExpr<'a> {
    let deep_err = |oe1: Option<Error>, e2: Error| match oe1 {
        Some(e1) => if e1.pos.n > e2.pos.n {
            Some(e1)
        } else {
            Some(e2)
        },
        None => Some(e2),
    };
    let init_tc: (_, &[Expression], _) = (status, &(multi_expr.0), None);

    tail_call(init_tc, |acc| {
        if acc.1.len() == 0 {
            TailCall::Return(Err(acc.2.expect("checked all options of or with no errors")))
        } else {
            let try_parse = parse_expr(acc.0.clone(), &acc.1[0]);
            match try_parse {
                Ok(result) => TailCall::Return(Ok(result)),
                Err(e) => TailCall::Call((acc.0, &acc.1[1..], deep_err(acc.2, e))),
            }
        }
    })
}

//-----------------------------------------------------------------------
fn parse_not<'a>(status: Status<'a>, expression: &'a Expression) -> ResultExpr<'a> {
    match parse_expr(status.clone(), expression) {
        Ok(_) => Err(Error::from_status(&status, "not")),
        Err(_) => Ok((status, vec![])),
    }
}

//-----------------------------------------------------------------------
fn parse_repeat<'a>(status: Status<'a>, rep_info: &'a RepInfo) -> ResultExpr<'a> {
    let big_min_bound = |counter| counter >= rep_info.min.0;
    let touch_max_bound = |counter: usize| match rep_info.max {
        Some(ref m) => counter + 1 == m.0,
        None => false,
    };

    let init_tc = (status, 0, vec![]);
    Ok(tail_call(init_tc, |acc| {
        let try_parse = parse_expr(acc.0.clone(), &rep_info.expression);
        match (try_parse, big_min_bound(acc.1), touch_max_bound(acc.1)) {
            (Err(_), true, _) => TailCall::Return(Ok((acc.0, acc.2))),
            (Err(e), false, _) => TailCall::Return(Err(e)),
            //     Err(Error::from_status(
            //     &acc.0,
            //     &format!("inside repeat {:#?}", e),
            // ))),
            (Ok((status, vnodes)), _, false) => TailCall::Call((status, acc.1 + 1, vnodes)),
            (ok, _, true) => TailCall::Return(ok),
        }
    })?)
}
//  SUPPORT
//-----------------------------------------------------------------------

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
