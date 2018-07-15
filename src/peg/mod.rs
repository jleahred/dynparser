#![warn(missing_docs)]
//! Module with functions to generate rules from PEG grammar
//!

use ast;
use parse;
use parser;
use std::result;
use Error;

#[cfg(test)]
mod test;

pub enum ErrPegAst {
    PegErr(Error),
    AstErr(String),
}
pub type Result<'a> = result::Result<parser::expression::SetOfRules<'a>, ErrPegAst>;

enum ExprOrRule<'a> {
    Expr(parser::expression::Expression<'a>),
    Rule(parser::expression::SetOfRules<'a>),
}

type ResultExprOrRule<'a> = result::Result<ExprOrRule<'a>, Error>;
type ResultExpr<'a> = result::Result<parser::expression::Expression<'a>, String>;

// -------------------------------------------------------------------------------------
//  A P I

/// Given a ```peg``` set of rules on an string, it will generate
/// the set of rules to use in the parser
pub fn rules_from_peg<'a>(peg: &str) -> Result {
    // let ast = parse(peg, &rules2parse_peg())?;
    // rules_from_ast(ast)
    let ast = parse(peg, &rules2parse_peg());
    println!("{:#?}", ast);

    // rules_from_ast(ast.map_err(|err| ErrPegAst::PegErr(err))?)
    use ast::Node::Val;
    let vnodes = vec![
        Val("\"".to_string()),
        Val("a".to_string()),
        Val("a".to_string()),
        Val("a".to_string()),
        Val("\"".to_string()),
    ];
    let ast = ast::Node::Rule(("literal".to_string(), vnodes));
    rules_from_ast(ast)
}

//  A P I
// -------------------------------------------------------------------------------------

fn rules_from_ast<'a>(ast: ast::Node) -> Result<'a> {
    // let eor = match ast {
    //     ast::Node::Rule((name, vr)) => expr_or_rule_from_ast_rule(name, vr),
    //     _ => (),
    // };
    // println!("{:#?}", ast);
    // let rules = rules!{ "main" => lit!("aa")};
    // println!("{:?}", rules);

    match ast {
        ast::Node::Rule((_, vnodes)) => {
            let expr = atom_literal_from_nodes(&vnodes).map_err(|e| ErrPegAst::AstErr(e))?;
            Ok(rules!("main" => expr))
        }
        _ => Err(ErrPegAst::AstErr("ERROR TESTING AST".to_string())),
    }
}

fn atom_literal_from_nodes<'a>(nodes: &'a [ast::Node]) -> ResultExpr<'a> {
    //  literal =   "\""  (!"\"" .)*  "\""

    let get_lit = |sq, val, eq| match (sq, val, eq) {
        ("\"", v, "\"") => Ok(lit!(v)),
        _ => Err(format!(
            "Error extracting literal from '{}', '{}', '{}'\nExpetected string between quotes",
            sq, val, eq
        )),
    };

    match nodes[..] {
        [ast::Node::Val(ref sq), ast::Node::Val(ref val), ast::Node::Val(ref eq)] => {
            get_lit(&sq, &val, &eq)
        }
        _ => Err("Error extracting literal expected 3 child val nodes".to_string()),
    }
}

// fn atom_match_from_nodes<'a>(nodes: &'a [ast::Node]) -> ResultExpr<'a> {
//     //  match   =   "["  ((.  "-"  .)  /  (.))+   "]"

//     // ex
//     // Val("["),
//     // Val("a"),
//     // Val("b"),
//     // Val("A"),
//     // Val("-"),
//     // Val("Z"),
//     // Val("]")

// }

fn atom_dot_from_nodes<'a>(nodes: &'a [ast::Node]) -> ResultExpr<'a> {
    //  dot     =   "."

    let get_dot = |val| match val {
        "." => Ok(dot!()),
        _ => Err(format!(
            "Error extracting dot from '{}'\nExpetected '.'",
            val
        )),
    };

    match nodes[..] {
        [ast::Node::Val(ref val)] => get_dot(&val),
        _ => Err("Error extracting literal expected 1 child val nodes".to_string()),
    }
}

fn atom_ref_rule_from_nodes<'a>(nodes: &'a [ast::Node]) -> ResultExpr<'a> {
    //  symbol  =   [a-zA-Z0-9_]+

    fn concat_val_lit_nodes<'a>(
        nodes: &'a [ast::Node],
        acc: String,
    ) -> result::Result<String, String> {
        let concat_node = |n: &_, mut acc: String| match n {
            ast::Node::Val(ref v) => Ok(format!("{}{}", acc, v)),
            _ => Err("Expected ast::Node::Val(String)"),
        };

        let r_name = match nodes.len() {
            0 => acc,
            _ => concat_val_lit_nodes(&nodes[1..], concat_node(&nodes[0], acc)?)?,
        };
        Ok(r_name)
    };

    let r_name = concat_val_lit_nodes(nodes, "".to_string())?;
    Ok(rule!(r_name))
}

//  ------------------------------------------------------------------------
//  ------------------------------------------------------------------------
//
//  this is the first version of code to parse the peg grammar
//  it was, obviously written by hand
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
                                rule!("parenth")
                            ),

        "parenth"       =>  and!(
                                lit!("("),
                                rule!("_"),
                                rule!("expr"),
                                rule!("_"),
                                lit!(")")
                            ),

        "atom"          =>  or!(
                                rule!("literal"),
                                rule!("match"),
                                rule!("dot"),
                                rule!("symbol")
                            ),

        "literal"       =>  and!(
                                lit!(r#"""#),
                                rep!(
                                    and!(
                                        not!(
                                            lit!(r#"""#)
                                        ),
                                        dot!()
                                    )
                                    , 0
                                ),
                                lit!(r#"""#)
                            ),

        "match"         =>  and!(
                                lit!("["),
                                or!(
                                    and!(dot!(), lit!("-"), dot!()),
                                    rep!(
                                        and!(not!(lit!("]")), dot!())
                                        ,1
                                    )
                                ),
                                lit!("]")
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
