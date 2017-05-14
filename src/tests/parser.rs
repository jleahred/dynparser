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


use parser::tools::*;
use {symbol, text2parse, parse};
use expression::NRep;



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


    let parsed = parse(&text2parse("aaaabbb"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("aaaacbbb"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("aaaa"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("bbbb"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("cccc"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}

#[test]
fn parse_and_and() {
    let rules = map!(symbol("main") =>
                    and(vec![
                        and(vec![lit("aaaa"), lit("bbbb")]),
                        lit("cccc")
                    ])
                );

    let parsed = parse(&text2parse("aaaabbbbcccc"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("cccc"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("aaaacccc"), &symbol("main"), &rules);
    assert!(parsed.is_err());


    let parsed = parse(&text2parse("aaaabbb"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("aaaacbbb"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("aaaa"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("dddd"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}

#[test]
fn parse_and_symbols() {
    let rules = map!(
        symbol("main") => and(vec![symref("a"), lit("bbbb")]),
        symbol("a") => lit("aaaa")
    );

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

#[test]
fn parse_repeating() {
    let rules = map!(symbol("main") =>
        repeat(lit("a"), NRep(1), Some(NRep(5)))
    );

    let parsed = parse(&text2parse("aaaaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("a"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("aaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse(""), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("aaaaaa"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("aaaaaaa"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}

#[test]
fn parse_klein() {
    let rules = map!(symbol("main") =>
        repeat(lit("a"), NRep(0), None)
    );

    let parsed = parse(&text2parse("aaaaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("a"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("aaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse(""), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("aaaaaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());
}


#[test]
fn parse_match_ch() {
    let rules = map!(symbol("main") =>
        repeat(
            match_ch("abc", vec![('r', 't'), ('5', '8')]),
            NRep(1), None
        )
    );

    let parsed = parse(&text2parse("aaaaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("a"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("bbbb"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("ccccc"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("r"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("t"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("5"), &symbol("main"), &rules);
    assert!(parsed.is_ok());
    let parsed = parse(&text2parse("7"), &symbol("main"), &rules);
    assert!(parsed.is_ok());
    let parsed = parse(&text2parse("5678"), &symbol("main"), &rules);
    assert!(parsed.is_ok());


    let parsed = parse(&text2parse("dddd"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("e"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("ff"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("af"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("fa"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("4"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("559"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}


#[test]
fn parse_match_ch_empty_vec() {
    let rules = map!(symbol("main") =>
        repeat(
            match_ch("abc", vec![]),
            NRep(1), None
        )
    );

    let parsed = parse(&text2parse("aaaaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("a"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("bbbb"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("ccccc"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("r"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("t"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("5"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("7"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("5678"), &symbol("main"), &rules);
    assert!(parsed.is_err());


    let parsed = parse(&text2parse("dddd"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("e"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("ff"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("af"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("fa"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("4"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("559"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}


#[test]
fn parse_match_ch_empty_chars() {
    let rules = map!(symbol("main") =>
        repeat(
            match_ch("", vec![('r', 't'), ('5', '8')]),
            NRep(1), None
        )
    );

    let parsed = parse(&text2parse("aaaaa"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("a"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("bbbb"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("ccccc"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("r"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("t"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("5"), &symbol("main"), &rules);
    assert!(parsed.is_ok());
    let parsed = parse(&text2parse("7"), &symbol("main"), &rules);
    assert!(parsed.is_ok());
    let parsed = parse(&text2parse("5678"), &symbol("main"), &rules);
    assert!(parsed.is_ok());


    let parsed = parse(&text2parse("dddd"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("e"), &symbol("main"), &rules);
    assert!(parsed.is_err());
    let parsed = parse(&text2parse("ff"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("af"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("fa"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("4"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("559"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}


#[test]
fn parse_repeat_till() {
    let rules = map!(symbol("main") =>
        and(vec![
            repeat(
                    and(vec![
                        not(lit("a")),
                        dot()
                    ]),
                    NRep(1), None
            ),
            lit("a")
        ])
    );

    let parsed = parse(&text2parse("bbbbbba"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

}


#[test]
fn parse_question_mark() {
    let rules = map!(symbol("main") =>
        repeat(
                lit("a"),
                NRep(0), Some(NRep(1))
        )
    );

    let parsed = parse(&text2parse("a"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse(""), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("aa"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("ab"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("b"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}

#[test]
fn parse_question_mark2() {
    let rules = map!(symbol("main") =>
        and(vec![
            lit("aaa"),
            repeat(
                    lit("b"),
                    NRep(0), Some(NRep(1))
            )
        ])
    );

    let parsed = parse(&text2parse("aaa"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse("aaab"), &symbol("main"), &rules);
    assert!(parsed.is_ok());

    let parsed = parse(&text2parse(""), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("aa"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("ab"), &symbol("main"), &rules);
    assert!(parsed.is_err());

    let parsed = parse(&text2parse("b"), &symbol("main"), &rules);
    assert!(parsed.is_err());
}
