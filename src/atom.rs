use {parser, Text2Parse, Error, error, Symbol, symbol};
use parser::Parse;


const MAX_DEPTH: parser::Depth = parser::Depth(100);

#[derive(Debug, PartialEq)]
pub enum Atom {
    Literal(String),
    Match(String, Vec<(char, char)>),
    Dot,
    Symbol(String),
    Nothing,
}


impl Parse for Atom {
    fn parse(&self,
             conf: &parser::Config,
             status: parser::Status)
             -> Result<parser::Status, Error> {
        match self {
            &Atom::Literal(ref lit) => parse_literal(&conf.text2parse, lit, status),
            &Atom::Dot => parse_dot(&conf.text2parse, status),
            &Atom::Symbol(ref s) => parse_symbol(conf, &symbol(s), status),
            &Atom::Match(ref chars, ref ch_ranges) => {
                parse_match(&conf.text2parse, chars, ch_ranges, status)
            }
            &Atom::Nothing => Ok(status),
        }
    }
}


fn parse_literal(text2parse: &Text2Parse,
                 s: &str,
                 mut status: parser::Status)
                 -> Result<parser::Status, Error> {
    let self_len = s.len();
    let in_text = text2parse.string()
        .chars()
        .skip(status.pos.n)
        .take(self_len)
        .collect::<String>();
    if s == in_text {
        status.pos.n += self_len;
        status.pos.col += self_len;
        Ok(status)
    } else {
        Err(error(&status.pos, &format!("expected {}", s)))
    }
}

fn parse_dot(text2parse: &Text2Parse, mut status: parser::Status) -> Result<parser::Status, Error> {
    match status.pos.n < text2parse.string().len() {
        true => {
            status.pos.n += 1;
            status.pos.col += 1;
            Ok(status)
        }
        false => Err(error(&status.pos, &format!("expected any char on end of file"))),
    }
}


fn parse_symbol(conf: &parser::Config,
                symbol: &Symbol,
                status: parser::Status)
                -> Result<parser::Status, Error> {
    let result = match status.depth > MAX_DEPTH {
        true => {
            Err(error(&status.pos,
                      &format!("too depth processing symbol {:?}", symbol)))
        }
        false => parser::parse(conf, symbol, status),
    };

    match result {
        Ok(_) => result,
        Err(error) => Err(::add_descr_error(error, &format!("paring rule <{:?}>", symbol))),
    }
}

fn parse_match(text2parse: &Text2Parse,
               chars: &String,
               ch_ranges: &Vec<(char, char)>,
               mut status: parser::Status)
               -> Result<parser::Status, Error> {

    fn match_ch(ch: char, chars: &String, ch_ranges: &Vec<(char, char)>) -> bool {
        if chars.find(ch).is_some() {
            true
        } else {
            for &(b, t) in ch_ranges {
                if b < ch && ch < t {
                    return true;
                }
            }
            false
        }
    }
    fn _error(status: &parser::Status, chars: &String, ch_ranges: &Vec<(char, char)>) -> Error {
        error(&status.pos, &format!("expected {} {:?}", chars, ch_ranges))
    }

    let next_charv = text2parse.string()
        .chars()
        .skip(status.pos.n)
        .take(1)
        .collect::<Vec<char>>();
    let next_char = next_charv.first();

    match next_char {
        Some(ch) => {
            if match_ch(*ch, chars, ch_ranges) {
                status.pos.n += 1;
                status.pos.col += 1;
                Ok(status)
            } else {
                Err(_error(&status, chars, ch_ranges))
            }
        }
        None => Err(_error(&status, chars, ch_ranges)),
    }
}
