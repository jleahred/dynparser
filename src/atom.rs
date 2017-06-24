use {parser, Text2Parse, Error, error, Symbol, symbol};
use parser::Parse;
use ast;


const MAX_DEPTH: parser::Depth = parser::Depth(50000);

#[derive(Debug, PartialEq)]
pub enum Atom {
    Literal(String),
    Match(String, Vec<(char, char)>),
    Dot,
    Symbol(String),
    EOF,
}


impl Parse for Atom {
    fn parse(&self,
             conf: &parser::Config,
             status: parser::Status)
             -> Result<(parser::Status, ast::Node), Error> {
        match self {
            &Atom::Literal(ref lit) => parse_literal(&conf.text2parse, lit, status),
            &Atom::Dot => parse_dot(&conf.text2parse, status),
            &Atom::Symbol(ref s) => parse_symbol(conf, &symbol(s), status.inc_depth()),

            &Atom::Match(ref chars, ref ch_ranges) => {
                parse_match(&conf.text2parse, chars, ch_ranges, status)
            }
            &Atom::EOF => parse_eof(&conf.text2parse, status),
        }
    }
}


fn parse_literal(text2parse: &Text2Parse,
                 s: &str,
                 mut status: parser::Status)
                 -> Result<(parser::Status, ast::Node), Error> {
    let self_len = s.len();
    let in_text = text2parse.0
        .chars()
        .skip(status.pos.n)
        .take(self_len)
        .collect::<String>();
    if s == in_text {
        status.pos.inc_chars(&in_text);
        Ok((status, ast::Node::new_valstr(ast::K::ALit, s)))
    } else {
        Err(error(&status.pos,
                  &format!("lit. expected {:?}, got {:?}", s, in_text),
                  text2parse))
    }
}

fn parse_dot(text2parse: &Text2Parse,
             mut status: parser::Status)
             -> Result<(parser::Status, ast::Node), Error> {
    let current_char = || {
        text2parse.0
            .chars()
            .skip(2)
            .next()
            .unwrap_or('?')
            .to_string()
    };

    match status.pos.n < text2parse.0.len() {
        true => {
            status.pos.inc_char(text2parse);
            Ok((status, ast::Node::new_valstr(ast::K::ADot, &current_char())))
        }
        false => {
            Err(error(&status.pos,
                      &format!("expected any char on end of file"),
                      text2parse))
        }
    }
}


pub fn parse_symbol(conf: &parser::Config,
                    symbol: &Symbol,
                    status: parser::Status)
                    -> Result<(parser::Status, ast::Node), Error> {
    match status.depth > MAX_DEPTH {
            true => {
                panic!("Too depth processing symbol.")
                // Err(error(&status.pos,
                //           &format!("too depth processing symbol {}", symbol.0),
                //           conf.text2parse))
            }
            false => {
                conf.rules
                    .get(symbol)
                    .ok_or(error(&status.pos,
                                 &format!("undefined symbol {}", symbol.0),
                                 conf.text2parse))?
                    .parse(conf, status)
            }
        }
        .map(|(nwst, nwast)| {
            (nwst, ast::Node::new_valstr(ast::K::ASymbref, &symbol.0).merge(nwast))
        })
        .map_err(|error| ::add_descr_error(error, &format!("s.{}", symbol.0)))
}

fn parse_match(text2parse: &Text2Parse,
               chars: &String,
               ch_ranges: &Vec<(char, char)>,
               mut status: parser::Status)
               -> Result<(parser::Status, ast::Node), Error> {
    fn match_ch(ch: char, chars: &String, ch_ranges: &Vec<(char, char)>) -> bool {
        if chars.find(ch).is_some() {
            true
        } else {
            for &(b, t) in ch_ranges {
                if b <= ch && ch <= t {
                    return true;
                }
            }
            false
        }
    }
    let _error = error(&status.pos.clone(),
                       &format!("match. expected {} {:?}", chars, ch_ranges),
                       text2parse);

    let next_char = text2parse.0
        .chars()
        .skip(status.pos.n)
        .next();

    match next_char {
        Some(ch) => {
            if match_ch(ch, chars, ch_ranges) {
                status.pos.inc_char(text2parse);
                Ok((status, ast::Node::new_valstr(ast::K::AMatch, &ch.to_string())))
            } else {
                Err(_error)
            }
        }
        None => Err(_error),
    }
}

fn parse_eof(text2parse: &Text2Parse,
             status: parser::Status)
             -> Result<(parser::Status, ast::Node), Error> {
    if status.pos.n == text2parse.0.len() {
        Ok((status, ast::Node::new_valstr(ast::K::AEof, "")))
    } else {
        Err(error(&status.pos.clone(), &format!("expected eof. "), text2parse))
    }
}
