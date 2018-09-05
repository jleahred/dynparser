//  --------------------------------------------------------------------------

extern crate dynparser;
use dynparser::{peg, rules_from_peg};

fn main() {
    let rules = rules_from_peg(
        r#"

main            =   grammar

grammar         =   rule+

rule            =   _  symbol  _  "="  _  expr  _eol _

expr            =   or

or              =   and         ( _ "/" _  or  )*

and             =   rep_or_neg  ( _1 _ !(symbol _ "=") and )*

rep_or_neg      =   atom_or_par ("*" / "+" / "?")?
                /   "!" atom_or_par

atom_or_par     =   (atom / parenth)

parenth         =   "("  _  expr  _  ")"

atom            =   literal
                /   match
                /   dot
                /   symbol

literal         =   _"  (  "\\" .
                        /  !_" .
                        )*  _"
_"              =   "\""

symbol          =   [_'a-zA-Z0-9] [_'"a-zA-Z0-9]*

eol             =   ("\r\n"  /  "\n"  /  "\r")
_eol            =   " "*  eol

match           =   "["
                        (
                            (mchars+  mbetween*)
                            / mbetween+
                        )
                    "]"

mchars          =   (!"]" !(. "-") .)+
mbetween        =   (.  "-"  .)

dot             =   "."

_               =   (  " "
                        /   eol
                    )*

_1              =   (" " / eol)

"#,
    ).map_err(|e| {
        println!("{}", e);
        panic!("FAIL");
    })
        .unwrap();

    println!("{}", peg::gcode::rust_from_rules(&rules))
}

//  --------------------------------------------------------------------------

// extern crate dynparser;
// use dynparser::{parse, rules_from_peg};

// fn main() {
//     let rules = rules_from_peg(
//         r#"

// main            =   letter letter_or_num+

// letter          =   [a-zA-Z]

// letter_or_num   =   letter
//                 /   number

// number          =   [0-9]

//         "#,
//     ).map_err(|e| {
//         println!("{}", e);
//         panic!("FAIL");
//     })
//         .unwrap();

//     println!("{:#?}", rules);

//     let result = parse("a2AA456bzJ88", &rules);
//     match result {
//         Ok(ast) => println!("{:#?}", ast),
//         Err(e) => println!("Error: {:?}", e),
//     };
// }

// extern crate dynparser;
// use dynparser::{parse, rules_from_peg};

// fn main() {
//     let rules = rules_from_peg(
//         r#"

//     main    =   "a" ( "bc" "c"
//                     / "bcdd"
//                     / b_and_c  d_or_z
//                     )

//     b_and_c =   "b" "c"
//     d_or_z  =   "d" / "z"

//         "#,
//     ).unwrap();

//     assert!(parse("abcz", &rules).is_ok());
//     // assert!(parse("abcdd", &rules).is_ok());
//     // assert!(parse("abcc", &rules).is_ok());
//     // assert!(parse("bczd", &rules).is_err());
// }

// extern crate dynparser;
// use dynparser::{parse, rules_from_peg};

// fn main() {
//     let rules = rules_from_peg(
//         r#"

// main            =   letter letter_or_num+

// letter          =   [a-zA-Z]

// letter_or_num   =   letter
//                 /   number

// number          =   [0-9]

//         "#,
//     ).map_err(|e| {
//         println!("{}", e);
//         panic!("FAIL");
//     })
//         .unwrap();

//     println!("{:#?}", rules);

//     let result = parse("a2Z", &rules);
//     match result {
//         Ok(ast) => println!("{:#?}", ast),
//         Err(e) => println!("Error: {:?}", e),
//     };
// }
