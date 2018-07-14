#![warn(missing_docs)]
//! Module with functions to generate rules from PEG grammar
//!

use parser;

#[cfg(test)]
mod test;

// -------------------------------------------------------------------------------------
//  A P I

/// Given a ```peg``` set of rules on an string, it will generate
/// the set of rules to use in the parser
pub fn rules_from_peg(_peg: &str) -> parser::expression::SetOfRules {
    rules2parse_peg()
}

//  A P I
// -------------------------------------------------------------------------------------

fn rules2parse_peg<'a>() -> parser::expression::SetOfRules<'a> {
    rules!(

        "main"      =>       rule!("grammar"),

        "grammar"   =>       rep!(rule!("rule"), 1),

        "rule"      =>       and!(
                                 rule!("_"), rule!("symbol") ,
                                 rule!("_"), lit! ("="),
                                 rule!("_"), rule!("expr"),
                                             or!(
                                                 rule!("_"),
                                                 rule!("eof")
                                             ),
                                 rule!("_")                                                
                             ),

        "expr"      =>      rule!("or"),

        "or"        =>      and!(
                                rule!("and"),
                                rep!(
                                    and!(
                                        rule!("_"), lit!("/"),
                                        rule!("_"), rule!("or")
                                    ),
                                    0
                                )
                            ),

        "and"       =>     and!(
                                rule!("rep_or_neg"),
                                rep!(
                                    and!(
                                        rule!("_"), lit!("/"),
                                        lit!(" "),  rule!("_"), rule!("and")
                                    ),
                                    0
                                )
                            ),

        "rep_or_neg" =>     or!(
                                and!(
                                    rule!("atom_or_par"),
                                    rep!(
                                        or!(
                                            lit!("*"),
                                            lit!("+"),
                                            lit!("?")
                                        )
                                        , 0, 1
                                    )
                                ),
                                and!(
                                    lit!("!"),
                                    rule!("atom_or_par")
                                )
                            ),

        "atom_or_par" =>    or!(
                                rule!("atom"),
                                rule!("paren")
                            ),

        "atom"          =>  or!(
                                rule!("literal"),
                                rule!("match"),
                                rule!("dot"),
                                rule!("symbol")
                            ),

        "literal"       =>  and!(
                                lit!("\u{34}"),
                                rep!(
                                    and!(
                                        not!(
                                            lit!("\u{34}")
                                        ),
                                        dot!()
                                    )
                                    , 0
                                ),
                                lit!("\u{34}")
                            ),

        "match"         =>  and!(
                                lit!("["),
                                or!(
                                    and!(dot!(), lit!("-"), dot!()),
                                    rep!(
                                        and!(not!(lit!("]")), dot!())
                                        ,1
                                    )
                                )
                            ),
        
        "dot"           =>  lit!("."),

        "symbol"        =>  rep!(
                                ematch!(    chlist "_'",
                                         from 'a', to 'z',
                                         from 'A', to 'Z',
                                         from '0', to '9'
                                ),
                                1
                            ),

        "_"             =>  rep!(   or!(
                                        lit!(" "),
                                        rule!("eol"),
                                        rule!("comment")
                                    )
                                    , 0
                            ),

        "eol"          =>   or!(
                                    lit!("\r\n"),
                                    lit!("\n"),
                                    lit!("\r")
                                ),
        "comment"       =>  or!(
                                and!(
                                    lit!("//"),
                                    rep!(
                                        and!(
                                            not!(rule!("eol")),
                                            dot!()
                                        )
                                        , 0
                                    ),
                                    rule!("eol")
                                ),
                                and!(
                                    lit!("/*"),
                                    rep!(
                                        and!(
                                            not!(lit!("*/")),
                                            dot!()
                                        )
                                        , 0
                                    ),
                                    lit!("*/")
                                )
                        )
    )
}
