use parser::{Parsing, ParsingText, ParsingPossition};


#[derive(Debug, PartialEq)]
pub enum Terminal {
    Literal(String),
    Match,
    Dot,
    Symbol,
}


impl Terminal {
    fn parse(&self, parsing: Parsing) -> Result<Parsing, String> {
        match self {
            &Terminal::Literal(ref s) => parse_literal(s, parsing),
            &Terminal::Dot => parse_dot(parsing),
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

fn literal(s: &str) -> Terminal {
    Terminal::Literal(s.to_owned())
}

#[test]
fn test_parse_literal() {
    assert!(literal("aaaa").parse(Parsing::new("aaaa")).is_ok());
    assert!(literal("aaaa").parse(Parsing::new("aaa")).is_err());
    assert!(literal("aaaa").parse(Parsing::new("")).is_err());
    assert!(literal("aaaa").parse(Parsing::new("bbbb")).is_err());
    assert!(literal("aaaa").parse(Parsing::new("b")).is_err());

    assert!(literal("aaaa").parse(Parsing::new("aaaa")) ==
            Ok(Parsing {
        position: ParsingPossition {
            n: 4,
            col: 4,
            row: 0,
        },
        parsing_text: ParsingText::new("aaaa"),
    }));
}



// #[test]
// fn validate_literal() {
//     let grammar = &parser!{
//             "main" => lit!("aaaa")
//     };
//     assert!(parse(&symbol("main"), Parsing::new("aaaa"), grammar).is_ok());
//     assert!(parse(&symbol("main"), Parsing::new("aaa"), grammar).is_err());
//     assert!(parse(&symbol("main"), Parsing::new("bbbb"), grammar).is_err());
//     assert!(parse(&symbol("main"), Parsing::new("aaaaa"), grammar).is_err());
//     assert!(parse(&symbol("main"), Parsing::new(""), grammar).is_err());
// }

// #[test]
// fn validate_dot() {
//     let grammar = &parser!{
//             "main" => dot!()
//     };

//     assert!(parse(&symbol("main"), Parsing::new("a"), grammar).is_ok());

//     assert!(parse(&symbol("main"), Parsing::new("aaa"), grammar).is_err());

//     assert!(parse(&symbol("main"), Parsing::new(""), grammar).is_err());
// }
