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
    Peg((String, Option<Box<Error>>)),
    Parser(parser::Error),
    Ast(ast::Error),
}

fn error_peg_s(s: &str) -> Error {
    Error::Peg((s.to_string(), None))
}

impl Error {
    fn push(self, desc: &str) -> Self {
        Error::Peg((desc.to_string(), Some(Box::new(self))))
    }
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
            Error::Peg((s, None)) => write!(f, "{}", s),
            Error::Peg((s, Some(b))) => write!(f, "{} > {}", s, b),
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
// type ResultExpr<'a> = result::Result<parser::expression::Expression<'a>, Error>;

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

    let vast = vec![ast];
    let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("main", &vast)?;
    check_empty_nodes(nodes)?;
    let (rules, _sub_nodes) = consume_grammar(sub_nodes)?;

    Ok(rules)
}

macro_rules! push_err {
    ($descr:expr, $e:expr) => {{
        let l = || $e;
        l().map_err(|e: Error| e.push($descr))
    }};
}

fn consume_grammar<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::SetOfRules<'a>, &[ast::Node]), Error> {
    //  grammar         =   rule+

    push_err!("consuming grammar", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("grammar", nodes)?;
        check_empty_nodes(nodes)?;
        let (name, expr, nodes) = consume_peg_rule(sub_nodes)?;

        let rules = rules!().add(name, expr);

        Ok((rules, nodes))
    })
}

fn consume_peg_rule<'a>(
    nodes: &[ast::Node],
) -> result::Result<(&str, parser::expression::Expression<'a>, &[ast::Node]), Error> {
    //  rule            =   symbol  _  "="  _   expr  (_ / eof)

    push_err!("consuming rule", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("rule", nodes)?;
        check_empty_nodes(nodes)?;
        let (symbol_name, sub_nodes) = consume_symbol(sub_nodes)?;
        let sub_nodes = consume_if_val("=", sub_nodes)?;
        let (expr, sub_nodes) = consume_peg_expr(sub_nodes)?;
        check_empty_nodes(sub_nodes)?;

        Ok((symbol_name, expr, nodes))
    })
}

fn consume_symbol<'a>(nodes: &[ast::Node]) -> result::Result<(&str, &[ast::Node]), Error> {
    //  symbol          =   [a-zA-Z0-9_']+

    push_err!("consuming symbol", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("symbol", nodes)?;
        let value = ast::get_nodes_unique_val(sub_nodes)?;
        Ok((value, nodes))
    })
}

fn consume_peg_expr<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::Expression<'a>, &[ast::Node]), Error> {
    //  expr            =   or

    push_err!("consuming expr", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("expr", nodes)?;
        check_empty_nodes(nodes)?;
        let (expr, sub_nodes) = consume_or(sub_nodes)?;
        check_empty_nodes(sub_nodes)?;

        Ok((expr, nodes))
    })
}

fn consume_or<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::Expression<'a>, &[ast::Node]), Error> {
    //  or              =   and         ( _ "/"  _  or)*

    push_err!("consuming or", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("or", nodes)?;
        check_empty_nodes(nodes)?;
        let (expr, sub_nodes) = consume_and(sub_nodes)?;
        check_empty_nodes(sub_nodes)?;

        Ok((expr, nodes))
    })
}

fn consume_and<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::Expression<'a>, &[ast::Node]), Error> {
    //  and             =   rep_or_neg  (   " "  _  and)*

    push_err!("consuming and", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("and", nodes)?;
        check_empty_nodes(nodes)?;
        let (expr, sub_nodes) = consume_rep_or_neg(sub_nodes)?;
        check_empty_nodes(sub_nodes)?;

        Ok((expr, nodes))
    })
}

fn consume_rep_or_neg<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::Expression<'a>, &[ast::Node]), Error> {
    // rep_or_neg      =   atom_or_par ("*" / "+" / "?")?
    //                 /   "!" atom_or_par

    let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("rep_or_neg", nodes)?;
    check_empty_nodes(nodes)?;
    let (expr, sub_nodes) = consume_atom_or_par(sub_nodes)?;
    check_empty_nodes(sub_nodes)?;

    Ok((expr, nodes))
}

fn consume_atom_or_par<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::Expression<'a>, &[ast::Node]), Error> {
    // atom_or_par     =   (atom / parenth)

    let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("atom_or_par", nodes)?;
    check_empty_nodes(nodes)?;
    let (expr, sub_nodes) = consume_atom(sub_nodes)?;
    check_empty_nodes(sub_nodes)?;

    Ok((expr, nodes))
}

fn consume_atom<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::Expression<'a>, &[ast::Node]), Error> {
    // atom            =   literal
    //                 /   match
    //                 /   dot
    //                 /   symbol

    push_err!("consuming atom", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("atom", nodes)?;
        check_empty_nodes(nodes)?;
        let (expr, sub_nodes) = consume_literal(sub_nodes)?;
        check_empty_nodes(sub_nodes)?;

        Ok((expr, nodes))
    })
}

fn consume_literal<'a>(
    nodes: &[ast::Node],
) -> result::Result<(parser::expression::Expression<'a>, &[ast::Node]), Error> {
    // literal         =   _" till_quote _"

    push_err!("consuming literal", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("literal", nodes)?;
        check_empty_nodes(nodes)?;
        let sub_nodes = consume_quote(sub_nodes)?;
        let (val, sub_nodes) = ast::consume_val(sub_nodes)?;
        let sub_nodes = consume_quote(sub_nodes)?;
        check_empty_nodes(sub_nodes)?;

        Ok((lit!(val), nodes))
    })
}

fn consume_quote<'a>(nodes: &[ast::Node]) -> result::Result<&[ast::Node], Error> {
    // _"              =   "\u{34}"

    push_err!("consuming quote", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is(r#"_""#, nodes)?;
        let sub_nodes = consume_if_val(r#"""#, sub_nodes)?;
        check_empty_nodes(sub_nodes)?;

        Ok(nodes)
    })
}

fn consume_if_val<'a>(v: &str, nodes: &'a [ast::Node]) -> result::Result<(&'a [ast::Node]), Error> {
    let (node, nodes) = ast::split_first_nodes(nodes)?;

    let nv = ast::get_node_val(node)?;
    match nv == v {
        true => Ok(nodes),
        false => Err(error_peg_s(&format!("expected {} readed {}", v, nv))),
    }
}

fn consume_node_get_subnodes_if_rule_name_is<'a>(
    name: &str,
    nodes: &'a [ast::Node],
) -> result::Result<(&'a [ast::Node], &'a [ast::Node]), Error> {
    let (node, nodes) = ast::split_first_nodes(nodes)?;
    match node {
        ast::Node::Rule((n, sub_nodes)) => if n == name {
            Ok((nodes, sub_nodes))
        } else {
            Err(error_peg_s(&format!("expected expr node, received {}", n)))
        },
        _ => Err(error_peg_s("expected rule node, received {:?}")),
    }
}

fn check_empty_nodes(nodes: &[ast::Node]) -> result::Result<(), Error> {
    match nodes.is_empty() {
        true => Ok(()),
        false => Err(error_peg_s("not consumed full nodes")),
    }
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
                                rule!(r#"_""#),
                                rep!(
                                    and!(
                                        not!(
                                            rule!(r#"_""#)
                                        ),
                                        dot!()
                                    )
                                , 0
                            ),
                                rule!(r#"_""#)
                            ),

        r#"_""#         =>  lit!(r#"""#),

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
