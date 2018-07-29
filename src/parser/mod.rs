#![warn(missing_docs)]
//! Tools to execute parser of a expression

use ast;
use std::result;

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//
//  mod parser
//
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

/// Support for minimum expressions elements
pub mod atom;
pub mod expression;

use std::str::Chars;

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  T Y P E S
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

/// Information about the possition on parsing
#[derive(PartialEq, Clone, Debug)]
pub struct Possition {
    /// char position parsing
    pub n: usize,
    /// row parsing row
    pub row: usize,
    /// parsing col
    pub col: usize,
    /// possition were line started for current pos *m*
    pub start_line: usize,
}

impl Possition {
    fn init() -> Self {
        Self {
            n: 0,
            row: 0,
            col: 0,
            start_line: 0,
        }
    }
}

/// Context error information
#[derive(Debug)]
pub struct Error {
    /// Possition achive parsing
    pub pos: Possition,
    /// Error description parsing
    pub descr: String,
    /// Line content where error was produced
    pub line: String,
    /// Suberrors when parsing an *or* (it could be removed!)
    pub errors: Vec<Error>,
    /// Rules path followed till got the error
    pub parsing_rules: Vec<String>,
}

//-----------------------------------------------------------------------
#[derive(Debug, Clone)]
pub(crate) struct Status<'a> {
    pub(crate) text2parse: &'a str,
    pub(crate) it_parsing: Chars<'a>,
    pub(crate) pos: Possition,
    pub(crate) walking_rules: Vec<String>,
    pub(crate) rules: &'a expression::SetOfRules,
}

impl<'a> Status<'a> {
    pub(crate) fn init(t2p: &'a str, rules: &'a expression::SetOfRules) -> Self {
        Status {
            text2parse: t2p,
            it_parsing: t2p.chars(),
            pos: Possition::init(),
            walking_rules: vec![],
            rules: rules,
        }
    }
    pub(crate) fn push_rule(mut self, on_node: &str) -> Self {
        self.walking_rules.push(on_node.to_string());
        self
    }
}

pub(crate) type Result<'a> = result::Result<(Status<'a>, ast::Node), Error>;

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  A P I
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

//-----------------------------------------------------------------------
//  T E S T
//-----------------------------------------------------------------------
#[cfg(test)]
mod test;

//-----------------------------------------------------------------------
//  I N T E R N A L
//-----------------------------------------------------------------------
impl Error {
    pub(crate) fn from_status(status: &Status, descr: &str) -> Self {
        Error {
            pos: status.pos.clone(),
            descr: descr.to_owned(),
            line: status.text2parse[status.pos.start_line..status.pos.n].to_string(),
            errors: vec![],
            parsing_rules: status.walking_rules.clone(),
        }
    }

    pub(crate) fn from_st_errs(status: &Status, descr: &str, errors: Vec<Error>) -> Self {
        Error {
            pos: status.pos.clone(),
            descr: descr.to_owned(),
            line: status.text2parse[status.pos.start_line..status.pos.n].to_string(),
            errors,
            parsing_rules: status.walking_rules.clone(),
        }
    }
}
