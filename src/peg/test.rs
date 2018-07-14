//-----------------------------------------------------------------------
//
//  mod peg  TEST
//
//-----------------------------------------------------------------------
use parse;
use peg;

#[test]
fn validate_pegs() {
    let peg = r#"
    
    main    = "aaa" "bb" c
    c       = "c"

    "#;

    println!("{:#?}", parse(peg, &peg::rules2parse_peg()));
    assert!(parse(peg, &peg::rules2parse_peg()).is_ok());
}
