//! Here it's the peg grammar to parse the peg input  ;-P
//!
//! There is also a function to print the rust source code
//!
//! It's used in order to develop myself
//!
//! A parser for the parser (you know)
//!
//! To generate the code to parse the peg, you just have to run...
//!
//! ```
//! extern crate dynparser;
//! use dynparser::peg::peg2code;
//!
//! fn main() {
//!     peg2code::print_rules2parse_peg();
//! }
//! ```
//!
//! And the result, has to be pasted in peg::rules.rs
//!

use {peg, rules_from_peg};

fn peg2code() -> &'static str {
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
                                (mchars  mbetween*)
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
    "#
}

/// A parser for the parser.
///
/// It will take the peg grammar to parse peg grammars
///
pub fn print_rules2parse_peg() {
    let rules = rules_from_peg(peg2code())
        .map_err(|e| {
            println!("{}", e);
            panic!("FAIL");
        }).unwrap();

    println!("{}", peg::gcode::rust_from_rules(&rules))
}
