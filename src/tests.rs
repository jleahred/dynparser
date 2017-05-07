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
use {symbol, parse, text2parse};
use expression::{Expression, MultiExpr};

fn lit(s: &str) -> Expression {
    Expression::Simple(Atom::Literal(s.to_owned()))
}

fn dot() -> Expression {
    Expression::Simple(Atom::Dot)
}

// fn nothing() -> Expression {
//     Expression::Simple(Atom::Nothing)
// }

fn or(exp_list: Vec<Expression>) -> Expression {
    Expression::Or(MultiExpr(exp_list))
}

fn and(exp_list: Vec<Expression>) -> Expression {
    Expression::And(MultiExpr(exp_list))
}

fn symref(s: &str) -> Expression {
    Expression::Simple(Atom::Symbol(s.to_owned()))
}

fn not(expr: Expression) -> Expression {
    Expression::Not(Box::new(expr))
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

    let parsed = parse(&text2parse("bbbb"), &symbol("main"), &rules);
    assert!(parsed.is_ok());


    let parsed = parse(&text2parse("cccc"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}


#[test]
fn parse_and() {
    let rules = map!(symbol("main") => and(vec![lit("aaaa"), lit("bbbb")]));

    let parsed = parse(&text2parse("aaaabbbb"), &symbol("main"), &rules);
    assert!(parsed.is_ok());


    let parsed = parse(&text2parse("aaaa"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("bbbb"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("cccc"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}


#[test]
fn parse_or_and() {
    let rules = map!(symbol("main") =>
            or(vec![
                and(vec![lit("aaaa"), lit("cccc")]),
                and(vec![lit("aaaa"), lit("bbbb")])
            ])
        );

    let parsed = parse(&text2parse("aaaabbbb"), &symbol("main"), &rules);
    assert!(parsed.is_ok());
    let parsed = parse(&text2parse("aaaacccc"), &symbol("main"), &rules);
    assert!(parsed.is_ok());


    let parsed = parse(&text2parse("aaaa"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("bbbb"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("cccc"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}


#[test]
fn parse_symbol() {
    let rules = map!(
            symbol("main") => symref("sa"),
            symbol("sa") => lit("aaaa")
        );
    let parsed = parse(&text2parse("aaaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let rules = map!(
            symbol("main") => symref("inexistent"),
            symbol("sa") => lit("aaaa")
        );
    let parsed = parse(&text2parse("aaaa"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}


#[test]
fn parse_negation() {
    let rules = map!(symbol("main") =>
        and(vec![
            not(and(vec![lit("aaaa"), lit("bbbb")])),
            lit("aaaa"),
        ])
    );

    let parsed = parse(&text2parse("aaaabbbb"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("aaaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());
}

#[test]
fn parse_negation_dot_simulate_klein_start() {
    let rules = map!(symbol("main") =>
        or(vec![
            and(vec![
                    not(lit("~")),
                    dot(),
                    symref("main")
            ]),
            lit("~"),
        ])
    );

    let parsed = parse(&text2parse("123456789~"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("123456789~abcd"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("~123456789~abcd"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("123456789abcd"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}
