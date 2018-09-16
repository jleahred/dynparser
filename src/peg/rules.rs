use parser;

pub(crate) fn parse_peg() -> parser::expression::SetOfRules {
    rules!(
    //     r#"rep_or_neg"# => or!(and!(ref_rule!(r#"atom_or_par"#), rep!(or!(lit!(r#"*"#), lit!(r#"+"#), lit!(r#"?"#)), 0, 1)), and!(lit!(r#"!"#), ref_rule!(r#"atom_or_par"#)))
    //    , r#"_"# => rep!(or!(lit!(r#" "#), ref_rule!(r#"eol"#)), 0)
    //    , r#"eol"# => or!(lit!("\r\n"), lit!("\n"), lit!("\r"))  //  <<<----------------
    //    , r#"grammar"# => rep!(ref_rule!(r#"rule"#), 1)
    //    , r#"expr"# => ref_rule!(r#"or"#)
    //    , r#"atom_or_par"# => or!(ref_rule!(r#"atom"#), ref_rule!(r#"parenth"#))
    //    , r#"mchars"# => rep!(and!(not!(lit!(r#"]"#)), not!(and!(dot!(), lit!(r#"-"#))), dot!()), 1)
    //    , r#"mbetween"# => and!(dot!(), lit!(r#"-"#), dot!())
    //    , r#"_eol"# => and!(rep!(lit!(r#" "#), 0), ref_rule!(r#"eol"#))
    //    , r#"rule"# => and!(ref_rule!(r#"_"#), ref_rule!(r#"symbol"#), ref_rule!(r#"_"#), lit!(r#"="#), ref_rule!(r#"_"#), ref_rule!(r#"expr"#), ref_rule!(r#"_eol"#), ref_rule!(r#"_"#))
    //    , r#"main"# => ref_rule!(r#"grammar"#)
    //    , r#"match"# => and!(lit!(r#"["#), or!(and!(rep!(ref_rule!(r#"mchars"#), 1), rep!(ref_rule!(r#"mbetween"#), 0)), rep!(ref_rule!(r#"mbetween"#), 1)), lit!(r#"]"#))
    //    , r#"lit_esc"# => and!(ref_rule!(r#"_""#), rep!(or!(and!(lit!(r#"\"#), dot!()), and!(not!(ref_rule!(r#"_""#)), dot!())), 0), ref_rule!(r#"_""#))
    //    , r#"_'"# => lit!(r#"'"#)
    //    , r#"literal"# => or!(ref_rule!(r#"lit_noesc"#), ref_rule!(r#"lit_esc"#))
    //    , r#"lit_noesc"# => and!(ref_rule!(r#"_'"#), rep!(and!(not!(ref_rule!(r#"_'"#)), dot!()), 0), ref_rule!(r#"_'"#))
    //    , r#"and"# => and!(ref_rule!(r#"rep_or_neg"#), rep!(and!(ref_rule!(r#"_1"#), ref_rule!(r#"_"#), not!(and!(ref_rule!(r#"symbol"#), ref_rule!(r#"_"#), lit!(r#"="#))), ref_rule!(r#"and"#)), 0))
    //    , r#"parenth"# => and!(lit!(r#"("#), ref_rule!(r#"_"#), ref_rule!(r#"expr"#), ref_rule!(r#"_"#), lit!(r#")"#))
    //    , r#"_""# => lit!(r#"""#)
    //    , r#"symbol"# => and!(ematch!(chlist r#"_"#  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), rep!(ematch!(chlist r#"_'""#  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), 0))
    //    , r#"or"# => and!(ref_rule!(r#"and"#), rep!(and!(ref_rule!(r#"_"#), lit!(r#"/"#), ref_rule!(r#"_"#), ref_rule!(r#"or"#)), 0))
    //    , r#"atom"# => or!(ref_rule!(r#"literal"#), ref_rule!(r#"match"#), ref_rule!(r#"dot"#), ref_rule!(r#"symbol"#))
    //    , r#"dot"# => lit!(r#"."#)
    //    , r#"_1"# => or!(lit!(r#" "#), ref_rule!(r#"eol"#))
         r#"rep_or_neg"# => or!(and!(ref_rule!(r#"atom_or_par"#), rep!(or!(lit!(r#"*"#), lit!(r#"+"#), lit!(r#"?"#)), 0, 1)), and!(lit!(r#"!"#), ref_rule!(r#"atom_or_par"#)))
       , r#"_'"# => lit!(r#"'"#)
       , r#"hexd"# => ematch!(chlist r#""#  , from '0', to '9' , from 'A', to 'F' )
       , r#"_eol"# => and!(rep!(lit!(r#" "#), 0), ref_rule!(r#"eol"#))
       , r#"grammar"# => rep!(ref_rule!(r#"rule"#), 1)
       , r#"symbol"# => and!(ematch!(chlist r#"_"#  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), rep!(ematch!(chlist r#"_'""#  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), 0))
       , r#"mchars"# => rep!(and!(not!(lit!(r#"]"#)), not!(and!(dot!(), lit!(r#"-"#))), dot!()), 1)
       , r#"lit_noesc"# => and!(ref_rule!(r#"_'"#), rep!(and!(not!(ref_rule!(r#"_'"#)), dot!()), 0), ref_rule!(r#"_'"#))
       , r#"_1"# => or!(lit!(r#" "#), ref_rule!(r#"eol"#))
       , r#"eol"# => or!(lit!("\r\n"), lit!("\n"), lit!("\r"))
       , r#"mbetween"# => and!(dot!(), lit!(r#"-"#), dot!())
       , r#"or"# => and!(ref_rule!(r#"and"#), rep!(and!(ref_rule!(r#"_"#), lit!(r#"/"#), ref_rule!(r#"_"#), ref_rule!(r#"or"#)), 0))
       , r#"rule"# => and!(ref_rule!(r#"_"#), ref_rule!(r#"symbol"#), ref_rule!(r#"_"#), lit!(r#"="#), ref_rule!(r#"_"#), ref_rule!(r#"expr"#), ref_rule!(r#"_eol"#), ref_rule!(r#"_"#))
       , r#"esc_char"# => or!(and!(lit!(r#"\0x"#), ref_rule!(r#"hexd"#), ref_rule!(r#"hexd"#)), and!(lit!(r#"\"#), ematch!(chlist r#"nrt"\"# )))
       , r#"expr"# => ref_rule!(r#"or"#)
       , r#"dot"# => lit!(r#"."#)
       , r#"atom_or_par"# => or!(ref_rule!(r#"atom"#), ref_rule!(r#"parenth"#))
       , r#"_"# => rep!(or!(lit!(r#" "#), ref_rule!(r#"eol"#)), 0)
       , r#"literal"# => or!(ref_rule!(r#"lit_noesc"#), ref_rule!(r#"lit_esc"#))
       , r#"main"# => ref_rule!(r#"grammar"#)
       , r#"match"# => and!(lit!(r#"["#), or!(and!(rep!(ref_rule!(r#"mchars"#), 1), rep!(ref_rule!(r#"mbetween"#), 0)), rep!(ref_rule!(r#"mbetween"#), 1)), lit!(r#"]"#))
       , r#"and"# => and!(ref_rule!(r#"rep_or_neg"#), rep!(and!(ref_rule!(r#"_1"#), ref_rule!(r#"_"#), not!(and!(ref_rule!(r#"symbol"#), ref_rule!(r#"_"#), lit!(r#"="#))), ref_rule!(r#"and"#)), 0))
       , r#"atom"# => or!(ref_rule!(r#"literal"#), ref_rule!(r#"match"#), ref_rule!(r#"dot"#), ref_rule!(r#"symbol"#))
       , r#"_""# => lit!(r#"""#)
       , r#"lit_esc"# => and!(ref_rule!(r#"_""#), rep!(or!(ref_rule!(r#"esc_char"#), and!(not!(ref_rule!(r#"_""#)), dot!())), 0), ref_rule!(r#"_""#))
       , r#"parenth"# => and!(lit!(r#"("#), ref_rule!(r#"_"#), ref_rule!(r#"expr"#), ref_rule!(r#"_"#), lit!(r#")"#))
    )
}

//  ------------------------------------------------------------------------
//  ------------------------------------------------------------------------
//
//  this is the first version of code to parse the peg grammar
//  it was, obviously written by hand
// pub(crate) fn parse_peg_first() -> parser::expression::SetOfRules {
//     rules!(

//         "main"      =>       ref_rule!("grammar"),

//         "grammar"   =>       rep!(ref_rule!("rule"), 1),

//         "rule"      =>       and!(
//                                  ref_rule!("_"), ref_rule!("symbol"),
//                                  ref_rule!("_"), lit! ("="),
//                                  ref_rule!("_"), ref_rule!("expr"),
//                                 ref_rule!("_eol"),
//                                 ref_rule!("_")
//                              ),

//         "expr"      =>      ref_rule!("or"),

//         "or"        =>      and!(
//                                 ref_rule!("and"),
//                                 rep!(
//                                     and!(
//                                         ref_rule!("_"), lit!("/"),
//                                         ref_rule!("_"), ref_rule!("or")
//                                     ),
//                                     0
//                                 )
//                             ),

//         "and"       =>     and!(
//                                 ref_rule!("rep_or_neg"),
//                                 rep!(
//                                     and!(
//                                         ref_rule!("_1"), ref_rule!("_"),
//                                         not!(and!(
//                                                 ref_rule!("symbol"),
//                                                 ref_rule!("_"), lit! ("=")
//                                         )),
//                                         ref_rule!("and")
//                                     ),
//                                     0
//                                 )
//                             ),

//         "rep_or_neg" =>     or!(
//                                 and!(
//                                     ref_rule!("atom_or_par"),
//                                     rep!(
//                                         or!(
//                                             lit!("*"),
//                                             lit!("+"),
//                                             lit!("?")
//                                         )
//                                         , 0, 1
//                                     )
//                                 ),
//                                 and!(
//                                     lit!("!"),
//                                     ref_rule!("atom_or_par")
//                                 )
//                             ),

//         "atom_or_par" =>    or!(
//                                 ref_rule!("atom"),
//                                 ref_rule!("parenth")
//                             ),

//         "parenth"       =>  and!(
//                                 lit!("("),
//                                 ref_rule!("_"),
//                                 ref_rule!("expr"),
//                                 ref_rule!("_"),
//                                 lit!(")")
//                             ),

//         "atom"          =>  or!(
//                                 ref_rule!("literal"),
//                                 ref_rule!("match"),
//                                 ref_rule!("dot"),
//                                 ref_rule!("symbol")
//                             ),

//         "literal"       =>  and!(
//                                 ref_rule!(r#"_""#),
//                                 rep!(
//                                     and!(
//                                         not!(
//                                             ref_rule!(r#"_""#)
//                                         ),
//                                         dot!()
//                                     )
//                                 , 0
//                             ),
//                                 ref_rule!(r#"_""#)
//                             ),

//         r#"_""#         =>  lit!(r#"""#),

//         "match"         =>  and!(
//                                 lit!("["),
//                                 or!(
//                                     and!(
//                                         rep!(ref_rule!("mchars"), 1),
//                                         rep!(ref_rule!("mbetween"), 0)
//                                     ),
//                                     rep!(ref_rule!("mbetween"), 1)
//                                 ),
//                                 lit!("]")
//                             ),

//         "mchars"        =>  rep!(
//                                 and!(
//                                     not!(lit!("]")),
//                                     not!(and!(dot!(), lit!("-"))),
//                                     dot!())
//                                 ,1
//                             ),

//         "mbetween"      =>  and!(dot!(), lit!("-"), dot!()),

//         "dot"           =>  lit!("."),

//         "symbol"        =>  and!(
//                                 ematch!(    chlist "_'",
//                                         from 'a', to 'z',
//                                         from 'A', to 'Z',
//                                         from '0', to '9'
//                                 ),
//                                 rep!(
//                                     ematch!(    chlist "_'\"",
//                                             from 'a', to 'z',
//                                             from 'A', to 'Z',
//                                             from '0', to '9'
//                                     ),
//                                     0
//                                 )
//                             ),

//         "_"             =>  rep!(   or!(
//                                         lit!(" "),
//                                         ref_rule!("eol")
//                                         // ref_rule!("comment")
//                                     )
//                                     , 0
//                             ),

//         "_eol"          =>  and!(
//                                 rep!(   or!(
//                                         lit!(" ")
//                                     )
//                                     , 0
//                                 ),
//                                 ref_rule!("eol")
//                             ),

//         "_1"            =>  or!(
//                                         lit!(" "),
//                                         ref_rule!("eol")
//                                         // ref_rule!("comment")
//                                 ),

//         "spaces"        =>  rep!(lit!(" "), 0),

//         "eol"          =>   or!(
//                                     lit!("\r\n"),
//                                     lit!("\n"),
//                                     lit!("\r")
//                                 )

//         // "comment"       =>  or!(
//         //                         and!(
//         //                             lit!("//"),
//         //                             rep!(
//         //                                 and!(
//         //                                     not!(ref_rule!("eol")),
//         //                                     dot!()
//         //                                 )
//         //                                 , 0
//         //                             ),
//         //                             ref_rule!("eol")
//         //                         ),
//         //                         and!(
//         //                             lit!("/*"),
//         //                             rep!(
//         //                                 and!(
//         //                                     not!(lit!("*/")),
//         //                                     dot!()
//         //                                 )
//         //                                 , 0
//         //                             ),
//         //                             lit!("*/")
//         //                         )
//         //                 )
//     )
// }

//  And this is the first autogenerated code  :-)  working
//     "rep_or_neg" => or!(and!(ref_rule!("atom_or_par"), rep!(or!(lit!("*"), lit!("+"), lit!("?")), 0, 1)), and!(lit!("!"), ref_rule!("atom_or_par")))
//    , "literal" => and!(ref_rule!("_\""), rep!(or!(and!(lit!("\\"), dot!()), and!(not!(ref_rule!("_\"")), dot!())), 0), ref_rule!("_\""))
//    , "eol" => or!(lit!("\r\n"), lit!("\n"), lit!("\r"))
//    , "mchars" => rep!(and!(not!(lit!("]")), not!(and!(dot!(), lit!("-"))), dot!()), 1)
//    , "mbetween" => and!(dot!(), lit!("-"), dot!())
//    , "atom_or_par" => or!(ref_rule!("atom"), ref_rule!("parenth"))
//    , "dot" => lit!(".")
//    , "or" => and!(ref_rule!("and"), rep!(and!(ref_rule!("_"), lit!("/"), ref_rule!("_"), ref_rule!("or")), 0))
//    , "_eol" => and!(rep!(lit!(" "), 0), ref_rule!("eol"))
//    , "rule" => and!(ref_rule!("_"), ref_rule!("symbol"), ref_rule!("_"), lit!("="), ref_rule!("_"), ref_rule!("expr"), ref_rule!("_eol"), ref_rule!("_"))
//    , "symbol" => and!(ematch!(chlist "_'"  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), rep!(ematch!(chlist "_'\""  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), 0))
//    , "main" => ref_rule!("grammar")
//    , "match" => and!(lit!("["), or!(and!(rep!(ref_rule!("mchars"), 1), rep!(ref_rule!("mbetween"), 0)), rep!(ref_rule!("mbetween"), 1)), lit!("]"))
//    , "grammar" => rep!(ref_rule!("rule"), 1)
//    , "and" => and!(ref_rule!("rep_or_neg"), rep!(and!(ref_rule!("_1"), ref_rule!("_"), not!(and!(ref_rule!("symbol"), ref_rule!("_"), lit!("="))), ref_rule!("and")), 0))
//    , "_" => rep!(or!(lit!(" "), ref_rule!("eol")), 0)
//    , "parenth" => and!(lit!("("), ref_rule!("_"), ref_rule!("expr"), ref_rule!("_"), lit!(")"))
//    , "expr" => ref_rule!("or")
//    , "_\"" => lit!("\"")
//    , "atom" => or!(ref_rule!("literal"), ref_rule!("match"), ref_rule!("dot"), ref_rule!("symbol"))
//    , "_1" => or!(lit!(" "), ref_rule!("eol"))
