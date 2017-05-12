use grammar::grammar;


use {symbol, text2parse, parse};


#[test]
fn parse_literal() {
    let rules = grammar();
    let parsed = parse(&text2parse(r#"main = "Hello world""#),
                       &symbol("main"),
                       &rules);
    assert!(parsed.is_ok());
}
