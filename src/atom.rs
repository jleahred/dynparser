use {parser, Text2Parse, Error, error};
use parser::Parse;


#[derive(Debug, PartialEq)]
pub enum Atom {
    Literal(String),
    Match,
    Dot,
    Symbol,
}


impl Parse for Atom {
    fn parse(&self,
             text2parse: &Text2Parse,
             pars_pos: parser::Possition)
             -> Result<parser::Possition, Error> {
        match self {
            &Atom::Literal(ref lit) => parse_literal(&text2parse, lit, pars_pos),
            &Atom::Dot => parse_dot(&text2parse, pars_pos),
            _ => Err(error(&pars_pos, "pending implementation")),
        }
    }
}


fn parse_literal(text2parse: &Text2Parse,
                 s: &str,
                 mut pars_pos: parser::Possition)
                 -> Result<parser::Possition, Error> {
    let self_len = s.len();
    let in_text = text2parse.string()
        .chars()
        .skip(pars_pos.n)
        .take(self_len)
        .collect::<String>();
    if s == in_text {
        pars_pos.n += self_len;
        pars_pos.col += self_len;
        Ok(pars_pos)
    } else {
        Err(error(&pars_pos, &format!("expected {}", s)))
    }
}

fn parse_dot(text2parse: &Text2Parse,
             mut pars_pos: parser::Possition)
             -> Result<parser::Possition, Error> {
    match pars_pos.n < text2parse.string().len() {
        true => {
            pars_pos.n += 1;
            pars_pos.col += 1;
            Ok(pars_pos)
        }
        false => Err(error(&pars_pos, &format!("expected any char on end of file"))),
    }
}
