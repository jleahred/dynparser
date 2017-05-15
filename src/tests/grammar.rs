use grammar::grammar;


use {symbol, text2parse, parse};


#[test]
fn parse_literal() {
    let parsed = parse(&text2parse(r#"main = "Hello world""#),
                       &symbol("grammar"),
                       &grammar());
    assert!(parsed.is_ok());
}


#[test]
fn parse_symbol() {
    let parsed = parse(&text2parse(r#"
        main = "Hello " world
        world = "world"
    "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}

#[test]
fn multiline() {
    let parsed = parse(&text2parse(r#"
        main    = "hello"
                / "hi"
                / "hola"

        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}

#[test]
fn disorganized() {
    let parsed = parse(&text2parse(r#"
        main = "hello"
            / "hi" / "hola"

        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}

#[test]
fn parenthesis0() {
    let parsed = parse(&text2parse(r#"main = ( hello )"#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}

#[test]
fn parenthesis1() {
    let parsed = parse(&text2parse(r#"
        main = ("hello" / "hi")  " world"

        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}


#[test]
fn multiline_org() {
    let parsed = parse(&text2parse(r#"
        main = ("hello" / "hi")  " world"
            / "bye"

        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}

#[test]
fn one_optional() {
    let parsed = parse(&text2parse(r#"
            main = ("hello" / "hi")  " world"?
        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}


#[test]
fn repetitions() {
    let parsed = parse(&text2parse(r#"
            main         = one_or_more_a / zero_or_many_b
            one_or_more  = "a"+
            zero_or_many = "b"*
        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}

#[test]
fn negation() {
    let parsed = parse(&text2parse(r#"
            main = (!"a" .)* "a"
        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}

#[test]
fn consume_till() {
    let parsed = parse(&text2parse(r#"
            comment = "//" (!"\n" .)*
                    / "/*" (!"*/" .)* "*/"
        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}

#[test]
fn match_chars() {
    let parsed = parse(&text2parse(r#"
            number  = digit+ ("." digit+)?
            digit   = [0-9]
            a_or_b  = [ab]
            id      = [_a-zA-Z] [_a-zA-Z0-9]*

            a_or_b_or_digit  = [ab0-9]
        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}

#[test]
fn simple_recursion() {
    let parsed = parse(&text2parse(r#"
            as  = "a" as
                / "a"

            //  simplified with `+`
            ak = "a"+
        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}

#[test]
fn recursion_parenth() {
    let parsed = parse(&text2parse(r#"
            match_par = "(" match_par ")"
                    / "(" ")"
        "#),
                       &symbol("grammar"),
                       &grammar());

    assert!(parsed.is_ok());
}
