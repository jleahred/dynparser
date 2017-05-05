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


use atom::Atom;
use ::symbol;
use ::parse;
use text2parse;
use expression::{Expression, MultiExpr};

pub fn lit(s: &str) -> Expression {
    Expression::Simple(Atom::Literal(s.to_owned()))
}

pub fn dot() -> Expression {
    Expression::Simple(Atom::Dot)
}

pub fn or(exp_list: Vec<Expression>) -> Expression {
    Expression::Or(MultiExpr(exp_list))
}




#[test]
fn parse_literal() {
    let rules = map!(symbol("main") => lit("aaaa"));
    let parsed = parse(&text2parse("aaaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let rules = map!(symbol("main") => lit("aaaa"));
    let parsed = parse(&text2parse("aaa"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let rules = map!(symbol("main") => lit("aaaa"));
    let parsed = parse(&text2parse("aaaaa"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let rules = map!(symbol("main") => lit("aaaa"));
    let parsed = parse(&text2parse("bbbb"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}


#[test]
fn parse_dot() {
    let rules = map!(symbol("main") => dot());
    let parsed = parse(&text2parse("a"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let rules = map!(symbol("main") => dot());
    let parsed = parse(&text2parse("aa"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let rules = map!(symbol("main") => dot());
    let parsed = parse(&text2parse(""), &symbol("main"), &rules);
    assert!(parsed.is_err());
}


#[test]
fn parse_or() {
    let rules = map!(symbol("main") => or(vec![lit("aaaa"), lit("bbbb")]));

    let parsed = parse(&text2parse("aaaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());


    let parsed = parse(&text2parse("cccc"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}
