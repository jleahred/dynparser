use grammar::grammar;


use {symbol, text2parse, parse};


#[test]
fn parse_literal() {
    let rules = grammar();
    let parsed = parse(&text2parse(r#"main = "Hello world""#),
                       &symbol("grammar"),
                       &rules);
    assert!(parsed.is_ok());
}


#[test]
fn parse_symbol() {
    let rules = grammar();
    let parsed = parse(&text2parse(r#"
        main = "Hello " world
        world = "world"
    "#),
                       &symbol("grammar"),
                       &rules);

    assert!(parsed.is_ok());
}

#[test]
fn multiline() {
    let rules = grammar();
    let parsed = parse(&text2parse(r#"
        main    = "hello"
                / "hi"
                / "hola"

        "#),
                       &symbol("grammar"),
                       &rules);

    assert!(parsed.is_ok());
}

#[test]
fn disorganized() {
    let rules = grammar();
    let parsed = parse(&text2parse(r#"
        main = "hello"
            / "hi" / "hola"

        "#),
                       &symbol("grammar"),
                       &rules);

    assert!(parsed.is_ok());
}

// #[test]
// fn parenthesis() {
//     let rules = grammar();
//     let parsed = parse(&text2parse(r#"
//         main = ("hello" / "hi")  " world"

//         "#),
//                        &symbol("grammar"),
//                        &rules);

//     println!("{:?} ***************", parsed);
//     assert!(parsed.is_ok());
// }

#[test]
fn parenthesis() {
    let rules = grammar();
    let parsed = parse(&text2parse(r#"main = ( hello )"#),
                       &symbol("grammar"),
                       &rules);
    match parsed.clone() {
        Err(err) => println!("error... {} ___________", err),
        Ok(res) => println!("Ok... {:?} ___________", res),
    };

    assert!(parsed.is_ok());
}
