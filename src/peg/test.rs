//-----------------------------------------------------------------------
//
//  mod peg  TEST
//
//-----------------------------------------------------------------------
use parse;
use peg;

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

    //println!("{:#?}", parse(peg, &peg::rules2parse_peg()));
    assert!(parse(peg, &peg::rules2parse_peg()).is_ok());
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

    assert!(parse(peg, &peg::rules2parse_peg()).is_ok());
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

    assert!(parse(peg, &peg::rules2parse_peg()).is_err());
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

    assert!(parse(peg, &peg::rules2parse_peg()).is_err());
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

    assert!(parse(peg, &peg::rules2parse_peg()).is_err());
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

    assert!(parse(peg, &peg::rules2parse_peg()).is_err());
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

    assert!(parse(peg, &peg::rules2parse_peg()).is_err());
}
