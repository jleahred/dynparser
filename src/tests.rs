macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);


use parsing::parsing_text;
use atom::Atom;
use ::symbol;
use ::parse;
use expression::Expression;

pub fn lit(s: &str) -> Expression {
    Expression::Atom(Atom::Literal(s.to_owned()))
}

pub fn dot() -> Expression {
    Expression::Atom(Atom::Dot)
}




#[test]
fn parse_literal() {
    let rules = map!(symbol("main") => lit("aaaa"));
    let parsed = parse(&symbol("main"), parsing_text("aaaa"), &rules);
    assert!(parsed.is_ok());

    let rules = map!(symbol("main") => lit("aaaa"));
    let parsed = parse(&symbol("main"), parsing_text("aaa"), &rules);
    assert!(parsed.is_err());

    let rules = map!(symbol("main") => lit("aaaa"));
    let parsed = parse(&symbol("main"), parsing_text("aaaaa"), &rules);
    assert!(parsed.is_err());

    let rules = map!(symbol("main") => lit("aaaa"));
    let parsed = parse(&symbol("main"), parsing_text("bbbb"), &rules);
    assert!(parsed.is_err());
}


#[test]
fn parse_dot() {
    let rules = map!(symbol("main") => dot());
    let parsed = parse(&symbol("main"), parsing_text("a"), &rules);
    assert!(parsed.is_ok());

    let rules = map!(symbol("main") => dot());
    let parsed = parse(&symbol("main"), parsing_text("aa"), &rules);
    assert!(parsed.is_err());

    let rules = map!(symbol("main") => dot());
    let parsed = parse(&symbol("main"), parsing_text(""), &rules);
    assert!(parsed.is_err());
}
