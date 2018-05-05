//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//
//  mod parser::atom
//
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

/// Here we have the parser and types for non dependencies kind

use std::result;
use super::{Error, ResultPartial, Status};

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
pub(crate) enum Atom<'a> {
    Literal(&'a str),
    Match(MatchRules<'a>),
    Dot,
    EOF,
}

/// contains a &str to the string to parse
// #[derive(Debug)]
// pub(crate) struct Literal<'a>(pub(crate) &'a str);

/// contains a char slice and a (char,char) slice
/// if char matches one in char slice -> OK
/// if char matches between tuple in elems slice -> OK
#[derive(Debug)]
pub(crate) struct MatchRules<'a>(&'a str, &'a [(char, char)]);

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  A P I
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

#[allow(dead_code)]
pub(crate) fn parse<'a>(status: Status<'a>, atom: &'a Atom) -> ResultPartial<'a> {
    match atom {
        &Atom::Literal(literal) => parse_literal(status, literal),
        &Atom::Match(ref match_rules) => parse_match(status, &match_rules),
        &Atom::Dot => parse_dot(status),
        &Atom::EOF => parse_eof(status),
    }
}

impl<'a> MatchRules<'a> {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        MatchRules("", &[])
    }
    #[allow(dead_code)]
    pub(crate) fn with_chars(mut self, chrs: &'a str) -> Self {
        self.0 = chrs;
        self
    }

    #[allow(dead_code)]
    pub(crate) fn with_bound_chars(mut self, bounds: &'a [(char, char)]) -> Self {
        self.1 = bounds.clone();
        self
    }
}

//-----------------------------------------------------------------------
//
//  SUPPORT
//
//-----------------------------------------------------------------------

#[allow(dead_code)]
fn parse_literal<'a>(mut status: Status<'a>, literal: &'a str) -> ResultPartial<'a> {
    for ch in literal.chars() {
        status = parse_char(status, ch)
            .map_err(|st| Error::from_status(&st, &format!("literal {}", literal)))?;
    }
    Ok(status)
}

#[allow(dead_code)]
fn parse_dot<'a>(status: Status<'a>) -> ResultPartial<'a> {
    let (st, _ch) = status
        .get_char()
        .map_err(|st| Error::from_status(&st, "dot"))?;

    Ok(st)
}

#[allow(dead_code)]
fn parse_match<'a>(status: Status<'a>, match_rules: &MatchRules) -> ResultPartial<'a> {
    let match_char = |ch: char| -> bool {
        if match_rules.0.find(ch).is_some() {
            true
        } else {
            for &(b, t) in match_rules.1 {
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
            true => Ok(st),
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
fn parse_eof<'a>(status: Status<'a>) -> ResultPartial<'a> {
    match status.get_char() {
        Ok((st, _ch)) => Err(Error::from_status(&st, "expected EOF")),
        Err(st) => Ok(st),
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
