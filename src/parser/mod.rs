//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//
//  mod parser
//
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

mod atom;
pub(crate) mod expression;

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
}

impl<'a> Status<'a> {
    #[allow(dead_code)]
    pub(crate) fn init(t2p: &'a str) -> Self {
        Status {
            text2parse: t2p,
            it_parsing: t2p.chars(),
            pos: Possition::init(),
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

#[macro_export]
macro_rules! lit {
    ($e:expr) => {{
        ::parser::expression::Expression::Simple(::parser::atom::Atom::Literal($e))
    }};
}

#[macro_export]
macro_rules! and {
    ($($e:expr),*) => {{
        use parser::expression::{Expression, MultiExpr};

        Expression::And(MultiExpr(vec![$($e ,)*]))
    }};
}

#[macro_export]
macro_rules! or {
    ($($e:expr),*) => {{
        use parser::expression::{Expression, MultiExpr};

        Expression::Or(MultiExpr(vec![$($e ,)*]))
    }};
}

#[macro_export]
macro_rules! not {
    ($e:expr) => {{
        use parser::expression::Expression;

        Expression::Not(Box::new($e))
    }};
}

#[macro_export]
macro_rules! rep {
    ($e:expr, $min:expr) => {{
        use parser::expression::{Expression, NRep, RepInfo};

        Expression::Repeat(RepInfo {
            expression: Box::new($e),
            min: NRep($min),
            max: None,
        })
    }};

    ($e:expr, $min:expr, $max:expr) => {{
        use parser::expression::{Expression, NRep, RepInfo};

        Expression::Repeat(RepInfo {
            expression: Box::new($e),
            min: NRep($min),
            max: Some(NRep($max)),
        })
    }};
}

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
