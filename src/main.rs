
extern crate dynparser;
use dynparser::grammar::grammar;
use dynparser::ast;


use dynparser::{symbol, text2parse, parse};



fn main() {
    let parsed = parse(&text2parse(r#"
            grammar         =   rule+

            rule            =   symbol  _  "="  _   expr  (_eol / eof)  _
    "#),
                       &symbol("grammar"),
                       &grammar());
    // let parsed = parse(&text2parse(r#"
    //         grammar         =   rule+

    //         rule            =   symbol  _  "="  _   expr  (_eol / eof)  _

    //         expr            =   or_expr

    //         or_expr         =   and_expr    (_ "/"  _  or_expr)*

    //         and_expr        =   compl_expr  (  " "  _  and_expr)*

    //         compl_expr      =   simpl_par ("*" / "+")?
    //                         /   "!" simpl_par

    //         simpl_par       =   (simple / parenth_expr)


    //         parenth_expr    =   "("  _  expr  _  ")"
    //         simple          =   atom



    //         atom    =   literal
    //                 /   match
    //                 /   dot
    //                 /   symbol

    //         literal =   "\u{34}"  (!"\u{34}" .)*  "\u{34}"
    //         match   =   "["  ( (.  "-"  .)  /  (!"]") )+   "]"
    //         dot     =   "."
    //         symbol  =   [a-zA-Z0-9_]+


    //         _   =  (" "
    //             /   "\n"
    //             /   comment)*

    //         _eol = " "*  "\n"
    //             / comment

    //         comment =  "//" (!"/n" .)* "/n"
    //                 /  "/*" (!"*/" .)* "*/"
    //     "#),
    //                    &symbol("grammar"),
    //                    &grammar());

    let check2prune = |kind: &ast::K, val: &str| {
        let prune_kind = match kind {
            &ast::K::EAnd => true,
            &ast::K::ERepeat => true,
            _ => false,
        };
        let prune_val = match val {
            _ => false,
        };
        let prune_comb = match (kind, val) {
            (&ast::K::ASymbref, "expr") => true,
            (&ast::K::ASymbref, "or_expr") => true,
            (&ast::K::ASymbref, "and_expr") => true,
            (&ast::K::ASymbref, "compl_expr") => true,
            (&ast::K::ASymbref, "simpl_par") => true,
            (&ast::K::ASymbref, "simple") => true,
            (&ast::K::ASymbref, "atom") => true,
            // (&ast::K::ASymbref, "_eol") => true,
            // (&ast::K::ASymbref, "_") => true,
            (&ast::K::ALit, " ") => true,
            (&ast::K::ALit, "\n") => true,
            _ => false,
        };
        prune_kind || prune_val || prune_comb

    };

    // let parsed = parse(&text2parse(r#"h=a (b"#), &symbol("grammar"), &grammar());

    let parsed = match parsed {
        Err(err) => {
            println!("error... {} ___________", err);
            panic!("Error parsing!!!")
        }
        Ok(res) => res,
    };
    let parsed = parsed.get_pruned(&check2prune);

    println!("ast... {:?} ___________", parsed);

    println!("parsed ... \n{}", iterate_nodes(&parsed, 0));
}

fn iterate_nodes(node: &ast::Node, depth: usize) -> String {
    let indent = " ".repeat(depth * 4);
    let ref mut result = format!("{}{}k: {:?}, v: {:?}\n", depth, indent, node.kind, node.val);
    // let ref mut result = format!("{}k: {:?}, v: {:?}\n", indent, node.kind, node.val);
    for n in node.nodes.iter() {
        *result += &iterate_nodes(n, depth + 1);
    }
    result.to_owned()
}