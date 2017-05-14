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
    let parsed = parse(&text2parse(r#"main=hello?"#),
                       &symbol("grammar"),
                       &grammar());

    match parsed.clone() {
        Err(err) => println!("error... {} ___________", err),
        Ok(res) => println!("Ok... {:?} ___________", res),
    };
    assert!(parsed.is_ok());
}
