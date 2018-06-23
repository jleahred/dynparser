//-----------------------------------------------------------------------
//
//  mod parser  TEST
//
//-----------------------------------------------------------------------

use parser::{expression::parse_expr, Status};

#[test]
fn test_parse_expr_lit() {
    let rules = rules!{};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);

    let expr = lit!("aaa");

    let result = parse_expr(status_init, &expr).ok().unwrap();
    assert!(result.0.pos.col == 3);
    assert!(result.0.pos.n == 3);
    assert!(result.0.pos.row == 0);
}

#[test]
fn test_parse_expr_and_ok() {
    let rules = rules!{};
    let status_init = Status::init("aabbcc", &rules);

    let expr = and![lit!("aa"), and![lit!("bb"), lit!("cc")]];

    let result = parse_expr(status_init, &expr).ok().unwrap();
    assert_eq!(result.0.pos.col, 6);
    assert_eq!(result.0.pos.n, 6);
    assert_eq!(result.0.pos.row, 0);
}

#[test]
fn test_parse_expr_or_ok() {
    let rules = rules!{};
    let status_init = Status::init("aabb", &rules);

    let expr = or![lit!("bb"), and![lit!("aa"), lit!("bb")]];

    let result = parse_expr(status_init, &expr).ok().unwrap();
    assert_eq!(result.0.pos.col, 4);
    assert_eq!(result.0.pos.n, 4);
    assert_eq!(result.0.pos.row, 0);
}

#[test]
fn test_parse_expr_not_ok() {
    let rules = rules!{};
    let status_init = Status::init("aa", &rules);

    let expr = not!(lit!("bb"));

    let result = parse_expr(status_init, &expr).ok().unwrap();
    assert_eq!(result.0.pos.col, 0);
    assert_eq!(result.0.pos.n, 0);
    assert_eq!(result.0.pos.row, 0);
}

#[test]
fn test_parse_expr_repeat_ok() {
    let rules = rules!{};
    {
        let status_init = Status::init("aaaaaa", &rules);

        let expr = rep![lit!("aa"), 3];

        let result = parse_expr(status_init, &expr).ok().unwrap();
        assert_eq!(result.0.pos.col, 6);
        assert_eq!(result.0.pos.n, 6);
        assert_eq!(result.0.pos.row, 0);
    }

    {
        let status_init = Status::init("aaaaaa", &rules);

        let expr = rep![lit!("aa"), 0, 3];

        let result = parse_expr(status_init, &expr).ok().unwrap();
        assert_eq!(result.0.pos.col, 6);
        assert_eq!(result.0.pos.n, 6);
        assert_eq!(result.0.pos.row, 0);
    }
}
