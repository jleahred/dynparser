#![warn(missing_docs)]
//! Module with functions to generate rules from PEG grammar
//!

pub mod gcode;
mod rules;

use ast;
use idata::IVec;
use parse;
use parser::{
    self,
    expression::{self, Expression},
};
use std::{self, result};

#[cfg(test)]
mod test;

#[derive(Debug)]
/// Most of peg functions will return a result with this type
/// on Error side
pub enum Error {
    /// When error has been on `peg` side
    /// we will receive a description and
    /// optionally, a link to a stacked error
    /// Then, we can have a errors stack of ilimited size
    Peg((String, Option<Box<Error>>)),
    /// When error is on parser side
    Parser(parser::Error),
    /// When error is on ast side
    Ast(ast::Error),
}

fn error_peg_s(s: &str) -> Error {
    Error::Peg((s.to_string(), None))
}

impl Error {
    fn ipush(self, desc: &str) -> Self {
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

/// Most of functions on peg module, will return a set of rules
/// or an error
pub type Result = result::Result<expression::SetOfRules, Error>;

// -------------------------------------------------------------------------------------
//  A P I

/// Given a ```peg``` set of rules on an string, it will generate
/// the set of rules to use in the parser
///
/// Next, is a full example showing the error messages, if so
/// ```
/// extern crate dynparser;
/// use dynparser::{parse, rules_from_peg};
///
/// fn main() {
///     let rules = rules_from_peg(
///         r#"
/// main    =   "hello"   " "   "world"
///         "#,
///     ).map_err(|e| {
///         println!("{}", e);
///         panic!("FAIL");
///     })
///         .unwrap();
///
///     println!("{:#?}", rules);
///
///     let result = parse("hello world", &rules);
///
///     assert!(result.is_ok());
///
///     match result {
///         Ok(ast) => println!("{:#?}", ast),
///         Err(e) => println!("Error: {:?}", e),
///     };
/// }
/// ```
///
/// Next is an example with some ```and``` ```literals```
/// ```
///extern crate dynparser;
///use dynparser::{parse, rules_from_peg};
///
///    let rules = rules_from_peg(
///        r#"
///
///main    =   "hello"   " "   "world"
///
///        "#,
///    ).unwrap();
///
///     assert!(parse("hello world", &rules).is_ok());
/// ```

pub fn rules_from_peg2(peg: &str) -> Result {
    let ast = parse(peg, &rules::parse_peg())?;
    let ast = ast.compact().prune(&["_", "_eol"]);
    rules_from_ast(&ast)
}

pub fn rules_from_peg(peg: &str) -> Result {
    let ast = parse(peg, &rules::parse_peg())?;
    let nodes = ast.compact().prune(&["_", "_1", "_eol"]).flattern();
    println!("{:#?}", nodes);

    rules_from_flattern_ast(&nodes)
}

//  A P I
// -------------------------------------------------------------------------------------

fn rules_from_flattern_ast(nodes: &[ast::FlatNode]) -> Result {
    let (rules, nodes) = consume_main(&nodes)?;
    if !nodes.is_empty() {
        Err(error_peg_s("expected empty nodes after processing main"))
    } else {
        Ok(rules)
    }
}

macro_rules! push_err {
    ($descr:expr, $e:expr) => {{
        let l = || $e;
        l().map_err(|e: Error| e.ipush($descr))
    }};
}

fn consuming_rule<'a, F, R>(
    rule_name: &str,
    nodes: &'a [ast::FlatNode],
    f: F,
) -> result::Result<(R, &'a [ast::FlatNode]), Error>
//result::Result<(expression::SetOfRules, &'a [ast::FlatNode]), Error>
where
    F: FnOnce(&'a [ast::FlatNode]) -> result::Result<(R, &'a [ast::FlatNode]), Error>, //result::Result<(expression::SetOfRules, &'a [ast::FlatNode]), Error>
                                                                                       // R: std::ops::Try,
{
    push_err!(&format!("consuming {}", rule_name), {
        let nodes = ast::consume_flat_node_start_rule_name(rule_name, &nodes)?;
        let (result, nodes) = f(&nodes)?;
        let nodes = ast::consume_flat_node_end_rule_name(rule_name, &nodes)?;
        Ok((result, nodes))
    })
}

fn consume_main(
    nodes: &[ast::FlatNode],
) -> result::Result<(expression::SetOfRules, &[ast::FlatNode]), Error> {
    // main            =   grammar

    consuming_rule("main", nodes, |nodes| consume_grammar_f(&nodes))
    // push_err!("consuming main", {
    //     let nodes = ast::consume_flat_node_start_rule_name("main", &nodes)?;
    //     let (rules, nodes) = consume_grammar_f(&nodes)?;
    //     let nodes = ast::consume_flat_node_end_rule_name("main", &nodes)?;
    //     Ok((rules, nodes))
    // })
}

fn consume_grammar_f(
    nodes: &[ast::FlatNode],
) -> result::Result<(expression::SetOfRules, &[ast::FlatNode]), Error> {
    // grammar         =   rule+

    fn rec_consume_rules(
        rules: expression::SetOfRules,
        nodes: &[ast::FlatNode],
    ) -> result::Result<(expression::SetOfRules, &[ast::FlatNode]), Error> {
        match ast::peek_first_flat_node(nodes)? {
            ast::FlatNode::BeginRule(_) => {
                let ((name, expr), nodes) = consume_rule_f(nodes)?;
                let rules = rules.add(name, expr);
                rec_consume_rules(rules, nodes)
            }
            _ => Ok((rules, nodes)),
        }
    }

    consuming_rule("grammar", nodes, |nodes| {
        rec_consume_rules(rules!(), &nodes)
    })

    // push_err!("consuming grammar", {
    //     let nodes = ast::consume_flat_node_start_rule_name("grammar", &nodes)?;
    //     let (rules, nodes) = rec_consume_rules(rules!(), nodes)?;
    //     let nodes = ast::consume_flat_node_end_rule_name("grammar", &nodes)?;
    //     Ok((rules, nodes))
    // })
}

fn consume_rule_f(
    nodes: &[ast::FlatNode],
) -> result::Result<((&str, expression::Expression), &[ast::FlatNode]), Error> {
    // rule            =   _  symbol  _  "="  _  expr  _eol _

    consuming_rule("rule", nodes, |nodes| {
        let (symbol_name, nodes) = consume_symbol_f(nodes)?;
        // push_err!(&format!("r:({})", symbol_name), {
        let nodes = ast::flat_consume_this_value("=", nodes)?;
        let (expr, nodes) = consume_peg_expr_f(nodes)?;

        Ok(((symbol_name, expr), nodes))
        // })
    })
    // push_err!("consuming rule", {
    //     let nodes = ast::consume_flat_node_start_rule_name("rule", &nodes)?;

    //     let (symbol_name, nodes) = consume_symbol_f(nodes)?;

    //     push_err!(&format!("r:({})", symbol_name), {
    //         let nodes = ast::flat_consume_this_value("=", nodes)?;
    //         let (expr, nodes) = consume_peg_expr_f(nodes)?;
    //         let nodes = ast::consume_flat_node_end_rule_name("rule", &nodes)?;

    //         Ok((symbol_name, expr, nodes))
    //     })
    // })
}

fn consume_symbol_f(nodes: &[ast::FlatNode]) -> result::Result<(&str, &[ast::FlatNode]), Error> {
    // symbol          =   [_'a-zA-Z0-9] [_'"a-zA-Z0-9]*

    consuming_rule("symbol", nodes, |nodes| {
        let (val, nodes) = ast::flat_consume_val(nodes)?;
        Ok((val, nodes))
    })
    // push_err!("consuming symbol", {
    //     let nodes = ast::consume_flat_node_start_rule_name("symbol", nodes)?;
    //     let value = ast::get_flat_nodes_unique_val(nodes)?;
    //     let nodes = ast::consume_flat_node_end_rule_name("symbol", nodes)?;
    //     Ok((value, nodes))
    // })
}

fn consume_peg_expr_f(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    //  expr            =   or

    consuming_rule("expr", nodes, |nodes| consume_or_f(nodes))
    // Err(error_peg_s("Not implemented"))
    // push_err!("consuming expr", {
    //     let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("expr", nodes)?;
    //     let (expr, sub_nodes) = consume_or(sub_nodes)?;
    //     ast::check_empty_nodes(sub_nodes)?;

    //     Ok((expr, nodes))
    // })
}

fn consume_or_f(nodes: &[ast::FlatNode]) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    //  or              =   and         ( _ "/"  _  or)*

    fn rec_consume_or(
        eov: ExprOrVecExpr,
        nodes: &[ast::FlatNode],
    ) -> result::Result<(ExprOrVecExpr, &[ast::FlatNode]), Error> {
        consuming_rule("or", nodes, move |nodes| {
            let (expr, nodes) = consume_and_f(nodes)?;
            let eov = eov.ipush(expr);

            let consume_next_or = |eov, nodes| {
                let (exprs, nodes) = match ast::flat_consume_this_value("/", nodes) {
                    Ok(nodes) => rec_consume_or(eov, &nodes)?,
                    _ => (eov, nodes),
                };
                Ok((exprs, nodes))
            };
            match nodes.len() {
                0 => Ok((eov, nodes)),
                _ => consume_next_or(eov, nodes),
            }
        })
    };

    let build_or_expr = |vexpr| Expression::Or(expression::MultiExpr(vexpr));
    //-----

    push_err!("or:", {
        let (eov, nodes) = rec_consume_or(ExprOrVecExpr::None, nodes)?;

        match eov {
            ExprOrVecExpr::None => Err(error_peg_s("logic error, empty or parsing???")),
            ExprOrVecExpr::Expr(e) => Ok((e, nodes)),
            ExprOrVecExpr::VExpr(v) => Ok((build_or_expr(v), nodes)),
        }
    })
}

fn consume_and_f(nodes: &[ast::FlatNode]) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // and             =   rep_or_neg  ( _1 _ !(symbol _ "=") and )*

    // Err(error_peg_s("Not implemented"))

    fn rec_consume_and(
        eov: ExprOrVecExpr,
        nodes: &[ast::FlatNode],
    ) -> result::Result<(ExprOrVecExpr, &[ast::FlatNode]), Error> {
        consuming_rule("and", nodes, move |nodes| {
            let (expr, nodes) = consume_rep_or_neg_f(nodes)?;
            let eov = eov.ipush(expr);
            let next_node = ast::peek_first_flat_node(nodes)?;

            match (next_node, ast::flat_get_nodename(next_node)) {
                (ast::FlatNode::BeginRule(_), Ok("and")) => rec_consume_and(eov, nodes),
                _ => Ok((eov, nodes)),
            }
        })
    }

    let build_and_expr = |vexpr| Expression::And(expression::MultiExpr(vexpr));

    let (eov, nodes) = rec_consume_and(ExprOrVecExpr::None, nodes)?;
    match eov {
        ExprOrVecExpr::None => Err(error_peg_s("logic error, empty or parsing???")),
        ExprOrVecExpr::Expr(e) => Ok((e, nodes)),
        ExprOrVecExpr::VExpr(v) => Ok((build_and_expr(v), nodes)),
    }

    // fn rec_consume_and(
    //     eov: ExprOrVecExpr,
    //     nodes: &[ast::FlatNode],
    // ) -> result::Result<(ExprOrVecExpr, &[ast::FlatNode]), Error> {
    //     println!("____ {:?}", nodes);
    //     consuming_rule("and", nodes, move |nodes| {
    //         let (expr, nodes) = consume_rep_or_neg_f(nodes)?;
    //         let consume_next_and = |eov, nodes| {
    //             let (exprs, nodes) = if true {
    //                 //ast::next_is_begin_rule_named("and", nodes) { todo
    //                 rec_consume_and(eov, nodes)?
    //             } else {
    //                 (eov, nodes)
    //             };
    //             Ok((exprs, nodes))
    //         };
    //         //----

    //         let eov = eov.ipush(expr);
    //         match nodes.len() {
    //             0 => Ok((eov, nodes)),
    //             _ => consume_next_and(eov, nodes),
    //         }
    //     })
    // };
    // let build_and_expr = |vexpr| Expression::And(expression::MultiExpr(vexpr));
    // //-----

    // push_err!("pre-and", {
    //     let (eov, nodes) = rec_consume_and(ExprOrVecExpr::None, nodes)?;

    //     match eov {
    //         ExprOrVecExpr::None => Err(error_peg_s("logic error, empty or parsing???")),
    //         ExprOrVecExpr::Expr(e) => Ok((e, nodes)),
    //         ExprOrVecExpr::VExpr(v) => Ok((build_and_expr(v), nodes)),
    //     }
    // })

    // fn rec_consume_and(
    //     eov: ExprOrVecExpr,
    //     nodes: &[ast::Node],
    // ) -> result::Result<(ExprOrVecExpr, &[ast::Node]), Error> {
    //     let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("and", nodes)?;

    //     let (expr, sub_nodes) = consume_rep_or_neg(sub_nodes)?;
    //     let consume_next_and = |eov, nodes, sub_nodes| {
    //         let (exprs, sub_nodes) =
    //             match ast::consume_node_get_subnodes_for_rule_name_is("_1", sub_nodes) {
    //                 Ok((sub_nodes, _)) => rec_consume_and(eov, &sub_nodes)?,
    //                 _ => (eov, nodes),
    //             };
    //         ast::check_empty_nodes(sub_nodes)?;
    //         Ok((exprs, nodes))
    //     };
    //     //----

    //     let eov = eov.ipush(expr);
    //     match sub_nodes.len() {
    //         0 => Ok((eov, nodes)),
    //         _ => consume_next_and(eov, nodes, sub_nodes),
    //     }
    // };
    // let build_and_expr = |vexpr| Expression::And(expression::MultiExpr(vexpr));
    // //-----

    // push_err!("consuming and", {
    //     let (eov, nodes) = rec_consume_and(ExprOrVecExpr::None, nodes)?;

    //     match eov {
    //         ExprOrVecExpr::None => Err(error_peg_s("logic error, empty or parsing???")),
    //         ExprOrVecExpr::Expr(e) => Ok((e, nodes)),
    //         ExprOrVecExpr::VExpr(v) => Ok((build_and_expr(v), nodes)),
    //     }
    // })
}

fn consume_rep_or_neg_f(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // rep_or_neg      =   atom_or_par ("*" / "+" / "?")?
    //                 /   "!" atom_or_par

    // Err(error_peg_s("Not implemented"))

    fn process_repetition_indicator_f(
        expr: Expression,
        rsymbol: &str,
    ) -> result::Result<Expression, Error> {
        match rsymbol {
            "+" => Ok(rep!(expr, 1)),
            "*" => Ok(rep!(expr, 0)),
            "?" => Ok(rep!(expr, 0, 1)),
            unknown => Err(error_peg_s(&format!(
                "repetition symbol unknown {}",
                unknown
            ))),
        }
    }

    let atom_and_rep = |nodes| {
        let (expr, nodes) = consume_atom_or_par_f(nodes)?;
        let next_node = ast::peek_first_flat_node(nodes)?;

        match next_node {
            ast::FlatNode::Val(_) => {
                let (sep, nodes) = ast::flat_consume_val(nodes)?;
                Ok((process_repetition_indicator_f(expr, sep)?, nodes))
            }
            _ => Ok((expr, nodes)),
        }
    };
    let neg_and_atom = |nodes| -> result::Result<(Expression, &[ast::FlatNode]), Error> {
        let nodes = ast::flat_consume_this_value(r#"!"#, nodes)?;
        let (expr, nodes) = consume_atom_or_par_f(nodes)?;
        Ok((not!(expr), nodes))
    };

    consuming_rule("rep_or_neg", nodes, |nodes| {
        neg_and_atom(nodes).or_else(|_| atom_and_rep(nodes))
    })
}

fn consume_atom_or_par_f(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // atom_or_par     =   (atom / parenth)

    consuming_rule("atom_or_par", nodes, |nodes| {
        let next_node = ast::peek_first_flat_node(nodes)?;
        let node_name = ast::flat_get_nodename(next_node)?;

        let (expr, nodes) = push_err!(&format!("n:{}", node_name), {
            match &node_name as &str {
                "atom" => consume_atom_f(nodes),
                "parenth" => consume_parenth_f(nodes),
                unknown => Err(error_peg_s(&format!("unknown {}", unknown))),
            }
        })?;

        Ok((expr, nodes))
    })

    // push_err!("consuming atom_or_par", {
    //     let (nodes, sub_nodes) =
    //         ast::consume_node_get_subnodes_for_rule_name_is("atom_or_par", nodes)?;

    //     let (node, _) = ast::split_first_nodes(sub_nodes)?;
    //     let (node_name, _) = ast::get_nodename_and_nodes(node)?;

    //     let (expr, sub_nodes) = push_err!(&format!("n:{}", node_name), {
    //         match &node_name as &str {
    //             "atom" => consume_atom(sub_nodes),
    //             "parenth" => consume_parenth(sub_nodes),
    //             unknown => Err(error_peg_s(&format!("unknown {}", unknown))),
    //         }
    //     })?;

    //     ast::check_empty_nodes(sub_nodes)?;
    //     Ok((expr, nodes))
    // })
}

fn consume_atom_f(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // atom            =   literal
    //                 /   match
    //                 /   dot
    //                 /   symbol

    consuming_rule("atom", nodes, |nodes| {
        let next_node = ast::peek_first_flat_node(nodes)?;
        let node_name = ast::flat_get_nodename(next_node)?;

        let (expr, nodes) = push_err!(&format!("n:{}", node_name), {
            match &node_name as &str {
                "literal" => consume_literal_f(nodes),
                "symbol" => consume_symbol_rule_ref_f(nodes),
                "dot" => consume_dot_f(nodes),
                "match" => consume_match_f(nodes),
                unknown => Err(error_peg_s(&format!("unknown {}", unknown))),
            }
        })?;

        Ok((expr, nodes))
    })
    // push_err!("consuming atom", {
    //     let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("atom", nodes)?;
    //     ast::check_empty_nodes(nodes)?;

    //     let (node, _) = ast::split_first_nodes(sub_nodes)?;
    //     let (node_name, _) = ast::get_nodename_and_nodes(node)?;

    //     let (expr, sub_nodes) = push_err!(&format!("n:{}", node_name), {
    //         match &node_name as &str {
    //             "literal" => consume_literal(sub_nodes),
    //             "symbol" => consume_symbol_rule_ref(sub_nodes),
    //             "dot" => consume_dot(sub_nodes),
    //             "match" => consume_match(sub_nodes),
    //             unknown => Err(error_peg_s(&format!("unknown {}", unknown))),
    //         }
    //     })?;

    //     ast::check_empty_nodes(sub_nodes)?;
    //     Ok((expr, sub_nodes))
    // })
}

fn consume_parenth_f(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    //  parenth         =   "("  _  expr  _  ")"

    consuming_rule("parenth", nodes, |nodes| {
        let nodes = ast::flat_consume_this_value(r#"("#, nodes)?;
        let (expr, nodes) = consume_peg_expr_f(nodes)?;
        let nodes = ast::flat_consume_this_value(r#")"#, nodes)?;
        Ok((expr, nodes))
    })
}

fn consume_literal_f(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // literal         =   _" till_quote _"

    consuming_rule("literal", nodes, |nodes| {
        let nodes = consume_quote_f(nodes)?;
        let (val, nodes) = ast::flat_consume_val(nodes)?;

        push_err!(&format!("l:({})", val), {
            let nodes = consume_quote_f(nodes)?;
            Ok((lit!(val), nodes))
        })
    })
}

fn consume_quote_f(nodes: &[ast::FlatNode]) -> result::Result<&[ast::FlatNode], Error> {
    // _"              =   "\u{34}"

    Ok(consuming_rule(r#"_""#, nodes, |nodes| {
        Ok(((), ast::flat_consume_this_value(r#"""#, nodes)?))
    })?.1)
}

fn consume_dot_f(nodes: &[ast::FlatNode]) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    //  dot             =   "."

    consuming_rule("dot", nodes, |nodes| {
        let (_, nodes) = ast::flat_consume_val(nodes)?;
        Ok((dot!(), nodes))
    })
}

fn consume_symbol_rule_ref_f(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    push_err!("consuming symbol rule_ref", {
        let (symbol_name, nodes) = consume_symbol_f(nodes)?;

        Ok((ref_rule!(symbol_name), nodes))
    })
}

fn consume_match_f(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // match           =   "["
    //                         (
    //                             (mchars  mbetween*)
    //                             / mbetween+
    //                         )
    //                     "]"

    consuming_rule("match", nodes, |nodes| {
        fn rec_consume_mbetween_f(
            acc: Vec<(char, char)>,
            nodes: &[ast::FlatNode],
        ) -> result::Result<(Vec<(char, char)>, &[ast::FlatNode]), Error> {
            let next_node = ast::peek_first_flat_node(nodes)?;
            let node_name = ast::flat_get_nodename(next_node)?;
            match node_name {
                "mbetween" => {
                    let ((from, to), nodes) = consume_mbetween_f(nodes)?;
                    rec_consume_mbetween_f(acc.ipush((from, to)), nodes)
                }
                _ => Ok((acc, nodes)),
            }
        }

        let nodes = ast::flat_consume_this_value("[", nodes)?;

        let (expr, nodes) = consume_mchars_f(nodes)
            .and_then(|| rec_consume_mbetween())
            .and_then(|((chs, vbetw), nodes)| Ok((ematch!(chlist chs, from2 vbetw), nodes)))
            .or_else(|_| {
                rec_consume_mbetween(nodes)
                    .and_then(|(vbetw, nodes)| Ok((ematch!(chlist "", from2 vbetw), nodes)))
            })?;

        let nodes = ast::flat_consume_this_value("]", nodes)?;

        Ok((expr, nodes))
    })
}

fn consume_mchars_f(nodes: &[ast::FlatNode]) -> result::Result<(&str, &[ast::FlatNode]), Error> {
    // mchars          =   (!"]" !(. "-") .)+

    consuming_rule("mchars", nodes, |nodes| Ok(ast::flat_consume_val(nodes)?))
}

fn consume_mbetween_f(
    nodes: &[ast::FlatNode],
) -> result::Result<((char, char), &[ast::FlatNode]), Error> {
    // mbetween        =   (.  "-"  .)

    consuming_rule("mbetween", nodes, |nodes| {
        let (from, nodes) = ast::flat_consume_val(nodes)?;
        let nodes = ast::flat_consume_this_value("-", nodes)?;
        let (to, nodes) = ast::flat_consume_val(nodes)?;

        let ch_from = from.chars().next();
        let ch_to = to.chars().next();

        match (ch_from, ch_to, from.chars().count(), to.chars().count()) {
            (Some(f), Some(t), 1, 1) => Ok(((f, t), nodes)),
            _ => Err(error_peg_s(&format!(
                "from an to has to be a char from:{}, to:{}",
                from, to
            ))),
        }
    })
}

// fn consume_chars_mbetween_f(
//     nodes: &[ast::FlatNode],
// ) -> result::Result<((&str, VecChCh), &[ast::FlatNode]), Error> {
//     //  mchars+  mbetween*

//     consuming_rule("mchars", nodes, |nodes| {
//         let (val, nodes) = ast::flat_consume_val(nodes)?;

//         let (vb, nodes) = consume_mbetween_f(nodes)?;
//         Ok(((val, vb), nodes))
//     })
// }

// type VecChCh = Vec<(char, char)>;

// fn consume_mbetween_f(
//     nodes: &[ast::FlatNode],
// ) -> result::Result<(VecChCh, &[ast::FlatNode]), Error> {
//     // mbetween        =   (.  "-"  .)

//     fn rec_consume_between(
//         acc: VecChCh,
//         nodes: &[ast::FlatNode],
//     ) -> result::Result<(VecChCh, &[ast::FlatNode]), Error> {
//         let process_between = |acc: VecChCh, nodes| {
//             let (val, nodes) = ast::flat_consume_val(nodes)?;
//             let _u8_dash = b'-';
//             match val.as_bytes() {
//                 [init, _u8_dash, end] => Ok((acc.ipush((*init as char, *end as char)), nodes)),
//                 unknown => Err(error_peg_s(&format!("getting match.between {:?}", unknown))),
//             }
//         };

//         match ast::consume_flat_node_start_rule_name("mbetween", nodes) {
//             Ok(nodes) => {
//                 let (vb, _sub_nodes) = process_between(acc, nodes)?;
//                 rec_consume_between(vb, nodes)
//             }
//             Err(_) => Ok((acc, nodes)),
//         }
//     };

//     rec_consume_between(vec![], nodes)
// }

//  -----------------------------------------------------------

fn rules_from_ast(ast: &ast::Node) -> Result {
    // let ast = ast.compact().prune(&["_", "_eol"]);

    let vast = vec![ast.compact()];
    let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("main", &vast)?;
    ast::check_empty_nodes(nodes)?;
    let (rules, _sub_nodes) = consume_grammar(sub_nodes)?;

    Ok(rules)
}

fn consume_grammar(
    nodes: &[ast::Node],
) -> result::Result<(expression::SetOfRules, &[ast::Node]), Error> {
    //  grammar         =   rule+

    fn rec_consume_rules(
        rules: expression::SetOfRules,
        nodes: &[ast::Node],
    ) -> result::Result<(expression::SetOfRules, &[ast::Node]), Error> {
        if nodes.is_empty() {
            Ok((rules, nodes))
        } else {
            let (name, expr, nodes) = consume_rule(nodes)?;
            let rules = rules.add(name, expr);
            rec_consume_rules(rules, nodes)
        }
    }

    push_err!("consuming grammar", {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("grammar", nodes)?;
        ast::check_empty_nodes(nodes)?;

        let (rules, nodes) = rec_consume_rules(rules!(), sub_nodes)?;
        ast::check_empty_nodes(nodes)?;

        Ok((rules, nodes))
    })
}

fn consume_rule(
    nodes: &[ast::Node],
) -> result::Result<(&str, expression::Expression, &[ast::Node]), Error> {
    //  rule            =   symbol  _  "="  _   expr  (_ / eof)

    push_err!("consuming rule", {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("rule", nodes)?;
        let (symbol_name, sub_nodes) = consume_symbol(sub_nodes)?;

        push_err!(&format!("r:({})", symbol_name), {
            let sub_nodes = ast::consume_this_value("=", sub_nodes)?;
            let (expr, sub_nodes) = consume_peg_expr(sub_nodes)?;
            ast::check_empty_nodes(sub_nodes)?;

            Ok((symbol_name, expr, nodes))
        })
    })
}

fn consume_symbol(nodes: &[ast::Node]) -> result::Result<(&str, &[ast::Node]), Error> {
    //  symbol          =   [a-zA-Z0-9_']+

    push_err!("consuming symbol", {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("symbol", nodes)?;
        let value = ast::get_nodes_unique_val(sub_nodes)?;
        Ok((value, nodes))
    })
}

fn consume_peg_expr(nodes: &[ast::Node]) -> result::Result<(Expression, &[ast::Node]), Error> {
    //  expr            =   or

    push_err!("consuming expr", {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("expr", nodes)?;
        let (expr, sub_nodes) = consume_or(sub_nodes)?;
        ast::check_empty_nodes(sub_nodes)?;

        Ok((expr, nodes))
    })
}

//  This is to manage And & Or multiexpressions
//  in consume_or and consume_and
enum ExprOrVecExpr {
    Expr(Expression),
    VExpr(Vec<Expression>),
    None,
}
impl ExprOrVecExpr {
    fn ipush(self, expr: Expression) -> Self {
        match self {
            ExprOrVecExpr::Expr(e) => ExprOrVecExpr::VExpr(vec![e, expr]),
            ExprOrVecExpr::VExpr(v) => ExprOrVecExpr::VExpr(v.ipush(expr)),
            ExprOrVecExpr::None => ExprOrVecExpr::Expr(expr),
        }
    }
}

fn consume_or(nodes: &[ast::Node]) -> result::Result<(Expression, &[ast::Node]), Error> {
    //  or              =   and         ( _ "/"  _  or)*

    fn rec_consume_or(
        eov: ExprOrVecExpr,
        nodes: &[ast::Node],
    ) -> result::Result<(ExprOrVecExpr, &[ast::Node]), Error> {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("or", nodes)?;

        let (expr, sub_nodes) = consume_and(sub_nodes)?;
        let eov = eov.ipush(expr);

        let consume_next_or = |eov, nodes, sub_nodes| {
            let (exprs, sub_nodes) = match ast::consume_this_value("/", sub_nodes) {
                Ok(sub_nodes) => rec_consume_or(eov, &sub_nodes)?,
                _ => (eov, nodes),
            };
            ast::check_empty_nodes(sub_nodes)?;
            Ok((exprs, nodes))
        };
        // ----

        match sub_nodes.len() {
            0 => Ok((eov, nodes)),
            _ => consume_next_or(eov, nodes, sub_nodes),
        }
    };

    let build_or_expr = |vexpr| Expression::Or(expression::MultiExpr(vexpr));
    //-----

    push_err!("consuming or", {
        let (eov, nodes) = rec_consume_or(ExprOrVecExpr::None, nodes)?;

        match eov {
            ExprOrVecExpr::None => Err(error_peg_s("logic error, empty or parsing???")),
            ExprOrVecExpr::Expr(e) => Ok((e, nodes)),
            ExprOrVecExpr::VExpr(v) => Ok((build_or_expr(v), nodes)),
        }
    })
}

fn consume_and(nodes: &[ast::Node]) -> result::Result<(Expression, &[ast::Node]), Error> {
    //  and             =   rep_or_neg  (   " "  _  and)*

    fn rec_consume_and(
        eov: ExprOrVecExpr,
        nodes: &[ast::Node],
    ) -> result::Result<(ExprOrVecExpr, &[ast::Node]), Error> {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("and", nodes)?;

        let (expr, sub_nodes) = consume_rep_or_neg(sub_nodes)?;
        let consume_next_and = |eov, nodes, sub_nodes| {
            let (exprs, sub_nodes) =
                match ast::consume_node_get_subnodes_for_rule_name_is("_1", sub_nodes) {
                    Ok((sub_nodes, _)) => rec_consume_and(eov, &sub_nodes)?,
                    _ => (eov, nodes),
                };
            ast::check_empty_nodes(sub_nodes)?;
            Ok((exprs, nodes))
        };
        //----

        let eov = eov.ipush(expr);
        match sub_nodes.len() {
            0 => Ok((eov, nodes)),
            _ => consume_next_and(eov, nodes, sub_nodes),
        }
    };
    let build_and_expr = |vexpr| Expression::And(expression::MultiExpr(vexpr));
    //-----

    push_err!("consuming and", {
        let (eov, nodes) = rec_consume_and(ExprOrVecExpr::None, nodes)?;

        match eov {
            ExprOrVecExpr::None => Err(error_peg_s("logic error, empty or parsing???")),
            ExprOrVecExpr::Expr(e) => Ok((e, nodes)),
            ExprOrVecExpr::VExpr(v) => Ok((build_and_expr(v), nodes)),
        }
    })
}

fn consume_rep_or_neg(nodes: &[ast::Node]) -> result::Result<(Expression, &[ast::Node]), Error> {
    // rep_or_neg      =   atom_or_par ("*" / "+" / "?")?
    //                 /   "!" atom_or_par

    let atom_and_rep = |sub_nodes| {
        let (expr, sub_nodes) = consume_atom_or_par(sub_nodes)?;

        match sub_nodes {
            [node] => process_repetition_indicator(expr, node),
            [] => Ok(expr),
            _ => Err(error_peg_s("expected one node with repeticion info")),
        }
    };
    let neg_and_atom = |nodes| -> result::Result<Expression, Error> {
        let nodes = ast::consume_this_value(r#"!"#, nodes)?;
        let (expr, _) = consume_atom_or_par(nodes)?;
        Ok(not!(expr))
    };

    push_err!("consuming rep_or_neg", {
        let (nodes, sub_nodes) =
            ast::consume_node_get_subnodes_for_rule_name_is("rep_or_neg", nodes)?;

        let expr = neg_and_atom(sub_nodes).or_else(|_| atom_and_rep(sub_nodes))?;

        Ok((expr, nodes))
    })
}

fn process_repetition_indicator(
    expr: Expression,
    node: &ast::Node,
) -> result::Result<Expression, Error> {
    let rsymbol = ast::get_node_val(node)?;

    match rsymbol {
        "+" => Ok(rep!(expr, 1)),
        "*" => Ok(rep!(expr, 0)),
        "?" => Ok(rep!(expr, 0, 1)),
        unknown => Err(error_peg_s(&format!(
            "repetition symbol unknown {}",
            unknown
        ))),
    }
}

fn consume_atom_or_par(nodes: &[ast::Node]) -> result::Result<(Expression, &[ast::Node]), Error> {
    // atom_or_par     =   (atom / parenth)

    push_err!("consuming atom_or_par", {
        let (nodes, sub_nodes) =
            ast::consume_node_get_subnodes_for_rule_name_is("atom_or_par", nodes)?;

        let (node, _) = ast::split_first_nodes(sub_nodes)?;
        let (node_name, _) = ast::get_nodename_and_nodes(node)?;

        let (expr, sub_nodes) = push_err!(&format!("n:{}", node_name), {
            match &node_name as &str {
                "atom" => consume_atom(sub_nodes),
                "parenth" => consume_parenth(sub_nodes),
                unknown => Err(error_peg_s(&format!("unknown {}", unknown))),
            }
        })?;

        ast::check_empty_nodes(sub_nodes)?;
        Ok((expr, nodes))
    })
}

fn consume_atom(nodes: &[ast::Node]) -> result::Result<(Expression, &[ast::Node]), Error> {
    // atom            =   literal
    //                 /   match
    //                 /   dot
    //                 /   symbol

    push_err!("consuming atom", {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("atom", nodes)?;
        ast::check_empty_nodes(nodes)?;

        let (node, _) = ast::split_first_nodes(sub_nodes)?;
        let (node_name, _) = ast::get_nodename_and_nodes(node)?;

        let (expr, sub_nodes) = push_err!(&format!("n:{}", node_name), {
            match &node_name as &str {
                "literal" => consume_literal(sub_nodes),
                "symbol" => consume_symbol_rule_ref(sub_nodes),
                "dot" => consume_dot(sub_nodes),
                "match" => consume_match(sub_nodes),
                unknown => Err(error_peg_s(&format!("unknown {}", unknown))),
            }
        })?;

        ast::check_empty_nodes(sub_nodes)?;
        Ok((expr, sub_nodes))
    })
}

fn consume_parenth(nodes: &[ast::Node]) -> result::Result<(Expression, &[ast::Node]), Error> {
    //  parenth         =   "("  _  expr  _  ")"

    push_err!("consuming parenth", {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("parenth", nodes)?;
        ast::check_empty_nodes(nodes)?;

        let sub_nodes = ast::consume_this_value(r#"("#, sub_nodes)?;
        let (expr, sub_nodes) = consume_peg_expr(sub_nodes)?;
        let sub_nodes = ast::consume_this_value(r#")"#, sub_nodes)?;
        ast::check_empty_nodes(sub_nodes)?;
        Ok((expr, sub_nodes))
    })
}

fn consume_dot(nodes: &[ast::Node]) -> result::Result<(Expression, &[ast::Node]), Error> {
    //  dot             =   "."
    push_err!("consuming dot", {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("dot", nodes)?;
        ast::check_empty_nodes(nodes)?;

        let (_, sub_nodes) = ast::consume_val(sub_nodes)?;
        ast::check_empty_nodes(sub_nodes)?;

        Ok((dot!(), nodes))
    })
}

fn consume_match(nodes: &[ast::Node]) -> result::Result<(Expression, &[ast::Node]), Error> {
    // match           =   "["
    //                         (
    //                             (mchars+  mbetween*)
    //                             / mbetween+
    //                         )
    //                     "]"

    push_err!("consuming match", {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("match", nodes)?;
        ast::check_empty_nodes(nodes)?;

        let sub_nodes = ast::consume_this_value("[", sub_nodes)?;

        let (expr, sub_nodes) = consume_chars_mbetween(sub_nodes)
            .and_then(|(chs, vbetw, sub_nodes)| Ok((ematch!(chlist chs, from2 vbetw), sub_nodes)))
            .or_else(|_| {
                consume_mbetween(sub_nodes)
                    .and_then(|(vbetw, sub_nodes)| Ok((ematch!(chlist "", from2 vbetw), sub_nodes)))
            })?;

        let sub_nodes = ast::consume_this_value("]", sub_nodes)?;
        ast::check_empty_nodes(sub_nodes)?;

        Ok((expr, nodes))
    })
}

//type VecChCh = Vec<(char, char)>;

fn consume_mbetween(nodes: &[ast::Node]) -> result::Result<(VecChCh, &[ast::Node]), Error> {
    // mbetween        =   (.  "-"  .)

    fn rec_consume_between(
        acc: VecChCh,
        nodes: &[ast::Node],
    ) -> result::Result<(VecChCh, &[ast::Node]), Error> {
        let process_between = |acc: VecChCh, nodes| {
            let (val, nodes) = ast::consume_val(nodes)?;
            ast::check_empty_nodes(nodes)?;
            let _u8_dash = b'-';
            match val.as_bytes() {
                [init, _u8_dash, end] => Ok((acc.ipush((*init as char, *end as char)), nodes)),
                unknown => Err(error_peg_s(&format!(
                    "gettíng match.between {:?}",
                    unknown
                ))),
            }
        };

        match ast::consume_node_get_subnodes_for_rule_name_is("mbetween", nodes) {
            Ok((nodes, sub_nodes)) => {
                let (vb, _sub_nodes) = process_between(acc, sub_nodes)?;
                rec_consume_between(vb, nodes)
            }
            Err(_) => Ok((acc, nodes)),
        }
    };

    rec_consume_between(vec![], nodes)
}

fn consume_chars_mbetween(
    nodes: &[ast::Node],
) -> result::Result<(&str, VecChCh, &[ast::Node]), Error> {
    //  mchars+  mbetween*

    let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("mchars", nodes)?;
    let (val, sub_nodes) = ast::consume_val(sub_nodes)?;
    ast::check_empty_nodes(sub_nodes)?;
    let (vb, nodes) = consume_mbetween(nodes)?;

    Ok((val, vb, nodes))
}

fn consume_symbol_rule_ref(
    nodes: &[ast::Node],
) -> result::Result<(Expression, &[ast::Node]), Error> {
    push_err!("consuming symbol rule_ref", {
        let (symbol_name, nodes) = consume_symbol(nodes)?;
        ast::check_empty_nodes(nodes)?;

        Ok((ref_rule!(symbol_name), nodes))
    })
}

fn consume_literal(nodes: &[ast::Node]) -> result::Result<(Expression, &[ast::Node]), Error> {
    // literal         =   _" till_quote _"

    push_err!("consuming literal", {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is("literal", nodes)?;
        ast::check_empty_nodes(nodes)?;

        let sub_nodes = consume_quote(sub_nodes)?;
        let (val, sub_nodes) = ast::consume_val(sub_nodes)?;

        push_err!(&format!("l:({})", val), {
            let sub_nodes = consume_quote(sub_nodes)?;
            ast::check_empty_nodes(sub_nodes)?;
            Ok((lit!(val), sub_nodes))
        })
    })
}

fn consume_quote(nodes: &[ast::Node]) -> result::Result<&[ast::Node], Error> {
    // _"              =   "\u{34}"

    push_err!("consuming quote", {
        let (nodes, sub_nodes) = ast::consume_node_get_subnodes_for_rule_name_is(r#"_""#, nodes)?;
        let sub_nodes = ast::consume_this_value(r#"""#, sub_nodes)?;
        ast::check_empty_nodes(sub_nodes)?;

        Ok(nodes)
    })
}
