// #![feature(external_doc)]
// #![doc(include = "../README.md")]

// #[macro_use]
mod parser;

// -------------------------------------------------------------------------------------
//  T Y P E S

#[derive(PartialEq, Clone, Debug)]
pub struct Possition {
    /// char position parsing
    pub n: usize,
    /// row parsing row
    pub row: usize,
    /// parsing col
    pub col: usize,
}

pub struct Error {
    pub pos: Possition,
    pub descr: String,
    pub line: String,
}

//  T Y P E S
// -------------------------------------------------------------------------------------

#[macro_export]
macro_rules! lit {
    ($e:expr) => {{
        ::parser::expression::Expression::Simple(::parser::atom::Atom::Literal($e))
    }};
}

#[macro_export]
macro_rules! and {
    ($($e:expr),*) => {{
        use parser::expression::{Expression, MultiExpr};

        Expression::And(MultiExpr(vec![$($e ,)*]))
    }};
}

#[macro_export]
macro_rules! or {
    ($($e:expr),*) => {{
        use parser::expression::{Expression, MultiExpr};

        Expression::Or(MultiExpr(vec![$($e ,)*]))
    }};
}

#[macro_export]
macro_rules! not {
    ($e:expr) => {{
        use parser::expression::Expression;

        Expression::Not(Box::new($e))
    }};
}

#[macro_export]
macro_rules! rep {
    ($e:expr, $min:expr) => {{
        use parser::expression::{Expression, NRep, RepInfo};

        Expression::Repeat(RepInfo {
            expression: Box::new($e),
            min: NRep($min),
            max: None,
        })
    }};

    ($e:expr, $min:expr, $max:expr) => {{
        use parser::expression::{Expression, NRep, RepInfo};

        Expression::Repeat(RepInfo {
            expression: Box::new($e),
            min: NRep($min),
            max: Some(NRep($max)),
        })
    }};
}

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
/// #[macro_use]
/// extern crate dynparser;
///    
/// fn main() {
///     assert!(parse("aaa", lit!("aaa")));
/// }
///
/// ```
/// Another example
///
/// ```
///
///
/// ```
///

pub fn parse(s: &str, expr: &parser::expression::Expression) -> Result<(), Error> {
    let (st, _) = parser::expression::parse(parser::Status::init(s), expr)?;
    match st.pos.n == s.len() {
        true => Ok(()),
        false => Err(Error::from_status(&st, "not consumed full input")),
    }
}

//  A P I
// -------------------------------------------------------------------------------------

//-----------------------------------------------------------------------
//  I N T E R N A L

impl Possition {
    #[allow(dead_code)]
    fn init() -> Self {
        Self {
            n: 0,
            row: 0,
            col: 0,
        }
    }
}
