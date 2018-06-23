//! Tools to execute parser of a expression

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
#[allow(missing_docs)]
pub mod expression;

use std::result;
use std::str::Chars;
use {Error, Possition};

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  T Y P E S
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

//-----------------------------------------------------------------------
#[derive(Debug, Clone)]
pub(crate) struct Status<'a> {
    text2parse: &'a str,
    it_parsing: Chars<'a>,
    pub(crate) pos: Possition,
    pub(crate) rules: &'a expression::SetOfRules<'a>,
}

impl<'a> Status<'a> {
    #[allow(dead_code)]
    pub(crate) fn init(t2p: &'a str, rules: &'a expression::SetOfRules) -> Self {
        Status {
            text2parse: t2p,
            it_parsing: t2p.chars(),
            pos: Possition::init(),
            rules: rules,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Started(usize);

type Result<'a> = result::Result<(Status<'a>, Started), Error>;
type ResultPartial<'a> = result::Result<Status<'a>, Error>;

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
            line: "pending".to_owned(),
        }
    }
}
