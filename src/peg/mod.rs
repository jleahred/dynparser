#![warn(missing_docs)]
//! Module with functions to generate rules from PEG grammar
//!

mod rules;

use ast;
use parse;
use parser::{
    self, expression::{self, Expression},
};
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

pub type Result<'a> = result::Result<expression::SetOfRules<'a>, Error>;

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

pub fn rules_from_peg(peg: &str) -> Result {
    let ast = parse(peg, &rules::parse_peg())?;

    // println!("{:#?}", ast);
    rules_from_ast(&ast)
}

//  A P I
// -------------------------------------------------------------------------------------

fn rules_from_ast<'a>(ast: &ast::Node) -> Result<'a> {
    let ast = ast.compact().prune(&vec!["_"]);
    println!(":::::::  {:#?}", ast);

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
) -> result::Result<(expression::SetOfRules<'a>, &[ast::Node]), Error> {
    //  grammar         =   rule+

    fn rec_consume_peg_rules<'a, 'b>(
        rules: expression::SetOfRules<'a>,
        nodes: &'b [ast::Node],
    ) -> result::Result<(expression::SetOfRules<'a>, &'b [ast::Node]), Error> {
        if nodes.len() == 0 {
            Ok((rules, nodes))
        } else {
            let (name, expr, nodes) = consume_peg_rule(nodes)?;
            let rules = rules.add(name, expr);
            rec_consume_peg_rules(rules, nodes)
        }
    }

    push_err!("consuming grammar", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("grammar", nodes)?;
        check_empty_nodes(nodes)?;

        let (rules, nodes) = rec_consume_peg_rules(rules!(), sub_nodes)?;
        check_empty_nodes(nodes)?;

        Ok((rules, nodes))
    })
}

fn consume_peg_rule<'a>(
    nodes: &[ast::Node],
) -> result::Result<(&str, expression::Expression<'a>, &[ast::Node]), Error> {
    //  rule            =   symbol  _  "="  _   expr  (_ / eof)

    push_err!("consuming rule", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("rule", nodes)?;
        let (symbol_name, sub_nodes) = consume_symbol(sub_nodes)?;

        push_err!(&format!("r:({})", symbol_name), {
            let sub_nodes = consume_if_val("=", sub_nodes)?;
            let (expr, sub_nodes) = consume_peg_expr(sub_nodes)?;
            check_empty_nodes(sub_nodes)?;

            Ok((symbol_name, expr, nodes))
        })
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
) -> result::Result<(Expression<'a>, &[ast::Node]), Error> {
    //  expr            =   or

    push_err!("consuming expr", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("expr", nodes)?;
        check_empty_nodes(nodes)?;
        let (expr, sub_nodes) = consume_or(sub_nodes)?;
        check_empty_nodes(sub_nodes)?;

        Ok((expr, nodes))
    })
}

fn consume_or<'a>(nodes: &[ast::Node]) -> result::Result<(Expression<'a>, &[ast::Node]), Error> {
    //  or              =   and         ( _ "/"  _  or)*

    fn rec_consume_or<'a, 'b>(
        nodes: &'a [ast::Node],
        exprs: Vec<Expression<'b>>,
    ) -> result::Result<(Vec<Expression<'b>>, &'a [ast::Node]), Error> {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("or", nodes)?;

        let (expr, sub_nodes) = consume_and(sub_nodes)?;
        let exprs = exprs.mpush(expr);

        match sub_nodes.len() {
            0 => Ok((exprs, nodes)),
            _ => {
                let (exprs, nodes) = match consume_if_val("/", sub_nodes) {
                    Ok(sub_nodes) => rec_consume_or(&sub_nodes, exprs)?,
                    _ => (exprs, nodes),
                };

                Ok((exprs, nodes))
            }
        }
    };

    push_err!("consuming or", {
        let (vexpr, nodes) = rec_consume_or(nodes, vec![])?;
        let and_expr = Expression::Or(expression::MultiExpr(vexpr));

        Ok((and_expr, nodes))
    })
}

trait IVec<T> {
    fn mpush(self, T) -> Self;
}

impl<T> IVec<T> for Vec<T>
where
    T: std::fmt::Debug,
{
    fn mpush(mut self, v: T) -> Self {
        self.push(v);
        self
    }
}

fn consume_and<'a>(nodes: &[ast::Node]) -> result::Result<(Expression<'a>, &[ast::Node]), Error> {
    //  and             =   rep_or_neg  (   " "  _  and)*

    fn rec_consume_and<'a, 'b>(
        nodes: &'a [ast::Node],
        exprs: Vec<Expression<'b>>,
    ) -> result::Result<(Vec<Expression<'b>>, &'a [ast::Node]), Error> {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("and", nodes)?;

        let (expr, sub_nodes) = consume_rep_or_neg(sub_nodes)?;
        let exprs = exprs.mpush(expr);

        match sub_nodes.len() {
            0 => Ok((exprs, nodes)),
            _ => {
                let (exprs, nodes) = match consume_if_val(" ", sub_nodes) {
                    Ok(sub_nodes) => rec_consume_and(&sub_nodes, exprs)?,
                    _ => (exprs, nodes),
                };

                Ok((exprs, nodes))
            }
        }
    };

    push_err!("consuming and", {
        let (vexpr, nodes) = rec_consume_and(nodes, vec![])?;
        let and_expr = Expression::And(expression::MultiExpr(vexpr));

        Ok((and_expr, nodes))
    })
}

fn consume_rep_or_neg<'a>(
    nodes: &[ast::Node],
) -> result::Result<(Expression<'a>, &[ast::Node]), Error> {
    // rep_or_neg      =   atom_or_par ("*" / "+" / "?")?
    //                 /   "!" atom_or_par

    push_err!("consuming rep_or_neg", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("rep_or_neg", nodes)?;
        let (expr, sub_nodes) = consume_atom_or_par(sub_nodes)?;
        check_empty_nodes(sub_nodes)?;

        Ok((expr, nodes))
    })
}

fn consume_atom_or_par<'a>(
    nodes: &[ast::Node],
) -> result::Result<(Expression<'a>, &[ast::Node]), Error> {
    // atom_or_par     =   (atom / parenth)

    push_err!("consuming atom_or_par", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("atom_or_par", nodes)?;
        check_empty_nodes(nodes)?;
        let (expr, sub_nodes) = consume_atom(sub_nodes)?;
        check_empty_nodes(sub_nodes)?;

        Ok((expr, nodes))
    })
}

fn consume_atom<'a>(nodes: &[ast::Node]) -> result::Result<(Expression<'a>, &[ast::Node]), Error> {
    // atom            =   literal
    //                 /   match
    //                 /   dot
    //                 /   symbol

    push_err!("consuming atom", {
        let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("atom", nodes)?;
        check_empty_nodes(nodes)?;

        let (node, sub_nodes) = ast::split_first_nodes(sub_nodes)?;
        let (node_name, atom_nodes) = ast::get_nodename_and_nodes(node)?;

        let expr = push_err!(&format!("n:{}", node_name), {
            let (expr, _sub_nodes) = match &node_name as &str {
                "literal" => consume_inside_literal(atom_nodes),
                "symbol" => consume_inside_symbol(atom_nodes),
                unknown => Err(error_peg_s(&format!("unknown {}", unknown))),
            }?;
            Ok(expr)
        })?;

        Ok((expr, sub_nodes))
    })
}

fn consume_inside_symbol<'a>(
    nodes: &[ast::Node],
) -> result::Result<(Expression<'a>, &[ast::Node]), Error> {
    // symbol          =   [a-zA-Z0-9_'][a-zA-Z0-9_'"]+

    push_err!("consuming inside symbol", {
        // let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("literal", nodes)?;
        // check_empty_nodes(nodes)?;
        let sub_nodes = nodes;
        let (val, sub_nodes) = ast::consume_val(sub_nodes)?;

        check_empty_nodes(sub_nodes)?;
        Ok((rule!(val), sub_nodes))
    })
}

fn consume_inside_literal<'a>(
    nodes: &[ast::Node],
) -> result::Result<(Expression<'a>, &[ast::Node]), Error> {
    // literal         =   _" till_quote _"

    push_err!("consuming inside literal", {
        // let (nodes, sub_nodes) = consume_node_get_subnodes_if_rule_name_is("literal", nodes)?;
        // check_empty_nodes(nodes)?;
        let sub_nodes = nodes;
        let sub_nodes = consume_quote(sub_nodes)?;
        let (val, sub_nodes) = ast::consume_val(sub_nodes)?;

        push_err!(&format!("l:({})", val), {
            let sub_nodes = consume_quote(sub_nodes)?;
            check_empty_nodes(sub_nodes)?;
            Ok((lit!(val), sub_nodes))
        })
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
            Err(error_peg_s(&format!(
                "expected {} node, received {}",
                name, n
            )))
        },
        unknown => Err(error_peg_s(&format!(
            "expected {} Node::Rule, not received {:?}",
            name, unknown
        ))),
    }
}

fn check_empty_nodes(nodes: &[ast::Node]) -> result::Result<(), Error> {
    match nodes.is_empty() {
        true => Ok(()),
        false => Err(error_peg_s("not consumed full nodes")),
    }
}
