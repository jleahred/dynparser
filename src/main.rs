
extern crate dynparser;
use dynparser::grammar::grammar;


use dynparser::{symbol, text2parse, parse};


fn main() {
    let parsed = parse(&text2parse(r#"
grammar         =   rule+

rule            =   symbol  _  "="  _   expr  (_eol / eof)  _

expr            =   or_expr

or_expr         =   and_expr    (_ "/"  _  or_expr)*

and_expr        =   compl_expr  (  " "  _  and_expr)*

compl_expr      =   simpl_par ("*" / "+")?
                /   "!" simpl_par

simpl_par       =   (simple / parenth_expr)


parenth_expr    =   "("  _  expr  _  ")"
simple          =   atom



atom    =   literal
        /   match
        /   dot
        /   symbol

//literal =   "\u{34}"  (!"\u{34}" .)*  "\u{34}"
// match   =   "["  ( (.  "-"  .)  /  (!"]") )+   "]"
// dot     =   "."
// symbol  =   [a-zA-Z0-9_]+


// _   =  (" " 
//     /   "\n"
//     /   comment)*

// _eol = " "*  "\n"
//      / comment

comment =  "//" (!"/n" .)* "/n"
        /  "/*" (!"*/" .)* "*/"
"#),
                       &symbol("grammar"),
                       &grammar());

    match parsed {
        Err(err) => println!("error... {} ___________", err),
        Ok(res) => println!("Ok... {:?} ___________", res),
    };
}
