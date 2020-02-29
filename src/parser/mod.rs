#![warn(missing_docs)]
//! Tools to execute parser of a expression

use crate::ast;
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

/// Error priority
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ErrPriority {
    /// normal error
    Normal,
    /// Very important error
    Critical,
}

/// Context error information
#[derive(Debug, Clone)]
pub struct Error {
    /// Possition achive parsing
    pub pos: Possition,
    /// Error description parsing
    pub descr: String,
    /// Line content before where error was produced
    pub line_before: String,
    /// Line content after where error was produced
    pub line_after: String,
    // Suberrors when parsing an *or* (it could be removed!)
    // pub errors: Vec<Error>,
    /// Rules path followed till got the error
    /// Only available if trace_rules is on
    pub parsing_rules: Vec<String>,
    /// error priority
    pub priority: ErrPriority,
}

//-----------------------------------------------------------------------
#[derive(Debug, Clone)]
pub(crate) struct Status<'a> {
    pub(crate) text2parse: &'a str,
    pub(crate) it_parsing: Chars<'a>,
    pub(crate) pos: Possition,
    pub(crate) rules: &'a expression::SetOfRules,
    //  main            =   ("a")*
    //  if you try to parse "abb" i.e.
    //  the error will not be processed full input
    //  It's true, but it could be more useful to know where
    //  it fail trying to repeat
    pub(crate) potential_error: Option<Error>,

    /// If true, it will fill walking rules
    /// too expensive. For use just to debug errors
    pub(crate) trace_rules: bool,
    pub(crate) walking_rules: Vec<String>,
}

impl<'a> Status<'a> {
    pub(crate) fn init(t2p: &'a str, rules: &'a expression::SetOfRules) -> Self {
        Status {
            text2parse: t2p,
            it_parsing: t2p.chars(),
            pos: Possition::init(),
            trace_rules: false,
            walking_rules: vec![],
            rules,
            potential_error: None,
        }
    }

    pub(crate) fn init_debug(
        t2p: &'a str,
        rules: &'a expression::SetOfRules,
        trace_rules: bool,
    ) -> Self {
        Status {
            text2parse: t2p,
            it_parsing: t2p.chars(),
            pos: Possition::init(),
            trace_rules,
            walking_rules: vec![],
            rules,
            potential_error: None,
        }
    }
    pub(crate) fn push_rule(mut self, on_node: &str) -> Self {
        self.walking_rules.push(on_node.to_string());
        self
    }
    pub(crate) fn set_potential_error(mut self, err: Error) -> Self {
        self.potential_error = Some(err);
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
    pub(crate) fn from_status(status: &Status, descr: &str, prior: ErrPriority) -> Self {
        Error {
            pos: status.pos.clone(),
            descr: descr.to_owned(),
            line_before: status.text2parse[status.pos.start_line..status.pos.n].to_string(),
            line_after: status
                .it_parsing
                .clone()
                .take_while(|&ch| ch != '\n' && ch != '\r')
                .collect(),
            // errors: vec![],
            parsing_rules: status.walking_rules.clone(),
            priority: prior,
        }
    }

    pub(crate) fn from_status_normal(status: &Status, descr: &str) -> Self {
        Self::from_status(status, descr, ErrPriority::Normal)
    }

    // pub(crate) fn from_st_errs(status: &Status, descr: &str, errors: Vec<Error>) -> Self {
    //     let max_pr = |verrors: &Vec<Error>| {
    //         use std::cmp::max;
    //         verrors
    //             .iter()
    //             .fold(ErrPriority::Normal, |acc, err| max(acc, err.priority))
    //     };

    //     let mp = max_pr(&errors);
    //     Error {
    //         pos: status.pos.clone(),
    //         descr: descr.to_owned(),
    //         line: status.text2parse[status.pos.start_line..status.pos.n].to_string(),
    //         // errors,
    //         parsing_rules: status.walking_rules.clone(),
    //         priority: mp,
    //     }
    // }
}
