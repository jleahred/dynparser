use ast;
/// Support for minimum expressions elements
/// Here we have the parser and types for non dependencies kind
use parser::{Error, Result, Status};
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

/// This is a minimum expression element
#[allow(dead_code)]
#[derive(Debug)]
pub enum Atom<'a> {
    /// Literal string
    Literal(String),
    /// Character matches a list of chars or a list of ranges
    Match(MatchRules<'a>),
    /// Any char
    Dot,
    /// End Of File
    EOF,
}

/// contains a char slice and a (char,char) slice
/// if char matches one in char slice -> OK
/// if char matches between tuple in elems slice -> OK
#[derive(Debug)]
pub struct MatchRules<'a>(&'a str, Vec<(char, char)>);

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  A P I
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

#[allow(dead_code)]
pub(crate) fn parse<'a>(status: Status<'a>, atom: &'a Atom) -> Result<'a> {
    match atom {
        Atom::Literal(literal) => parse_literal(status, &literal),
        Atom::Match(ref match_rules) => parse_match(status, &match_rules),
        Atom::Dot => parse_dot(status),
        Atom::EOF => parse_eof(status),
    }
}

impl<'a> MatchRules<'a> {
    /// Create a MatchRules instance based on string and bounds
    pub fn init(s: &'a str, bounds: Vec<(char, char)>) -> Self {
        MatchRules(s, bounds)
    }
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        MatchRules("", vec![])
    }
    #[allow(dead_code)]
    pub(crate) fn with_chars(mut self, chrs: &'a str) -> Self {
        self.0 = chrs;
        self
    }

    #[allow(dead_code)]
    pub(crate) fn with_bound_chars(mut self, bounds: Vec<(char, char)>) -> Self {
        self.1 = bounds;
        self
    }
}

//-----------------------------------------------------------------------
//
//  SUPPORT
//
//-----------------------------------------------------------------------

macro_rules! ok {
    ($st:expr, $val:expr) => {
        Ok(($st, ast::Node::Val($val.to_owned())))
    };
}

#[allow(dead_code)]
fn parse_literal<'a>(mut status: Status<'a>, literal: &'a str) -> Result<'a> {
    for ch in literal.chars() {
        status = parse_char(status, ch)
            .map_err(|st| Error::from_status(&st, &format!("literal {}", literal)))?;
    }
    ok!(status, literal)
}

#[allow(dead_code)]
fn parse_dot<'a>(status: Status<'a>) -> Result<'a> {
    let (status, ch) = status
        .get_char()
        .map_err(|st| Error::from_status(&st, "dot"))?;

    ok!(status, ch.to_string())
}

#[allow(dead_code)]
fn parse_match<'a>(status: Status<'a>, match_rules: &MatchRules) -> Result<'a> {
    let match_char = |ch: char| -> bool {
        if match_rules.0.find(ch).is_some() {
            true
        } else {
            for &(b, t) in &match_rules.1 {
                if b <= ch && ch <= t {
                    return true;
                }
            }
            false
        }
    };

    status
        .get_char()
        .and_then(|(st, ch)| match match_char(ch) {
            true => ok!(st, ch.to_string()),
            false => Err(st),
        })
        .map_err(|st| {
            Error::from_status(
                &st,
                &format!("match. expected {} {:?}", match_rules.0, match_rules.1),
            )
        })
}

#[allow(dead_code)]
fn parse_eof<'a>(status: Status<'a>) -> Result<'a> {
    match status.get_char() {
        Ok((st, _ch)) => Err(Error::from_status(&st, "expected EOF")),
        Err(st) => ok!(st, "EOF"),
    }
}

fn parse_char<'a>(status: Status<'a>, ch: char) -> result::Result<Status<'a>, Status<'a>> {
    let (st, got_ch) = status.get_char()?;
    match ch == got_ch {
        true => Ok(st),
        false => Err(st),
    }
}

impl<'a> Status<'a> {
    #[allow(dead_code)]
    fn get_char(mut self) -> result::Result<(Self, char), Self> {
        match self.it_parsing.next() {
            None => Err(self),
            Some(ch) => {
                self.pos.n += 1;
                match ch {
                    '\n' => {
                        self.pos.col = 0;
                        self.pos.row += 1;
                        self.pos.start_line = self.pos.n;
                    }
                    '\r' => {
                        self.pos.col = 0;
                    }
                    _ => {
                        self.pos.col += 1;
                    }
                }
                Ok((self, ch))
            }
        }
    }
}
