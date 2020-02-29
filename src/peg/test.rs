//-----------------------------------------------------------------------
//
//  mod peg  TEST
//
//-----------------------------------------------------------------------
use crate::parse;
use crate::peg;

#[test]
fn validate_peg1() {
    let peg = r#"
    
    main    = "aaa" "bb" c*  / ((d f)+ / es)
    c       = c'
    c'      = c''
    c''     = "c"
    d       = "d"
    f       = "f"
    es       = "e"+

    "#;

    assert!(parse(peg, &peg::rules::parse_peg()).is_ok());
}

#[test]
fn validate_peg2() {
    let peg = r#"
    
    main    =   "aaa" 
            /   "bb" c*  
            / ((d f)+ / es)

    c       = c'
    c'      = c''
    c''     = "c"
    d       = "d"
    f       = "f"
    es       = "e"+

    "#;

    assert!(parse(peg, &peg::rules::parse_peg()).is_ok());
}

#[test]
fn invalid_peg1() {
    let peg = r#"
    
    main    = "aaa" "bb" c *  / ((d f)+ / es)
    c       = c'
    c'      = c''
    c''     = "c"
    d       = "d"
    f       = "f"
    es       = "e"+

    "#;

    assert!(parse(peg, &peg::rules::parse_peg()).is_err());
}

#[test]
fn invalid_peg2() {
    let peg = r#"
    
    main    =   "aaa" 
            /   "bb" c*  
            / ((d f)+ / es
            
    c       = c'
    c'      = c''
    c''     = "c"
    d       = "d"
    f       = "f"
    es       = "e"+

    "#;

    assert!(parse(peg, &peg::rules::parse_peg()).is_err());
}

#[test]
fn validate_peg3() {
    let peg = r#"
    
    main    = "aaa "bb" c*  / ((d f)+ / es)
    c       = c'
    c'      = c''
    c''     = "c"
    d       = "d"
    f       = "f"
    es       = "e"+

    "#;

    assert!(parse(peg, &peg::rules::parse_peg()).is_err());
}

#[test]
fn validate_peg4() {
    let peg = r#"
    
    ma in    = "aaa" "bb" c*  / ((d f)+ / es)
    c       = c'
    c'      = c''
    c''     = "c"
    d       = "d"
    f       = "f"
    es       = "e"+

    "#;

    assert!(parse(peg, &peg::rules::parse_peg()).is_err());
}

#[test]
fn validate_peg5() {
    let peg = r#"
    
    main    = "aaa" "bb" c*  / ((d f)+* / es)
    c       = c'
    c'      = c''
    c''     = "c"
    d       = "d"
    f       = "f"
    es       = "e"+

    "#;

    assert!(parse(peg, &peg::rules::parse_peg()).is_err());
}

#[test]
fn parse_literal() {
    let peg = r#"

    main    = "hello"

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("hello", &rules).is_ok());
}

#[test]
fn parse_and_literal() {
    let peg = r#"

    main    = "hello"   " "   "world"

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("hello world", &rules).is_ok());
}

#[test]
fn parse_or_literal() {
    let peg = r#"

    main    =   "hello" " "  "world"   
            /   "hola"  " "  "mundo"
            /   "hola"

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("hello world", &rules).is_ok());
    assert!(parse("hola", &rules).is_ok());
    assert!(parse("hola mundo", &rules).is_ok());
    assert!(parse("bye", &rules).is_err());
}

#[test]
fn parse_ref_rule() {
    let peg = r#"

    main    = "hello"  " "   world

    world   = "world"

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("hello world", &rules).is_ok());
    assert!(parse("bye", &rules).is_err());
}

#[test]
fn parse_parenth() {
    let peg = r#"

    main    =   "hello"  " " (world / "mars")

    world   =   "world"

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("hello world", &rules).is_ok());
    assert!(parse("hello mars", &rules).is_ok());
    assert!(parse("hello pluto", &rules).is_err());
    assert!(parse("hello", &rules).is_err());
}

#[test]
fn parse_klean() {
    let peg = r#"

    main    =   "a"*

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("aaaaaa", &rules).is_ok());
    assert!(parse("a", &rules).is_ok());
    assert!(parse("", &rules).is_ok());
    assert!(parse("bbb", &rules).is_err());
    assert!(parse("b", &rules).is_err());
    assert!(parse("aab", &rules).is_err());
}

#[test]
fn parse_one_or_more() {
    let peg = r#"

    main    =   "a"+

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("aaaaaa", &rules).is_ok());
    assert!(parse("a", &rules).is_ok());
    assert!(parse("", &rules).is_err());
    assert!(parse("bbb", &rules).is_err());
    assert!(parse("b", &rules).is_err());
    assert!(parse("aab", &rules).is_err());
}

#[test]
fn parse_one_or_zero() {
    let peg = r#"

    main    =   "a"?

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("aaaaaa", &rules).is_err());
    assert!(parse("a", &rules).is_ok());
    assert!(parse("", &rules).is_ok());
    assert!(parse("bbb", &rules).is_err());
    assert!(parse("b", &rules).is_err());
    assert!(parse("ab", &rules).is_err());
}

#[test]
fn parse_negation() {
    let peg = r#"

    main    =   !"bye"  "hello"

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("hello", &rules).is_ok());
    assert!(parse("bye", &rules).is_err());
    assert!(parse("hi", &rules).is_err());
    assert!(parse("bye hello", &rules).is_err());
}

#[test]
fn parse_dot() {
    let peg = r#"

    main    =   . . . . .

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("hello", &rules).is_ok());
    assert!(parse("hola.", &rules).is_ok());
    assert!(parse("hi", &rules).is_err());
    assert!(parse("bye hello", &rules).is_err());
}

#[test]
fn parse_dot_with_rep() {
    let peg = r#"

    main    =   (!"h" .)*  "hello"

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("-----hello", &rules).is_ok());
    assert!(parse("_hello", &rules).is_ok());
    assert!(parse("hello", &rules).is_ok());
    assert!(parse("Hola hello", &rules).is_ok());
    assert!(parse("hola hello", &rules).is_err());
    assert!(parse("hola_hello", &rules).is_err());
    assert!(parse("Hello", &rules).is_err());
    assert!(parse("bye", &rules).is_err());
    assert!(parse("hola", &rules).is_err());
    assert!(parse("hello hola", &rules).is_err());
}

#[test]
fn peg_incorrect_match() {
    let peg = r#"

    main    =   [A-Zab]

    "#;

    assert!(peg::rules_from_peg(peg).is_err());
}

#[test]
fn test_match() {
    let peg = r#"

    main    =   [123A-X]+

    "#;

    let rules = peg::rules_from_peg(peg).unwrap();

    assert!(parse("1111", &rules).is_ok());
    assert!(parse("222311", &rules).is_ok());
    assert!(parse("ABCDEF", &rules).is_ok());
    assert!(parse("ABCDEF4", &rules).is_err());
    assert!(parse("", &rules).is_err());
    assert!(parse("1234", &rules).is_err());
    assert!(parse("Z", &rules).is_err());
    assert!(parse("ABZ", &rules).is_err());
}
