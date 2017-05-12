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
                                        symref("rule"),
                                        symref("_"),
                                        lit   ("="),
                                        symref("_"),
                                        symref("expr"),
                                        symref("_")
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
                                            lit("!"),
                                            symref("simpl_par")
                                        ]),
                                        and(vec![
                                            symref("simpl_par"),
                                            or(vec![
                                                lit("*"),
                                                lit("+")
                                            ]),
                                        ]),
                                    ])

        , symbol("simpl_par") =>    or(vec![
                                        symref("simpl_par"),
                                        symref("parenth_expr")
                                    ])

        , symbol("simpl_par") =>    and(vec![
                                        lit("("),
                                        lit("_"),
                                        symref("expr"),
                                        lit("_"),
                                        lit(")")
                                    ])

        , symbol("simple") =>       symref("atom")

        , symbol("atom") =>         or(vec![
                                        symref("literal"),
                                        symref("match"),
                                        symref("dot"),
                                        symref("symbol")
                                    ])

        , symbol("literal") =>      or(vec![
                                        lit("\""),
                                        repeat(
                                            and(vec![
                                                not(lit("\"")),
                                                dot()
                                            ]),
                                            NRep(0), None
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
                                                dot()
                                            ]),
                                            NRep(1), None
                                        ),
                                        lit("]")
                                    ])

        , symbol("dot") =>          lit(".")

        //, symbol("symbol") =>       lit(".")

        , symbol("_") =>            repeat(
                                        and(vec![
                                            lit(" "),
                                            lit("\n"),
                                            symref("comment")
                                        ]),
                                        NRep(0), None
                                    )

        , symbol("comment") =>      or(vec![
                                        and(vec![
                                            lit("//"),
                                            and(vec![
                                                not(lit("\n")),
                                                dot()
                                            ]),
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