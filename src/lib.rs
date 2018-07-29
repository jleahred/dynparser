#![warn(missing_docs)]

// #![feature(external_doc)]
// #![doc(include = "../README.md")]

//! This library is to create parsing rules, and parsing text
//! dynamically (not compile time).
//!
//! It lets you to create rules to parse text and it will generate an AST
//!
//! examples
//!
//! ```
//! #[macro_use]  extern crate dynparser;
//! use dynparser::parse;
//!
//! fn main() {
//!     let rules = rules!{
//!        "main"   =>  and!{
//!                         lit!("aa"),
//!                         ref_rule!("rule2")
//!                     },
//!        "rule2"  =>  and!{
//!                         lit!("b"),
//!                         lit!("c")
//!                     }
//!     };
//!
//!     assert!(parse("aabc", &rules).is_ok())
//! }
//!
//! ```
//!
//! main is de starting rule to parse
//!
//! ```
//! #[macro_use]  extern crate dynparser;
//! use dynparser::parse;
//!
//! fn main() {
//!     let rules = rules!{
//!        "main"   =>  and!{
//!                         rep!(lit!("a"), 1, 5),
//!                         ref_rule!("rule2")
//!                     },
//!         "rule2" =>  or!{
//!                         lit!("zz"),
//!                         not!{ lit!("bc") },
//!                         and!{
//!                             lit!("b"),
//!                             dot!(),
//!                             ematch!(    chlist "abcd",
//!                                         from 'a', to 'd',
//!                                         from 'j', to 'p'
//!                             )
//!                         }
//!                     }
//!     };
//!
//!     assert!(parse("aabcd", &rules).is_ok())
//! }
//! ```
//!
//! This is a dynamic parser, therefore, it's important to add rules at runtime
//!
//! ```
//! #[macro_use]  extern crate dynparser;
//! use dynparser::parse;
//!
//! fn main() {
//!     let rules = rules!{
//!        "main"   =>  and!{
//!                         rep!(lit!("a"), 1, 5),
//!                         ref_rule!("rule2")
//!                     }
//!     };
//!
//!     let rules = rules.add("rule2", lit!("bcd"));
//!
//!     assert!(parse("aabcd", &rules).is_ok())
//! }
//! ```
//!
//! ```add``` take the ownership and returs a "new" (in fact modified)
//! set of rules. This helps to reduce mutability
//!
//! Remember, you can use recursion in order to manage dinamically
//!
//!
//! And ofcourse, you can add several rules at once
//!
//! ```
//! #[macro_use]  extern crate dynparser;
//! use dynparser::parse;
//!
//! fn main() {
//!     let r = rules!{
//!        "main"   =>  and!{
//!                         rep!(lit!("a"), 1, 5),
//!                         ref_rule!("rule2")
//!                     }
//!     };
//!
//!     let r = r.merge(rules!{"rule2" => lit!("bcd")});
//!
//!     assert!(parse("aabcd", &r).is_ok())
//! }
//! ```
//!
//! ```merge``` take the ownership and returs a "new" (in fact modified)
//! set of rules. This helps to reduce mutability
//!

// -------------------------------------------------------------------------------------
//  M A C R O S

/// Create a map of rules
///
/// example
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  and!{
///                         lit!("aa"),
///                         ref_rule!("rule2")
///                     },
///        "rule2"  =>  and!{
///                         lit!("b"),
///                         lit!("c")
///                     }
///     };
///
///     assert!(parse("aabc", &rules).is_ok())
/// }
/// ```
#[macro_export]
macro_rules! rules {
    ($($n:expr => $e:expr),*) => {{
        use $crate::parser::expression;
        use std::collections::HashMap;

        let rules = expression::SetOfRules::new(HashMap::<String, expression::Expression>::new());
        $(let rules = rules.add($n, $e);)*
        rules
    }};
}

/// Create a literal
///
/// example
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  lit!("aa")
///     };
///
///     assert!(parse("aa", &rules).is_ok())
/// }
/// ```
#[macro_export]
macro_rules! lit {
    ($e:expr) => {{
        $crate::parser::expression::Expression::Simple($crate::parser::atom::Atom::Literal(
            $e.to_string(),
        ))
    }};
}

/// Atom::Dot (any character)
///
/// example
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  and!(dot!(), dot!())
///     };
///
///     assert!(parse("aa", &rules).is_ok())
/// }
/// ```
#[macro_export]
macro_rules! dot {
    () => {{
        $crate::parser::expression::Expression::Simple($crate::parser::atom::Atom::Dot)
    }};
}

/// Generate a match expression with optional characters and a list
/// of bounds
///
///  "String", from 'a', to 'b', from 'c', to 'd'
/// The first string, is a set of chars.
/// Later you can write a list of tuples with ranges to validate
///
/// example
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  rep!(ematch!(    chlist "cd",
///                                         from 'a', to 'b',
///                                         from 'j', to 'p'
///                     ), 0)
///     };
///
///     assert!(parse("aabcdj", &rules).is_ok())
/// }
/// ```
///
///
/// You can also pass a list of chars and a vector of char bounds as next
/// example
///
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  rep!(ematch!(    chlist "cd",
///                                      from2   vec![
///                                             ('a', 'b'),
///                                             ('j', 'p')
///                                         ]
///                     ), 0)
///     };
///
///     assert!(parse("aabcdj", &rules).is_ok())
/// }
/// ```

#[macro_export]
macro_rules! ematch {
    (chlist $chars:expr, $(from $from:expr,  to $to:expr),*) => {{
        use $crate::parser;
        let mut v = Vec::<(char, char)>::new();

        $(v.push(($from, $to));)+
        let amatch = parser::atom::Atom::Match(parser::atom::MatchRules::init($chars, v));
        parser::expression::Expression::Simple(amatch)
    }};

    (chlist $chars:expr, from2 $vfrom2:expr) => {{
        use $crate::parser;

        let amatch = parser::atom::Atom::Match(parser::atom::MatchRules::init($chars, $vfrom2));
        parser::expression::Expression::Simple(amatch)
    }};
}

/// Concat expressions (and)
///
/// example
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  and!(dot!(), dot!())
///     };
///
///     assert!(parse("aa", &rules).is_ok())
/// }
/// ```
#[macro_export]
macro_rules! and {
    ($($e:expr),*) => {{
        use $crate::parser::expression::{Expression, MultiExpr};

        Expression::And(MultiExpr::new(vec![$($e ,)*]))
    }};
}

/// Choose expressions (or)
///
/// example
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  or!(lit!("z"), lit!("a"))
///     };
///
///     assert!(parse("a", &rules).is_ok())
/// }
/// ```
#[macro_export]
macro_rules! or {
    ($($e:expr),*) => {{
        use $crate::parser::expression::{Expression, MultiExpr};

        Expression::Or(MultiExpr::new(vec![$($e ,)*]))
    }};
}

/// negate expression
///
/// example
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  and!(not!(lit!("b")), dot!())
///     };
///
///     assert!(parse("a", &rules).is_ok())
/// }
/// ```
///
/// not! will not move the parsing possition
#[macro_export]
macro_rules! not {
    ($e:expr) => {{
        $crate::parser::expression::Expression::Not(Box::new($e))
    }};
}

/// repeat expression.
/// You have to define minimum repetitions and opionally
/// maximum repetitions (if missing, infinite)
///
/// example
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  rep!(lit!("a"), 0)
///     };
///
///     assert!(parse("aaaaaaaa", &rules).is_ok())
/// }
/// ```
/// repeating from 0 to infinite
///
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  rep!(lit!("a"), 0, 3)
///     };
///
///     assert!(parse("aaa", &rules).is_ok())
/// }
/// ```
#[macro_export]
macro_rules! rep {
    ($e:expr, $min:expr) => {{
        use $crate::parser::expression;

        expression::Expression::Repeat(expression::RepInfo::new(Box::new($e), $min, None))
    }};

    ($e:expr, $min:expr, $max:expr) => {{
        use $crate::parser::expression;

        expression::Expression::Repeat(expression::RepInfo::new(Box::new($e), $min, Some($max)))
    }};
}

/// This will create a subexpression refering to a "rule name"
///
/// ```
/// #[macro_use]  extern crate dynparser;
///
/// fn main() {
///     let rules = rules!{
///        "main" => ref_rule!("3a"),
///        "3a"   => lit!("aaa")
///     };
///
///     assert!(dynparser::parse("aaa", &rules).is_ok())
/// }
/// ```
#[macro_export]
macro_rules! ref_rule {
    ($e:expr) => {{
        $crate::parser::expression::Expression::RuleName($e.to_owned())
    }};
}

//  M A C R O S
// -------------------------------------------------------------------------------------

pub mod ast;
pub mod parser;
mod peg;

// -------------------------------------------------------------------------------------
//  T Y P E S

//  T Y P E S
// -------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------
//  A P I

/// Parse a string with a set of rules
///
/// the `main` rule is the starting point to parse
///
/// # Examples
///
/// Parse a simple literal
///
/// ```
/// #[macro_use]  extern crate dynparser;
///
/// fn main() {
///     let rules = rules!{
///        "main" => ref_rule!("3a"),
///        "3a"   => lit!("aaa")
///     };
///
///     assert!(dynparser::parse("aaa", &rules).is_ok())
/// }
///
/// ```
/// More examples in marcros
///

pub fn parse(s: &str, rules: &parser::expression::SetOfRules) -> Result<ast::Node, parser::Error> {
    let (st, ast) = parser::expression::parse(parser::Status::init(s, &rules))?;
    match st.pos.n == s.len() {
        true => Ok(ast),
        false => Err(parser::Error::from_status(&st, "not consumed full input")),
    }
}

pub use peg::rules_from_peg;

//  A P I
// -------------------------------------------------------------------------------------

//-----------------------------------------------------------------------
//  I N T E R N A L
