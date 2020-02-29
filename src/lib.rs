#![warn(missing_docs)]
// #![feature(external_doc)]
// #![doc(include = "../README.md")]

//! For an introduction and context view, read...
//!
//! [README.md](https://github.com/jleahred/dynparser)
//!
//! A very basic example...
//! ```rust
//!    extern crate dynparser;
//!    use dynparser::{parse, rules_from_peg};
//!
//!    fn main() {
//!        let rules = rules_from_peg(
//!            r#"
//!
//!    main            =   letter letter_or_num+
//!
//!    letter          =   [a-zA-Z]
//!
//!    letter_or_num   =   letter
//!                    /   number
//!
//!    number          =   [0-9]
//!
//!            "#,
//!        ).unwrap();
//!
//!        assert!(parse("a2AA456bzJ88", &rules).is_ok());
//!    }
//!```
//!
//!
//! The classical calculator example
//!
//! ```rust
//! extern crate dynparser;
//! use dynparser::{parse, rules_from_peg};
//!
//!
//! fn main() {
//!     let rules = rules_from_peg(
//!         r#"
//!
//!     main            =   _  expr  _
//!
//!     expr            =   add_t       (_  add_op  _   add_t)*
//!                     /   portion_expr
//!
//!     add_t           =   fact_t      (_  fact_op _   fact_t)*
//!
//!     fact_t          =   portion_expr
//!
//!     portion_expr    =   '(' expr ')'
//!                     /   item
//!
//!     item            =   num
//!
//!     num             =   [0-9]+ ('.' [0-9]+)?
//!     add_op          =   '+'  /  '-'
//!     fact_op         =   '*'  /  '/'
//!
//!     _               =   ' '*
//!
//!         "#,
//!     ).map_err(|e| {
//!         println!("{}", e);
//!         panic!("FAIL");
//!     })
//!         .unwrap();
//!
//!     let result = parse(" 1 +  2*  3 +(5/5 - (8-7))", &rules);
//!     match result {
//!         Ok(ast) => println!(
//!             "{:#?}",
//!             ast.compact()
//!                 .prune(&vec!["_"])
//!                 .pass_through_except(&vec!["main", "add_t", "fact_t"])
//!         ),
//!         Err(e) => println!("Error: {:?}", e),
//!     };
//! }
//! ```
//!
//! Please, read [README.md](https://github.com/jleahred/dynparser) for
//! more context information
//!
//! ```rust
//! extern crate dynparser;
//! use dynparser::{parse, rules_from_peg};
//! fn main() {
//!     let rules = rules_from_peg(
//!         r#"
//!
//!     main    =   '('  main   ( ')'  /  error("unbalanced parenthesis") )
//!             /   'hello'
//!
//!         "#,
//!     ).unwrap();
//!
//!     match parse("((hello)", &rules) {
//!         Ok(_) => panic!("It should fail"),
//!         Err(e) => assert!(e.descr == "unbalanced parenthesis"),
//!     }
//! }
//! ```

extern crate idata;

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
#[cfg_attr(feature = "cargo-clippy", allow(clippy::let_and_return))]
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

/// Generate an error
///
/// example
/// ```
/// #[macro_use]  extern crate dynparser;
/// use dynparser::parse;
///
/// fn main() {
///     let rules = rules!{
///        "main"   =>  error!("aa")
///     };
///
///     assert!(parse("aa", &rules).is_err())
/// }
/// ```
///
/// ```rust
/// extern crate dynparser;
/// use dynparser::{parse, rules_from_peg};
/// fn main() {
///     let rules = rules_from_peg(
///         r#"
///
///     main    =   '('  main   ( ')'  /  error("unbalanced parenthesis") )
///             /   'hello'
///
///         "#,
///     ).unwrap();
///
///     match parse("((hello)", &rules) {
///         Ok(_) => panic!("It should fail"),
///         Err(e) => assert!(e.descr == "unbalanced parenthesis"),
///     }
/// }
/// ```

#[macro_export]
macro_rules! error {
    ($e:expr) => {{
        $crate::parser::expression::Expression::Simple($crate::parser::atom::Atom::Error(
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
        //use idata::cont::IVec;  //  pending macros by example 2.0
        use $crate::parser;
        let mut v = Vec::<(char, char)>::new();

        //$(let v = v.ipush(($from, $to));)+  //  pending macros by example 2.0
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
/// not! will not move the parsing position
#[macro_export]
macro_rules! not {
    ($e:expr) => {{
        $crate::parser::expression::Expression::Not(Box::new($e))
    }};
}

/// repeat expression.
/// You have to define minimum repetitions and optionally
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

/// This will create a subexpression referring to a "rule name"
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
pub mod peg;

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
/// More examples in macros
///

pub fn parse(s: &str, rules: &parser::expression::SetOfRules) -> Result<ast::Node, parser::Error> {
    parse_with_debug(s, rules, false)
}

/// Same as parser, but with debug info
///
/// It will trace the rules called
///
/// It's expensive, use it just to develop and locate errors
///
pub fn parse_debug(
    s: &str,
    rules: &parser::expression::SetOfRules,
) -> Result<ast::Node, parser::Error> {
    parse_with_debug(s, rules, true)
}

fn parse_with_debug(
    s: &str,
    rules: &parser::expression::SetOfRules,
    debug: bool,
) -> Result<ast::Node, parser::Error> {
    let (st, ast) = if debug {
        parser::expression::parse(parser::Status::init_debug(s, &rules, debug))?
    } else {
        parser::expression::parse(parser::Status::init(s, &rules))?
    };
    match (st.pos.n == s.len(), st.potential_error.clone()) {
        (true, _) => Ok(ast),
        (false, Some(e)) => Err(e),
        (false, None) => Err(parser::Error::from_status_normal(
            &st,
            "not consumed full input",
        )),
    }
}

pub use peg::rules_from_peg;

//  A P I
// -------------------------------------------------------------------------------------

//-----------------------------------------------------------------------
//  I N T E R N A L
