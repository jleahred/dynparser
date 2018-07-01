//-----------------------------------------------------------------------
//
//  mod parser::expression  TEST
//
//-----------------------------------------------------------------------

use super::{parse_expr, Expression, MultiExpr, NRep, RepInfo, Status};
use parser::atom::Atom;

#[test]
fn test_parse_literal_ok() {
    let rules = rules!{};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let expr = Expression::Simple(Atom::Literal("aaa"));
    let result = parse_expr(status_init, &expr).ok().unwrap();

    assert!(result.status.pos.col == 3);
    assert!(result.status.pos.n == 3);
    assert!(result.status.pos.row == 0);
}

#[test]
fn test_parse_literal_error() {
    let rules = rules!{};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let expr = Expression::Simple(Atom::Literal("bb"));
    assert!(parse_expr(status_init, &expr).is_err());
}

#[test]
fn test_parse_and_ok() {
    let rules = rules!{};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let and_rules = vec![
        Expression::Simple(Atom::Literal("aa")),
        Expression::Simple(Atom::Literal("aa")),
    ];
    let expr = Expression::And(MultiExpr(and_rules));

    let result = parse_expr(status_init, &expr).ok().unwrap();

    assert_eq!(result.status.pos.col, 4);
    assert_eq!(result.status.pos.n, 4);
    assert_eq!(result.status.pos.row, 0);
}

#[test]
fn test_parse_and_fail() {
    let rules = rules!{};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let and_rules = vec![
        Expression::Simple(Atom::Literal("aa")),
        Expression::Simple(Atom::Literal("bb")),
    ];
    let expr = Expression::And(MultiExpr(and_rules));

    assert!(parse_expr(status_init, &expr).is_err());
}

#[test]
fn test_parse_not_ok() {
    let rules = rules!{};
    let status_init = Status::init("aa", &rules);

    let expr_not = Expression::Not(Box::new(Expression::Simple(Atom::Literal("bb"))));
    let result = parse_expr(status_init, &expr_not).ok().unwrap();

    assert_eq!(result.status.pos.col, 0);
    assert_eq!(result.status.pos.n, 0);
    assert_eq!(result.status.pos.row, 0);
}

#[test]
fn test_parse_not_fail() {
    let rules = rules!{};
    let status_init = Status::init("aa", &rules);

    let expr_not = Expression::Not(Box::new(Expression::Simple(Atom::Literal("aa"))));
    assert!(parse_expr(status_init, &expr_not).is_err());
}

#[test]
fn test_parse_or_ok() {
    let rules = rules!{};
    {
        let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
        let rules = vec![
            Expression::Simple(Atom::Literal("aa")),
            Expression::Simple(Atom::Literal("aa")),
        ];
        let expr = Expression::Or(MultiExpr(rules));

        let result = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(result.status.pos.col, 2);
        assert_eq!(result.status.pos.n, 2);
        assert_eq!(result.status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
        let rules = vec![
            Expression::Simple(Atom::Literal("aa")),
            Expression::Simple(Atom::Literal("bb")),
        ];
        let expr = Expression::Or(MultiExpr(rules));

        let result = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(result.status.pos.col, 2);
        assert_eq!(result.status.pos.n, 2);
        assert_eq!(result.status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
        let rules = vec![
            Expression::Simple(Atom::Literal("bb")),
            Expression::Simple(Atom::Literal("aa")),
        ];
        let expr = Expression::Or(MultiExpr(rules));

        let result = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(result.status.pos.col, 2);
        assert_eq!(result.status.pos.n, 2);
        assert_eq!(result.status.pos.row, 0);
    }
}

#[test]
fn test_parse_or_fail() {
    let rules = rules!{};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let and_rules = vec![
        Expression::Simple(Atom::Literal("cc")),
        Expression::Simple(Atom::Literal("bb")),
    ];
    let expr = Expression::And(MultiExpr(and_rules));

    assert!(parse_expr(status_init, &expr).is_err());
}

#[test]
fn test_parse_repeat_ok() {
    let rules = rules!{};
    let repeat_literal = |literal, min, max: Option<NRep>| {
        Expression::Repeat(RepInfo {
            expression: Box::new(Expression::Simple(Atom::Literal(literal))),
            min: min,
            max: max,
        })
    };

    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("aa", NRep(0), None);
        let result = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(result.status.pos.col, 6);
        assert_eq!(result.status.pos.n, 6);
        assert_eq!(result.status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("aa", NRep(3), None);
        let result = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(result.status.pos.col, 6);
        assert_eq!(result.status.pos.n, 6);
        assert_eq!(result.status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("aa", NRep(0), Some(NRep(3)));
        let result = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(result.status.pos.col, 6);
        assert_eq!(result.status.pos.n, 6);
        assert_eq!(result.status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("aa", NRep(0), Some(NRep(1)));
        let result = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(result.status.pos.col, 2);
        assert_eq!(result.status.pos.n, 2);
        assert_eq!(result.status.pos.row, 0);
    }
    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("bb", NRep(0), None);
        let result = parse_expr(status_init, &expr).ok().unwrap();

        assert_eq!(result.status.pos.col, 0);
        assert_eq!(result.status.pos.n, 0);
        assert_eq!(result.status.pos.row, 0);
    }
}

#[test]
fn test_parse_repeat_fail() {
    let rules = rules!{};
    let repeat_literal = |literal, min, max: Option<NRep>| {
        Expression::Repeat(RepInfo {
            expression: Box::new(Expression::Simple(Atom::Literal(literal))),
            min: min,
            max: max,
        })
    };

    {
        let status_init = Status::init("aaaaaa", &rules);
        let expr = repeat_literal("aa", NRep(4), None);
        assert!(parse_expr(status_init, &expr).is_err());
    }
}
