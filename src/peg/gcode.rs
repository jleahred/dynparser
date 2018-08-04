#![warn(missing_docs)]
//! Generate source code from a set of rules
//!

use parser::{
    atom, atom::Atom, expression::{self, Expression},
};

/// Generate a string with rust code from a ```expression::SetOfRules```
pub fn rust_from_rules(rules: &expression::SetOfRules) -> String {
    rules.0.iter().fold("".to_string(), |acc, (name, expr)| {
        format!("{}\n{}", rule2code(name, expr), acc)
    })
}

fn rule2code(name: &str, expr: &Expression) -> String {
    format!("\"{}\" = {},", name.to_string(), expr2code(expr))
}

fn expr2code(expr: &Expression) -> String {
    match expr {
        Expression::Simple(atom) => atom2code(atom),
        Expression::And(mexpr) => format!("and!({})", mexpr2code(mexpr)),
        Expression::Or(mexpr) => format!("or!({})", mexpr2code(mexpr)),
        Expression::Not(e) => format!("not!({})", expr2code(e)),
        Expression::Repeat(rep) => repeat2code(rep),
        Expression::RuleName(rname) => format!("rule!(\"{}\")", rname),
    }
}

fn mexpr2code(mexpr: &expression::MultiExpr) -> String {
    mexpr
        .0
        .iter()
        .fold(String::new(), |acc, expr| match acc.len() {
            0 => format!("{}", expr2code(expr)),
            _ => format!("{}, {}", acc, expr2code(expr)),
        })
}

fn atom2code(atom: &Atom) -> String {
    match atom {
        Atom::Literal(s) => format!("lit!(\"{}\")", s),
        Atom::Match(mrules) => match_rules2code(mrules),
        Atom::Dot => format!("dot!()"),
        Atom::EOF => format!("eof!()"),
    }
}

fn match_rules2code(mrules: &atom::MatchRules) -> String {
    fn bounds2code(acc: String, bounds: &[(char, char)]) -> String {
        match bounds.split_first() {
            Some(((f, t), rest)) => bounds2code(format!("from '{}', to '{}', ", f, t), rest),
            None => acc,
        }
    }

    format!(
        "ematch!(chlist {}, {})",
        mrules.0,
        bounds2code(String::new(), &mrules.1)
    )
}

fn repeat2code(rep: &expression::RepInfo) -> String {
    format!(
        "rep!({}, {:?}, {:?})",
        expr2code(&rep.expression),
        rep.min,
        rep.max
    )
}
