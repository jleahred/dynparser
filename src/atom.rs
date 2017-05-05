use parsing::Parsing;


#[derive(Debug, PartialEq)]
pub enum Atom {
    Literal(String),
    Match,
    Dot,
    Symbol,
}



impl Atom {
    pub fn parse(&self, parsing: Parsing) -> Result<Parsing, String> {
        match self {
            &Atom::Literal(ref s) => parse_literal(s, parsing),
            &Atom::Dot => parse_dot(parsing),
            _ => Err("pending implementation".to_owned()),
        }
    }
}


fn parse_literal(s: &str, mut parsing: Parsing) -> Result<Parsing, String> {
    let self_len = s.len();
    let in_text = parsing.parsing_text
        .string()
        .chars()
        .skip(parsing.position.n)
        .take(self_len)
        .collect::<String>();
    if s == in_text {
        parsing.position.n += self_len;
        parsing.position.col += self_len;
        Ok(parsing)
    } else {
        Err("error parsing".to_owned())
    }
}

fn parse_dot(mut parsing: Parsing) -> Result<Parsing, String> {
    match parsing.position.n < parsing.parsing_text.string().len() {
        true => {
            parsing.position.n += 1;
            parsing.position.col += 1;
            Ok(parsing)
        }
        false => Err("expected any char on end of file".to_owned()),
    }
}
