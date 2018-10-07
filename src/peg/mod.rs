#![warn(missing_docs)]
//! Module with functions to generate rules from PEG grammar
//!

pub mod gcode;
mod rules;

use ast;
use idata::{self, IVec};
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

pub fn rules_from_peg(peg: &str) -> Result {
    let ast = parse(peg, &rules::parse_peg())?;
    let nodes = ast.compact().prune(&["_", "_1", "_eol"]).flattern();

    rules_from_flat_ast(&nodes)
}

//  A P I
// -------------------------------------------------------------------------------------

fn rules_from_flat_ast(nodes: &[ast::FlatNode]) -> Result {
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

    consuming_rule("main", nodes, |nodes| consume_grammar(&nodes))
}

fn consume_grammar(
    nodes: &[ast::FlatNode],
) -> result::Result<(expression::SetOfRules, &[ast::FlatNode]), Error> {
    // grammar         =   rule+

    fn rec_consume_rules(
        rules: expression::SetOfRules,
        nodes: &[ast::FlatNode],
    ) -> result::Result<(expression::SetOfRules, &[ast::FlatNode]), Error> {
        match ast::peek_first_flat_node(nodes)? {
            ast::FlatNode::BeginRule(_) => {
                let ((name, expr), nodes) = consume_rule(nodes)?;
                let rules = rules.add(name, expr);
                rec_consume_rules(rules, nodes)
            }
            _ => Ok((rules, nodes)),
        }
    }
    //  --------------------------

    consuming_rule("grammar", nodes, |nodes| {
        rec_consume_rules(rules!(), &nodes)
    })
}

fn consume_rule(
    nodes: &[ast::FlatNode],
) -> result::Result<((&str, expression::Expression), &[ast::FlatNode]), Error> {
    // rule            =   _  symbol  _  "="  _  expr  _eol _

    consuming_rule("rule", nodes, |nodes| {
        let (symbol_name, nodes) = consume_symbol(nodes)?;
        let nodes = ast::flat_consume_this_value("=", nodes)?;
        let (expr, nodes) = consume_peg_expr(nodes)?;

        Ok(((symbol_name, expr), nodes))
    })
}

fn consume_symbol(nodes: &[ast::FlatNode]) -> result::Result<(&str, &[ast::FlatNode]), Error> {
    // symbol          =   [_'a-zA-Z0-9] [_'"a-zA-Z0-9]*

    consuming_rule("symbol", nodes, |nodes| {
        let (val, nodes) = ast::flat_consume_val(nodes)?;
        Ok((val, nodes))
    })
}

fn consume_peg_expr(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    //  expr            =   or

    consuming_rule("expr", nodes, |nodes| consume_or(nodes))
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

fn consume_or(nodes: &[ast::FlatNode]) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    //  or              =   and         ( _ "/"  _  or)*

    fn rec_consume_or(
        eov: ExprOrVecExpr,
        nodes: &[ast::FlatNode],
    ) -> result::Result<(ExprOrVecExpr, &[ast::FlatNode]), Error> {
        consuming_rule("or", nodes, move |nodes| {
            let (expr, nodes) = consume_and(nodes)?;
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
    //  --------------------------

    push_err!("or:", {
        let (eov, nodes) = rec_consume_or(ExprOrVecExpr::None, nodes)?;

        match eov {
            ExprOrVecExpr::None => Err(error_peg_s("logic error, empty or parsing???")),
            ExprOrVecExpr::Expr(e) => Ok((e, nodes)),
            ExprOrVecExpr::VExpr(v) => Ok((build_or_expr(v), nodes)),
        }
    })
}

fn consume_and(nodes: &[ast::FlatNode]) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // and             =   rep_or_neg  ( _1 _ !(symbol _ "=") and )*

    fn rec_consume_and(
        eov: ExprOrVecExpr,
        nodes: &[ast::FlatNode],
    ) -> result::Result<(ExprOrVecExpr, &[ast::FlatNode]), Error> {
        consuming_rule("and", nodes, move |nodes| {
            let (expr, nodes) = consume_rep_or_neg(nodes)?;
            let eov = eov.ipush(expr);
            let next_node = ast::peek_first_flat_node(nodes)?;

            match (next_node, ast::flat_get_nodename(next_node)) {
                (ast::FlatNode::BeginRule(_), Ok("and")) => rec_consume_and(eov, nodes),
                _ => Ok((eov, nodes)),
            }
        })
    }

    let build_and_expr = |vexpr| Expression::And(expression::MultiExpr(vexpr));
    //  --------------------------

    let (eov, nodes) = rec_consume_and(ExprOrVecExpr::None, nodes)?;
    match eov {
        ExprOrVecExpr::None => Err(error_peg_s("logic error, empty or parsing???")),
        ExprOrVecExpr::Expr(e) => Ok((e, nodes)),
        ExprOrVecExpr::VExpr(v) => Ok((build_and_expr(v), nodes)),
    }
}

fn consume_rep_or_neg(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // rep_or_neg      =   atom_or_par ("*" / "+" / "?")?
    //                 /   "!" atom_or_par

    fn process_repetition_indicator(
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
        let (expr, nodes) = consume_atom_or_par(nodes)?;
        let next_node = ast::peek_first_flat_node(nodes)?;

        match next_node {
            ast::FlatNode::Val(_) => {
                let (sep, nodes) = ast::flat_consume_val(nodes)?;
                Ok((process_repetition_indicator(expr, sep)?, nodes))
            }
            _ => Ok((expr, nodes)),
        }
    };
    let neg_and_atom = |nodes| -> result::Result<(Expression, &[ast::FlatNode]), Error> {
        let nodes = ast::flat_consume_this_value(r#"!"#, nodes)?;
        let (expr, nodes) = consume_atom_or_par(nodes)?;
        Ok((not!(expr), nodes))
    };
    //  --------------------------

    consuming_rule("rep_or_neg", nodes, |nodes| {
        neg_and_atom(nodes).or_else(|_| atom_and_rep(nodes))
    })
}

fn consume_atom_or_par(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // atom_or_par     =   (atom / parenth)

    consuming_rule("atom_or_par", nodes, |nodes| {
        let next_node = ast::peek_first_flat_node(nodes)?;
        let node_name = ast::flat_get_nodename(next_node)?;

        let (expr, nodes) = push_err!(&format!("n:{}", node_name), {
            match &node_name as &str {
                "atom" => consume_atom(nodes),
                "parenth" => consume_parenth(nodes),
                unknown => Err(error_peg_s(&format!("unknown {}", unknown))),
            }
        })?;

        Ok((expr, nodes))
    })
}

fn consume_atom(nodes: &[ast::FlatNode]) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // atom            =   literal
    //                 /   match
    //                 /   dot
    //                 /   symbol

    consuming_rule("atom", nodes, |nodes| {
        let next_node = ast::peek_first_flat_node(nodes)?;
        let node_name = ast::flat_get_nodename(next_node)?;

        let (expr, nodes) = push_err!(&format!("n:{}", node_name), {
            match &node_name as &str {
                "literal" => consume_literal(nodes),
                "symbol" => consume_symbol_rule_ref(nodes),
                "dot" => consume_dot(nodes),
                "match" => consume_match(nodes),
                unknown => Err(error_peg_s(&format!("unknown {}", unknown))),
            }
        })?;

        Ok((expr, nodes))
    })
}

fn consume_parenth(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    //  parenth         =   "("  _  expr  _  ")"

    consuming_rule("parenth", nodes, |nodes| {
        let nodes = ast::flat_consume_this_value(r#"("#, nodes)?;
        let (expr, nodes) = consume_peg_expr(nodes)?;
        let nodes = ast::flat_consume_this_value(r#")"#, nodes)?;
        Ok((expr, nodes))
    })
}

fn consume_literal(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // literal         =   _" till_quote _"

    consuming_rule("literal", nodes, |nodes| {
        let nodes = consume_quote(nodes)?;
        let (val, nodes) = ast::flat_consume_val(nodes)?;

        push_err!(&format!("l:({})", val), {
            let nodes = consume_quote(nodes)?;
            Ok((lit!(val), nodes))
        })
    })
}

fn consume_quote(nodes: &[ast::FlatNode]) -> result::Result<&[ast::FlatNode], Error> {
    // _"              =   "\u{34}"

    Ok(consuming_rule(r#"_""#, nodes, |nodes| {
        Ok(((), ast::flat_consume_this_value(r#"""#, nodes)?))
    })?.1)
}

fn consume_dot(nodes: &[ast::FlatNode]) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    //  dot             =   "."

    consuming_rule("dot", nodes, |nodes| {
        let (_, nodes) = ast::flat_consume_val(nodes)?;
        Ok((dot!(), nodes))
    })
}

fn consume_symbol_rule_ref(
    nodes: &[ast::FlatNode],
) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    push_err!("consuming symbol rule_ref", {
        let (symbol_name, nodes) = consume_symbol(nodes)?;

        Ok((ref_rule!(symbol_name), nodes))
    })
}

fn consume_match(nodes: &[ast::FlatNode]) -> result::Result<(Expression, &[ast::FlatNode]), Error> {
    // match           =   "["
    //                         (
    //                             (mchars  mbetween*)
    //                             / mbetween+
    //                         )
    //                     "]"

    type VecChCh = Vec<(char, char)>;
    consuming_rule("match", nodes, |nodes| {
        fn rec_consume_mbetween(
            acc: Vec<(char, char)>,
            nodes: &[ast::FlatNode],
        ) -> result::Result<(VecChCh, &[ast::FlatNode]), Error> {
            let next_node = ast::peek_first_flat_node(nodes)?;
            let node_name = ast::flat_get_nodename(next_node);
            match node_name {
                Ok("mbetween") => {
                    let ((from, to), nodes) = consume_mbetween(nodes)?;
                    rec_consume_mbetween(acc.ipush((from, to)), nodes)
                }
                _ => Ok((acc, nodes)),
            }
        }
        //  --------------------------

        let nodes = ast::flat_consume_this_value("[", nodes)?;

        let (ochars, nodes) = match consume_mchars(nodes) {
            Ok((chars, nodes)) => (Some(chars), nodes),
            _ => (None, nodes),
        };

        let (vchars, nodes) = rec_consume_mbetween(vec![], nodes)?;

        let (expr, nodes) = match (ochars, vchars.is_empty()) {
            (Some(chars), true) => Ok((ematch!(chlist chars, from2 vec![]), nodes)),
            (Some(chars), false) => Ok((ematch!(chlist chars, from2 vchars), nodes)),
            (None, false) => Ok((ematch!(chlist "", from2 vchars), nodes)),
            _ => Err(error_peg_s("Invalid match combination")),
        }?;

        let nodes = ast::flat_consume_this_value("]", nodes)?;

        Ok((expr, nodes))
    })
}

fn consume_mchars(nodes: &[ast::FlatNode]) -> result::Result<(&str, &[ast::FlatNode]), Error> {
    // mchars          =   (!"]" !(. "-") .)+

    consuming_rule("mchars", nodes, |nodes| Ok(ast::flat_consume_val(nodes)?))
}

fn consume_mbetween(
    nodes: &[ast::FlatNode],
) -> result::Result<((char, char), &[ast::FlatNode]), Error> {
    // mbetween        =   (.  "-"  .)

    consuming_rule("mbetween", nodes, |nodes| {
        let (from_to, nodes) = ast::flat_consume_val(nodes)?;

        let (from, chars) = idata::consume_char(from_to.chars())
            .ok_or_else(|| error_peg_s("expected from char"))?;
        let (_, chars) =
            idata::consume_char(chars).ok_or_else(|| error_peg_s("expected '-' char"))?;
        let (to, _) = idata::consume_char(chars).ok_or_else(|| error_peg_s("expected to char"))?;;
        Ok(((from, to), nodes))
    })
}
