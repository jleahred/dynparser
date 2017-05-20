use parser::tools::*;
use symbol;
use Rules;


macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);


pub fn grammar() -> Rules {
    map!(
        symbol("grammar")   =>  repeat(symref("rule"), NRep(1), None)

        , symbol("rule")    =>  and(vec![
                                        symref("_"),
                                        symref("symbol"),
                                        symref("_"),
                                        lit   ("="),
                                        symref("_"),
                                        symref("expr"),
                                        or(vec![
                                            symref("_eol"),
                                            symref("eof")
                                        ])
                                    ])

        , symbol("expr")    =>    symref("or_expr")

        , symbol("or_expr") =>    and(vec![
                                        symref("and_expr"),
                                        repeat(
                                            and(vec![
                                                symref("_"),
                                                lit   ("/"),
                                                symref("_"),
                                                symref("or_expr")
                                            ]),
                                            NRep(0), None
                                        )
                                    ])

        , symbol("and_expr") =>    and(vec![
                                        symref("compl_expr"),
                                        repeat(
                                            and(vec![
                                                lit   (" "),
                                                symref("_"),
                                                symref("and_expr")
                                            ]),
                                            NRep(0), None
                                        )
                                    ])

        , symbol("compl_expr") =>   or(vec![
                                        and(vec![
                                            symref("simpl_par"),
                                            repeat(
                                                or(vec![
                                                    lit("*"),
                                                    lit("+"),
                                                    lit("?"),
                                                ]),
                                            NRep(0), Some(NRep(1))),
                                        ]),
                                        and(vec![
                                            lit("!"),
                                            symref("simpl_par")
                                        ]),
                                    ])

        , symbol("simpl_par") =>    or(vec![
                                        symref("simple"),
                                        symref("parenth_expr")
                                    ])

        , symbol("parenth_expr") =>    and(vec![
                                        lit("("),
                                        symref("_"),
                                        symref("expr"),
                                        symref("_"),
                                        lit(")")
                                    ])

        , symbol("simple") =>       symref("atom")

        , symbol("atom") =>         or(vec![
                                        symref("literal"),
                                        symref("match"),
                                        symref("dot"),
                                        symref("symbol")
                                    ])

        , symbol("literal") =>      and(vec![
                                        lit("\""),
                                        repeat(
                                            and(vec![
                                                not(lit("\"")),
                                                dot()
                                            ]),
                                            NRep(1), None
                                        ),
                                        lit("\"")
                                    ])

        , symbol("match") =>        and(vec![
                                        lit("["),
                                        repeat(
                                            or(vec![
                                                and(vec![
                                                    dot(),
                                                    lit("-"),
                                                    dot()
                                                ]),
                                                and(vec![
                                                    not(lit("]")),
                                                    dot()
                                                ])
                                            ]),
                                            NRep(1), None
                                        ),
                                        lit("]")
                                    ])

        , symbol("dot") =>          lit(".")

        , symbol("symbol") =>       repeat(
                                        match_ch("_",
                                            vec![('a', 'z'),
                                                 ('A', 'Z'),
                                                 ('0','9')]),
                                        NRep(1), None)

        , symbol("_") =>            repeat(
                                        or(vec![
                                            lit(" "),
                                            lit("\n"),
                                            symref("comment")
                                        ]),
                                        NRep(0), None
                                    )

        , symbol("_eol") =>         and(vec![
                                        or(vec![
                                            repeat(lit(" "), NRep(0), None),
                                            symref("comment")
                                        ]),
                                        symref("_")
                                    ])

        , symbol("comment") =>      or(vec![
                                        and(vec![
                                            lit("//"),
                                            repeat(
                                                and(vec![
                                                    not(lit("\n")),
                                                    dot()
                                                ]),
                                                NRep(0), None
                                            ),
                                            lit("\n")
                                        ]),
                                        and(vec![
                                            lit("/*"),
                                            and(vec![
                                                not(lit("*/")),
                                                dot()
                                            ]),
                                            lit("*/")
                                        ]),
                                    ])

    )
}