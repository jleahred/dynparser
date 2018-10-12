#![warn(missing_docs)]
//! Generate source code from a set of rules
//!
//! example
//! ```rust
//! extern crate dynparser;
//! use dynparser::{peg, rules_from_peg};
//!
//! fn main() {
//!     let rules = rules_from_peg(
//!         r#"
//!
//!             main            =   as / a  bs
//!
//!             as              =   a+
//!
//!             a               =   'a'
//!
//!             bs              =   'b'+
//!
//! "#,
//!     ).map_err(|e| {
//!         println!("{}", e);
//!         panic!("FAIL");
//!     })
//!         .unwrap();
//!
//!     println!("{}", peg::gcode::rust_from_rules(&rules))
//! }
//! ```
//! The parser itself uses a peg grammar and generate the rules with this
//! function
//!
//! The rules code generated by this program will be
//! ```ignore
//!      "as" => rep!(ref_rule!("a"), 1)
//!    , "a" => lit!("a")
//!    , "main" => or!(ref_rule!("as"), and!(ref_rule!("a"), ref_rule!("bs")))
//!    , "bs" => rep!(lit!("b"), 1)
//! ```

use parser::{
    atom,
    atom::Atom,
    expression::{self, Expression},
};

/// Generate a string with rust code from a ```expression::SetOfRules```
pub fn rust_from_rules(rules: &expression::SetOfRules) -> String {
    let add_rule = |crules: String, rule: &str| -> String {
        let begin = if crules == "" { "  " } else { ", " };
        crules + "\n       " + begin + rule
    };

    rules.0.iter().fold("".to_string(), |acc, (name, expr)| {
        add_rule(acc, &rule2code(name, expr))
    })
}

fn rule2code(name: &str, expr: &Expression) -> String {
    format!(r##"r#"{}"# => {}"##, name, expr2code(expr))
}

fn expr2code(expr: &Expression) -> String {
    match expr {
        Expression::Simple(atom) => atom2code(atom),
        Expression::And(mexpr) => format!("and!({})", mexpr2code(mexpr)),
        Expression::Or(mexpr) => format!("or!({})", mexpr2code(mexpr)),
        Expression::Not(e) => format!("not!({})", expr2code(e)),
        Expression::Repeat(rep) => repeat2code(rep),
        Expression::RuleName(rname) => format!(r##"ref_rule!(r#"{}"#)"##, rname),
    }
}

fn mexpr2code(mexpr: &expression::MultiExpr) -> String {
    mexpr
        .0
        .iter()
        .fold(String::new(), |acc, expr| match acc.len() {
            0 => expr2code(expr).to_string(),
            _ => format!("{}, {}", acc, expr2code(expr)),
        })
}

fn atom2code(atom: &Atom) -> String {
    let replace_esc = |s: String| {
        s.replace("\n", r#"\n"#)
            .replace("\r", r#"\r"#)
            .replace("\t", r#"\t"#)
            .replace(r#"""#, r#"\""#)
    };

    match atom {
        Atom::Literal(s) => format!(r#"lit!("{}")"#, replace_esc(s.to_string())),
        Atom::Error(s) => format!(r#"error!("{}")"#, replace_esc(s.to_string())),
        Atom::Match(mrules) => match_rules2code(mrules),
        Atom::Dot => "dot!()".to_string(),
        Atom::EOF => "eof!()".to_string(),
    }
}

fn match_rules2code(mrules: &atom::MatchRules) -> String {
    fn bounds2code(acc: String, bounds: &[(char, char)]) -> String {
        match bounds.split_first() {
            Some(((f, t), rest)) => {
                format!(", from '{}', to '{}' {}", f, t, bounds2code(acc, rest))
            }
            None => acc,
        }
    }

    format!(
        r##"ematch!(chlist r#"{}"#  {})"##,
        &mrules.0,
        bounds2code(String::new(), &mrules.1)
    )
}

fn repeat2code(rep: &expression::RepInfo) -> String {
    "rep!(".to_owned()
        + &expr2code(&rep.expression)
        + ", "
        + &rep.min.0.to_string()
        + &match rep.max {
            Some(ref m) => format!(", {}", m.0),
            None => "".to_owned(),
        }
        + ")"
}
