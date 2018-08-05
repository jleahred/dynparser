use parser;

//  ------------------------------------------------------------------------
//  ------------------------------------------------------------------------
//
//  this is the first version of code to parse the peg grammar
//  it was, obviously written by hand
pub(crate) fn parse_peg() -> parser::expression::SetOfRules {
    rules!(

        "main"      =>       ref_rule!("grammar"),

        "grammar"   =>       rep!(ref_rule!("rule"), 1),

        "rule"      =>       and!(
                                 ref_rule!("_"), ref_rule!("symbol"),
                                 ref_rule!("_"), lit! ("="),
                                 ref_rule!("_"), ref_rule!("expr"),
                                ref_rule!("_eol"),
                                ref_rule!("_")                                                
                             ),

        "expr"      =>      ref_rule!("or"),

        "or"        =>      and!(
                                ref_rule!("and"),
                                rep!(
                                    and!(
                                        ref_rule!("_"), lit!("/"),
                                        ref_rule!("_"), ref_rule!("or")
                                    ),
                                    0
                                )
                            ),

        "and"       =>     and!(
                                ref_rule!("rep_or_neg"),
                                rep!(
                                    and!(
                                        ref_rule!("_1"), ref_rule!("_"), 
                                        not!(and!(
                                                ref_rule!("symbol"),
                                                ref_rule!("_"), lit! ("=")
                                        )),
                                        ref_rule!("and")
                                    ),
                                    0
                                )
                            ),

        "rep_or_neg" =>     or!(
                                and!(
                                    ref_rule!("atom_or_par"),
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
                                    ref_rule!("atom_or_par")
                                )
                            ),

        "atom_or_par" =>    or!(
                                ref_rule!("atom"),
                                ref_rule!("parenth")
                            ),

        "parenth"       =>  and!(
                                lit!("("),
                                ref_rule!("_"),
                                ref_rule!("expr"),
                                ref_rule!("_"),
                                lit!(")")
                            ),

        "atom"          =>  or!(
                                ref_rule!("literal"),
                                ref_rule!("match"),
                                ref_rule!("dot"),
                                ref_rule!("symbol")
                            ),

        "literal"       =>  and!(
                                ref_rule!(r#"_""#),
                                rep!(
                                    and!(
                                        not!(
                                            ref_rule!(r#"_""#)
                                        ),
                                        dot!()
                                    )
                                , 0
                            ),
                                ref_rule!(r#"_""#)
                            ),

        r#"_""#         =>  lit!(r#"""#),

        "match"         =>  and!(
                                lit!("["),
                                or!(
                                    and!(
                                        rep!(ref_rule!("mchars"), 1),
                                        rep!(ref_rule!("mbetween"), 0)
                                    ),
                                    rep!(ref_rule!("mbetween"), 1)
                                ),
                                lit!("]")
                            ),

        "mchars"        =>  rep!(
                                and!(
                                    not!(lit!("]")), 
                                    not!(and!(dot!(), lit!("-"))),
                                    dot!())
                                ,1
                            ),

        "mbetween"      =>  and!(dot!(), lit!("-"), dot!()),

        "dot"           =>  lit!("."),

        "symbol"        =>  and!(
                                ematch!(    chlist "_'",
                                        from 'a', to 'z',
                                        from 'A', to 'Z',
                                        from '0', to '9'
                                ),
                                rep!(
                                    ematch!(    chlist "_'\"",
                                            from 'a', to 'z',
                                            from 'A', to 'Z',
                                            from '0', to '9'
                                    ),
                                    0
                                )
                            ),

        "_"             =>  rep!(   or!(
                                        lit!(" "),
                                        ref_rule!("eol")
                                        // ref_rule!("comment")
                                    )
                                    , 0
                            ),

        "_eol"          =>  and!(
                                rep!(   or!(
                                        lit!(" ")
                                    )
                                    , 0
                                ),
                                ref_rule!("eol")
                            ),

        "_1"            =>  or!(
                                        lit!(" "),
                                        ref_rule!("eol")
                                        // ref_rule!("comment")
                                ),

        "spaces"        =>  rep!(lit!(" "), 0),

        "eol"          =>   or!(
                                    lit!("\r\n"),
                                    lit!("\n"),
                                    lit!("\r")
                                )

        // "comment"       =>  or!(
        //                         and!(
        //                             lit!("//"),
        //                             rep!(
        //                                 and!(
        //                                     not!(ref_rule!("eol")),
        //                                     dot!()
        //                                 )
        //                                 , 0
        //                             ),
        //                             ref_rule!("eol")
        //                         ),
        //                         and!(
        //                             lit!("/*"),
        //                             rep!(
        //                                 and!(
        //                                     not!(lit!("*/")),
        //                                     dot!()
        //                                 )
        //                                 , 0
        //                             ),
        //                             lit!("*/")
        //                         )
        //                 )
    )
}
