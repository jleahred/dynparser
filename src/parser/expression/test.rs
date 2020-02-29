//-----------------------------------------------------------------------
//
//  mod parser::expression  TEST
//
//-----------------------------------------------------------------------

use super::{parse_expr, Expression, MultiExpr, NRep, RepInfo, Status};
use crate::parser::atom::Atom;

#[test]
fn test_parse_literal_ok() {
    let rules = rules! {};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let expr = Expression::Simple(Atom::Literal("aaa".to_string()));
    let (status, _) = parse_expr(status_init, &expr).ok().unwrap();

    assert!(status.pos.col == 3);
    assert!(status.pos.n == 3);
    assert!(status.pos.row == 0);
}

#[test]
fn test_parse_literal_error() {
    let rules = rules! {};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let expr = Expression::Simple(Atom::Literal("bb".to_string()));
    assert!(parse_expr(status_init, &expr).is_err());
}

#[test]
fn test_parse_and_ok() {
    let rules = rules! {};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let and_rules = vec![
        Expression::Simple(Atom::Literal("aa".to_string())),
        Expression::Simple(Atom::Literal("aa".to_string())),
    ];
    let expr = Expression::And(MultiExpr(and_rules));

    let (status, _) = parse_expr(status_init, &expr).ok().unwrap();

    assert_eq!(status.pos.col, 4);
    assert_eq!(status.pos.n, 4);
    assert_eq!(status.pos.row, 0);
}

#[test]
fn test_parse_and_fail() {
    let rules = rules! {};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let and_rules = vec![
        Expression::Simple(Atom::Literal("aa".to_string())),
        Expression::Simple(Atom::Literal("bb".to_string())),
    ];
    let expr = Expression::And(MultiExpr(and_rules));

    assert!(parse_expr(status_init, &expr).is_err());
}

#[test]
fn test_parse_not_ok() {
    let rules = rules! {};
    let status_init = Status::init("aa", &rules);

    let expr_not = Expression::Not(Box::new(Expression::Simple(Atom::Literal(
        "bb".to_string(),
    ))));
    let (status, _) = parse_expr(status_init, &expr_not).ok().unwrap();

    assert_eq!(status.pos.col, 0);
    assert_eq!(status.pos.n, 0);
    assert_eq!(status.pos.row, 0);
}

#[test]
fn test_parse_not_fail() {
    let rules = rules! {};
    let status_init = Status::init("aa", &rules);

    let expr_not = Expression::Not(Box::new(Expression::Simple(Atom::Literal(
        "aa".to_string(),
    ))));
    assert!(parse_expr(status_init, &expr_not).is_err());
}

#[test]
fn test_parse_or_ok() {
    let rules = rules! {};
    {
        let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
        let rules = vec![
            Expression::Simple(Atom::Literal("aa".to_string())),
            Expression::Simple(Atom::Literal("aa".to_string())),
        ];
        let expr = Expression::Or(MultiExpr(rules));

        let (status, _) = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(status.pos.col, 2);
        assert_eq!(status.pos.n, 2);
        assert_eq!(status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
        let rules = vec![
            Expression::Simple(Atom::Literal("aa".to_string())),
            Expression::Simple(Atom::Literal("bb".to_string())),
        ];
        let expr = Expression::Or(MultiExpr(rules));

        let (status, _) = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(status.pos.col, 2);
        assert_eq!(status.pos.n, 2);
        assert_eq!(status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
        let rules = vec![
            Expression::Simple(Atom::Literal("bb".to_string())),
            Expression::Simple(Atom::Literal("aa".to_string())),
        ];
        let expr = Expression::Or(MultiExpr(rules));

        let (status, _) = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(status.pos.col, 2);
        assert_eq!(status.pos.n, 2);
        assert_eq!(status.pos.row, 0);
    }
}

#[test]
fn test_parse_or_fail() {
    let rules = rules! {};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let and_rules = vec![
        Expression::Simple(Atom::Literal("cc".to_string())),
        Expression::Simple(Atom::Literal("bb".to_string())),
    ];
    let expr = Expression::And(MultiExpr(and_rules));

    assert!(parse_expr(status_init, &expr).is_err());
}

#[test]
fn test_parse_repeat_ok() {
    let rules = rules! {};
    let repeat_literal = |literal, min, max: Option<NRep>| {
        Expression::Repeat(RepInfo {
            expression: Box::new(Expression::Simple(Atom::Literal(literal))),
            min,
            max,
        })
    };

    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("aa".to_string(), NRep(0), None);
        let (status, _) = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(status.pos.col, 6);
        assert_eq!(status.pos.n, 6);
        assert_eq!(status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("aa".to_string(), NRep(3), None);
        let (status, _) = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(status.pos.col, 6);
        assert_eq!(status.pos.n, 6);
        assert_eq!(status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("aa".to_string(), NRep(0), Some(NRep(3)));
        let (status, _) = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(status.pos.col, 6);
        assert_eq!(status.pos.n, 6);
        assert_eq!(status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("aa".to_string(), NRep(0), Some(NRep(1)));
        let (status, _) = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(status.pos.col, 2);
        assert_eq!(status.pos.n, 2);
        assert_eq!(status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("bb".to_string(), NRep(0), None);
        let (status, _) = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(status.pos.col, 0);
        assert_eq!(status.pos.n, 0);
        assert_eq!(status.pos.row, 0);
    }
}

#[test]
fn test_parse_repeat_fail() {
    let rules = rules! {};
    let repeat_literal = |literal, min, max: Option<NRep>| {
        Expression::Repeat(RepInfo {
            expression: Box::new(Expression::Simple(Atom::Literal(literal))),
            min,
            max,
        })
    };

    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("aa".to_string(), NRep(4), None);
        assert!(parse_expr(status_init, &expr).is_err());
    }
}
