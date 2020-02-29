#![warn(missing_docs)]
//! Here we have the parser for non atomic things

use super::super::idata::{
    cont::IVec,
    tc::{tail_call, TailCall},
};
use crate::ast;
use crate::parser::{atom, atom::Atom, ErrPriority, Error, Result, Status};
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
pub struct SetOfRules(pub(crate) HashMap<String, Expression>);

impl SetOfRules {
    /// Initialize a set of rules with a hashmap of <String, Expression>
    /// In general, is better to use the ```rules!``` macro
    pub fn new(mrules: HashMap<String, Expression>) -> Self {
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
    ///                         ref_rule!("rule2")
    ///                     }
    ///     };
    ///
    ///     let rules = rules.add("rule2", lit!("bcd"));
    ///
    ///     assert!(parse("aabcd", &rules).is_ok())
    /// }
    /// ```
    pub fn add(mut self, name: &str, expr: Expression) -> Self {
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
    ///                         ref_rule!("rule2")
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
pub enum Expression {
    Simple(Atom),
    And(MultiExpr),
    Or(MultiExpr),
    Not(Box<Expression>),
    Repeat(RepInfo),
    RuleName(String),
}

/// Opaque type to manage multiple expressions
#[derive(Debug)]
pub struct MultiExpr(pub(crate) Vec<Expression>);

impl MultiExpr {
    /// Creates a new instance of ```MultiExpr``` from a vector
    pub fn new(v: Vec<Expression>) -> Self {
        MultiExpr(v)
    }
}

/// Opaque type to manage repetition subexpression
#[derive(Debug)]
pub struct RepInfo {
    pub(crate) expression: Box<Expression>,
    pub(crate) min: NRep,
    pub(crate) max: Option<NRep>,
}

impl RepInfo {
    /// Creates a Repeticion Info for an expression with min and
    /// optionally max values to repeat
    pub fn new(expression: Box<Expression>, min: usize, max: Option<usize>) -> Self {
        RepInfo {
            expression,
            min: NRep(min),
            max: max.map(NRep),
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
pub(crate) fn parse(status: Status) -> Result {
    parse_rule_name(status, "main")
}

//-----------------------------------------------------------------------
//  SUPPORT

//-----------------------------------------------------------------------
fn parse_rule_name<'a>(status: Status<'a>, rule_name: &str) -> Result<'a> {
    // use std::time::{Duration, Instant};
    // let start = Instant::now();

    let status = if status.trace_rules {
        status.push_rule(&format!("r:{}", rule_name))
    } else {
        status
    };

    let rules = &status.rules.0;
    let expression = rules.get(rule_name).ok_or_else(|| {
        Error::from_status(
            &status,
            &format!("Missing rule: {}", rule_name),
            ErrPriority::Critical,
        )
    })?;
    let (st, nodes) = parse_expr(status, &expression)?;

    // let elapsed = start.elapsed();
    // println!(
    //     "____ elapsed time parsing {} {}.{}",
    //     rule_name,
    //     elapsed.as_secs(),
    //     elapsed.subsec_millis()
    // );
    Ok((st, ast::Node::Rule((rule_name.to_owned(), nodes))))
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
    match *expression {
        Expression::Simple(ref val) => parse_atom_as_expr(status, &val),
        Expression::And(ref val) => parse_and(status, &val),
        Expression::Or(ref val) => parse_or(&status, &val),
        Expression::Not(ref val) => parse_not(status, &val),
        Expression::Repeat(ref val) => parse_repeat(status, &val),
        Expression::RuleName(ref val) => parse_rule_name_as_expr(status, &val),
    }
}

//-----------------------------------------------------------------------
fn parse_and<'a>(status: Status<'a>, multi_expr: &'a MultiExpr) -> ResultExpr<'a> {
    let init_tc: (_, &[Expression], Vec<ast::Node>) = (status, &(multi_expr.0), vec![]);

    tail_call(init_tc, |acc| {
        if acc.1.is_empty() {
            TailCall::Return(Ok((acc.0, acc.2)))
        } else {
            let result_parse = parse_expr(acc.0, &acc.1[0]);
            match result_parse {
                Ok((status, vnodes)) => {
                    TailCall::Call((status, &acc.1[1..], acc.2.iappend(vnodes)))
                }
                Err(err) => TailCall::Return(Err(err)),
            }
        }
    })
}

//-----------------------------------------------------------------------
fn parse_or<'a>(status: &Status<'a>, multi_expr: &'a MultiExpr) -> ResultExpr<'a> {
    let deep_err = |oe1: Option<Error>, e2: Error| match oe1 {
        Some(e1) => match (e1.priority > e2.priority, e1.pos.n > e2.pos.n) {
            (true, _) => Some(e1),
            (false, true) => Some(e1),
            (false, false) => Some(e2),
        },
        None => Some(e2),
    };

    let init_tc: (_, &[Expression], Option<Error>) = (status.clone(), &(multi_expr.0), None);

    tail_call(init_tc, |acc| {
        if acc.1.is_empty() {
            TailCall::Return(Err(match acc.2 {
                Some(err) => err,
                _ => Error::from_status_normal(
                    &status,
                    "LOGIC ERROR!!! checked all options in or with ¿NO? errors",
                ),
            }))
        } else {
            let try_parse = parse_expr(acc.0.clone(), &acc.1[0]);
            match try_parse {
                Ok(result) => TailCall::Return(Ok(result)),
                Err(e) => {
                    if e.priority == ErrPriority::Critical {
                        TailCall::Return(Err(e))
                    } else {
                        TailCall::Call((acc.0, &acc.1[1..], deep_err(acc.2, e)))
                    }
                }
            }
        }
    })
}

//-----------------------------------------------------------------------
fn parse_not<'a>(status: Status<'a>, expression: &'a Expression) -> ResultExpr<'a> {
    match parse_expr(status.clone(), expression) {
        Ok(_) => Err(Error::from_status_normal(&status, "not")),
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

    let init_tc: (_, _, Vec<ast::Node>) = (status, 0, vec![]);
    Ok(tail_call(init_tc, |acc| {
        let try_parse = parse_expr(acc.0.clone(), &rep_info.expression);
        match (try_parse, big_min_bound(acc.1), touch_max_bound(acc.1)) {
            (Err(e), true, _) => {
                if e.priority == ErrPriority::Critical {
                    TailCall::Return(Err(e))
                } else {
                    TailCall::Return(Ok((acc.0.set_potential_error(e), acc.2)))
                }
            }
            (Err(e), false, _) => TailCall::Return(Err(e)),
            //     Err(Error::from_status(
            //     &acc.0,
            //     &format!("inside repeat {:#?}", e),
            // ))),
            (Ok((status, vnodes)), _, false) => {
                TailCall::Call((status, acc.1 + 1, acc.2.iappend(vnodes)))
            }
            (Ok((status, vnodes)), _, true) => {
                TailCall::Return(Ok((status, acc.2.iappend(vnodes))))
            }
        }
    })?)
}
//  SUPPORT
//-----------------------------------------------------------------------
