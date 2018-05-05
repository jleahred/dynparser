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
mod expression;

use {Error, Possition};
use std::str::Chars;
use std::result;

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
    pos: Possition,
}

impl<'a> Status<'a> {
    #[allow(dead_code)]
    fn init(t2p: &'a str) -> Self {
        Status {
            text2parse: t2p,
            it_parsing: t2p.chars(),
            pos: Possition::init(),
        }
    }
}

impl Error {
    pub(crate) fn from_status(status: &Status, descr: &str) -> Self {
        Error {
            pos: status.pos.clone(),
            descr: descr.to_owned(),
            line: "pending".to_owned(),
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
