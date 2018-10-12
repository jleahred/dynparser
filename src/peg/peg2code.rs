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

fn text_peg2code() -> &'static str {
    r#"
    /*      A peg grammar to parse peg grammars
     *
     */

    main            =   grammar

    grammar         =   rule+

    rule            =   _  symbol  _  '='  _  expr  _eol _

    expr            =   or

    or              =   and         ( _  '/'  _  (error  /  or)  )?
    error           =   'error' _  '('  _  literal  _  ')'

    and             =   rep_or_neg  ( _1 _ !(symbol _ '=') and )*
    _1              =   (' ' / eol)     //  this is the and separator

    rep_or_neg      =   atom_or_par ('*' / '+' / '?')?
                    /   '!' atom_or_par

    atom_or_par     =   (atom / parenth)

    parenth         =   '('  _  expr  _  ')'

    atom            =   literal
                    /   match
                    /   dot
                    /   symbol

    literal         =  lit_noesc  /  lit_esc

    lit_noesc       =   _'   (  !_' .  )*   _'
    _'              =   "'"

    lit_esc         =   _"
                            (   esc_char
                            /   hex_char
                            /   !_" .
                            )*
                        _"
    _"              =   '"'

    esc_char        =   '\r'
                    /   '\n'
                    /   '\t'
                    /   '\\'
                    /   '\"'

    hex_char        =   '\0x' [0-9A-F] [0-9A-F]

    symbol          =   [_a-zA-Z0-9] [_'"a-zA-Z0-9]*

    eol             =   ("\r\n"  /  "\n"  /  "\r")
    _eol            =   (' ' / comment)*  eol

    match           =   '['
                            (
                                (mchars  mbetween*)
                                / mbetween+
                            )
                        ']'

    mchars          =   (!']' !(. '-') .)+
    mbetween        =   (.  '-'  .)

    dot             =   '.'

    _               =   (  ' '
                        /   eol
                        /   comment
                        )*

    comment         =   line_comment
                    /   mline_comment

    line_comment    =   '//' (!eol .)*  eol
    mline_comment   =   '/*' (!'*/' .)* '*/'
    "#
}

/// A parser for the parser.
///
/// It will take the peg grammar to parse peg grammars
///
pub fn print_rules2parse_peg() {
    let rules = rules_from_peg(text_peg2code())
        .map_err(|e| {
            println!("{}", e);
            panic!("FAIL");
        }).unwrap();

    println!("{}", peg::gcode::rust_from_rules(&rules))
}
