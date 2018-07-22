#![warn(missing_docs)]
//! Module with functions to generate rules from PEG grammar
//!

use ast;
use parse;
use parser;
use std::{self, result};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum Error {
    Peg(String),
    Parser(parser::Error),
    Ast(ast::Error),
}

impl From<parser::Error> for Error {
    fn from(e: parser::Error) -> Self {
        Error::Parser(e)
    }
}

impl From<ast::Error> for Error {
    fn from(e: ast::Error) -> Self {
        Error::Ast(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Peg(s) => write!(f, "{}", s),
            Error::Parser(p) => write!(f, "Parser({:?})", p),
            Error::Ast(a) => write!(f, "AST({:?})", a),
        }
    }
}

pub type Result<'a> = result::Result<parser::expression::SetOfRules<'a>, Error>;

// enum ExprOrRule<'a> {
//     Expr(parser::expression::Expression<'a>),
//     Rule(parser::expression::SetOfRules<'a>),
// }

// type ResultExprOrRule<'a> = result::Result<ExprOrRule<'a>, Error>;
type ResultExpr<'a> = result::Result<parser::expression::Expression<'a>, Error>;

// -------------------------------------------------------------------------------------
//  A P I

/// Given a ```peg``` set of rules on an string, it will generate
/// the set of rules to use in the parser
pub fn rules_from_peg(peg: &str) -> Result {
    let ast = parse(peg, &rules2parse_peg())?;

    // println!("{:#?}", ast);
    rules_from_ast(&ast)
}

//  A P I
// -------------------------------------------------------------------------------------

fn rules_from_ast<'a>(ast: &ast::Node) -> Result<'a> {
    let ast = ast.compact().prune(&vec!["_"]);
    println!(":::::::  {:?}", ast);

    process_node(&ast)
    // let result = process_node(&ast)?;

    // match result {
    //     ExprOrRule::Expr(expr) => Ok(rules!("maina" => expr)),
    //     ExprOrRule::Rule(rule) => Ok(rule),
    // }
}

fn process_node<'a>(node: &ast::Node) -> Result<'a> {
    match node {
        ast::Node::Rule((rname, nodes)) => process_peg_rule(&rname, &nodes),
        _ => Err(Error::Peg("ERROR TESTING AST".to_string())),
    }
}

fn process_peg_rule<'a, 'b>(rname: &str, nodes: &'a [ast::Node]) -> Result<'b> {
    match rname {
        "main" => passthrow(&nodes),
        "grammar" => passthrow(&nodes),
        "rule" => process_rule(&nodes),
        "expr" => passthrow(&nodes),
        "or" => passthrow(&nodes),
        "and" => passthrow(&nodes),
        "rep_or_neg" => passthrow(&nodes),
        "atom_or_par" => passthrow(&nodes),
        // "atom" => Ok(ExprOrRule::Expr(process_atom(&nodes)?)),
        _ => Err(Error::Peg(format!("unknown peg rule {}", rname))),
    }.or_else(|e| Err(Error::Peg(format!("processing {} > {}", rname, e))))
}

fn split_first_nodes(nodes: &[ast::Node]) -> result::Result<(&ast::Node, &[ast::Node]), Error> {
    nodes.split_first().ok_or(Error::Peg(
        "trying get first element from nodes on empty slice".to_string(),
    ))
}

fn consume_symbol_value(nodes: &[ast::Node]) -> result::Result<(&str, &[ast::Node]), Error> {
    let (node, nodes) = split_first_nodes(nodes)?;

    let (nname, sub_nodes) = ast::get_nodename_and_nodes(node)?;
    match nname {
        "symbol" => Ok((ast::get_nodes_unique_val(sub_nodes)?, nodes)),
        _ => Err(Error::Peg("expected symbol".to_string())),
    }
}

fn consume_and_check_val<'a>(
    v: &str,
    nodes: &'a [ast::Node],
) -> result::Result<(&'a [ast::Node]), Error> {
    let (node, nodes) = split_first_nodes(nodes)?;

    let nv = ast::get_node_val(node)?;
    match nv == v {
        true => Ok(nodes),
        false => Err(Error::Peg(format!("expected {} readed {}", v, nv))),
    }
}

fn get_sub_nodes_if_rule_name_is<'a>(
    name: &str,
    node: &'a ast::Node,
) -> result::Result<(&'a [ast::Node]), Error> {
    match node {
        ast::Node::Rule((n, sub_nodes)) => if n == name {
            Ok(sub_nodes)
        } else {
            Err(Error::Peg(format!("expected expr node, received {}", n)))
        },
        _ => Err(Error::Peg("expected rule node, received {:?}".to_string())),
    }
}

fn consume_and<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::Expression<'a>, &[ast::Node]), Error> {
    //  and             =   rep_or_neg  (   " "  _  and)*

    let (node, nodes) = split_first_nodes(nodes)?;
    let sub_nodes = get_sub_nodes_if_rule_name_is("and", node)?;

    Ok((lit!("hello"), nodes))
}

fn consume_or<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::Expression<'a>, &[ast::Node]), Error> {
    //  or              =   and         ( _ "/"  _  or)*

    let (node, nodes) = split_first_nodes(nodes)?;
    let sub_nodes = get_sub_nodes_if_rule_name_is("or", node)?;

    let (expr1, sub_nodes) = consume_and(sub_nodes)?;
    let expr2 = if sub_nodes.len() > 0 {
        let sub_nodes = consume_and_check_val("/", nodes)?;
        let (expr2, sub_nodes) = consume_or(sub_nodes)?;
        Some(expr2)
    } else {
        None
    };
    check_empty_nodes(sub_nodes)?;

    match expr2 {
        Some(e2) => Ok((or!(expr1, e2), nodes)),
        _ => Ok((or!(expr1), nodes)),
    }
}

fn consume_expr<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::Expression<'a>, &[ast::Node]), Error> {
    // expr            =   or

    let (node, nodes) = split_first_nodes(nodes)?;
    let sub_nodes = get_sub_nodes_if_rule_name_is("expr", node)?;

    let (expr, sub_nodes) = consume_or(sub_nodes)?;
    check_empty_nodes(sub_nodes)?;

    Ok((expr, nodes))
}

fn check_empty_nodes(nodes: &[ast::Node]) -> result::Result<(), Error> {
    match nodes.is_empty() {
        true => Ok(()),
        false => Err(Error::Peg(format!("not consumed full nodes"))),
    }
}

fn process_rule<'a>(nodes: &[ast::Node]) -> Result<'a> {
    //  rule            =   symbol  _  "="  _   expr  (_ / eof)

    let (symbol_name, nodes) = consume_symbol_value(nodes)?;
    let nodes = consume_and_check_val("=", nodes)?;
    let (expr, nodes) = consume_expr(nodes)?;
    check_empty_nodes(nodes)?;
    Ok(rules!(symbol_name => expr))
}

fn passthrow<'a>(nodes: &[ast::Node]) -> Result<'a> {
    match nodes {
        [node] => process_node(node),
        _ => Err(Error::Peg(format!(
            "passthrow can have only one child node {:?}",
            nodes
        ))),
    }
}

// fn process_atom<'a, 'b>(nodes: &'a [ast::Node]) -> ResultExpr<'b> {
//     let get_atom_child_node = |nodes: &'a [ast::Node]| match nodes {
//         &[ref node] => Ok(node),
//         _ => Err(Error::Peg(format!(
//             "an atom can have only one child {:?}",
//             &nodes
//         ))),
//     };

//     let get_atom_rule_info = |&node| match node {
//         &ast::Node::Rule((ref name, ref nodes)) => Ok((name, nodes)),
//         _ => Err(Error::Peg(format!(
//             "incorrect atom info in ast {:?}",
//             &nodes
//         ))),
//     };

//     let atom_node = get_atom_child_node(nodes)?;
//     let (rname, nodes) = get_atom_rule_info(&atom_node)?;

//     match (&rname as &str, nodes) {
//         ("literal", nodes) => atom_literal_from_nodes(&nodes),
//         // ("symbol", nodes) => atom_symbol_from_nodes(&nodes),
//         (at, _) => Err(Error::Peg(format!("not registered atom type {}", at))),
//     }
// }

// fn atom_literal_from_nodes<'a, 'b>(nodes: &'a [ast::Node]) -> ResultExpr<'b> {
//     //  literal =   "\""  (!"\"" .)*  "\""

//     let check_quote = |n: &ast::Node| match n {
//         ast::Node::Val(v) => {
//             if v == "\"" {
//                 Ok(())
//             } else {
//                 Err(Error::Peg(format!(
//                     "Expected quote arround literal string, got {}",
//                     v
//                 )))
//             }
//         }
//         _ => Err(Error::Peg(format!(
//             "Expected ast::Node::Val arround literal string, got {:?}",
//             n
//         ))),
//     };

//     let remove_quotes_arround = |nodes: &'a [ast::Node]| -> result::Result<&[ast::Node], Error> {
//         let error_inv_nodes_size = || {
//             Error::Peg(format!(
//                 "Invalid ast for literal. Minimum nodes size 3 '{:?}''",
//                 &nodes
//             ))
//         };
//         let (f, nodes) = nodes.split_first().ok_or(error_inv_nodes_size())?;
//         let (l, nodes) = nodes.split_last().ok_or(error_inv_nodes_size())?;
//         let (_, _) = (check_quote(f)?, check_quote(l)?);
//         Ok(nodes)
//     };

//     let concat_str_nodes2string = |nodes: &[ast::Node]| {
//         nodes
//             .iter()
//             .try_fold("".to_string(), |acc, n: &ast::Node| match n {
//                 ast::Node::Val(v) => Ok(format!("{}{}", acc, v)),
//                 _ => Err(Error::Peg(format!("Expected ast::Node::Val {:?}", &n))),
//             })
//     };

//     let removed_quotes = remove_quotes_arround(nodes)?;
//     let slit = concat_str_nodes2string(removed_quotes)?;

//     Ok(lit!(slit))
// }

// fn atom_symbol_from_nodes(nodes: &[ast::Node]) -> result::Result<String, String> {
//     //  symbol          =   [a-zA-Z0-9_']+

//     Ok("symbol".to_string())
// }

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

fn atom_dot_from_nodes(nodes: &[ast::Node]) -> ResultExpr {
    //  dot     =   "."

    let get_dot = |val| match val {
        "." => Ok(dot!()),
        _ => Err(Error::Peg(format!(
            "Error extracting dot from '{}'\nExpetected '.'",
            val
        ))),
    };

    match nodes[..] {
        [ast::Node::Val(ref val)] => get_dot(&val),
        _ => Err(Error::Peg(
            "Error extracting literal expected 1 child val nodes".to_string(),
        )),
    }
}

// fn atom_ref_rule_from_nodes(nodes: & [ast::Node]) -> ResultExpr {
//     //  symbol  =   [a-zA-Z0-9_]+

//     fn concat_val_lit_nodes<'a>(
//         nodes: &'a [ast::Node],
//         acc: String,
//     ) -> result::Result<String, Error> {
//         let concat_node = |n: &_, acc: String| match n {
//             ast::Node::Val(ref v) => Ok(format!("{}{}", acc, v)),
//             _ => Err(Error::Peg("Expected ast::Node::Val(String)".to_string())),
//         };

//         let r_name = match nodes.len() {
//             0 => acc,
//             _ => concat_val_lit_nodes(&nodes[1..], concat_node(&nodes[0], acc)?)?,
//         };
//         Ok(r_name)
//     };

//     let r_name = concat_val_lit_nodes(nodes, "".to_string())?;
//     Ok(rule!(r_name))
// }

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
