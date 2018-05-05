//-----------------------------------------------------------------------
//
//  mod parser::atom  TEST
//
//-----------------------------------------------------------------------
use super::Status;
use super::{parse_dot, parse_eof, parse_literal, parse_match, MatchRules};

#[test]
fn test_parse_literal_ok() {
    let status_init = Status::init("aaaaaaaaaaaaaaaa");
    let status_end = parse_literal(status_init, "aaa").ok().unwrap();

    assert!(status_end.pos.col == 3);
    assert!(status_end.pos.n == 3);
    assert!(status_end.pos.row == 0);
}

#[test]
fn test_parse_literal_ok2() {
    let status_init = Status::init("abcdefghij");
    let status_end = parse_literal(status_init, "abc").ok().unwrap();

    assert_eq!(status_end.pos.col, 3);
    assert_eq!(status_end.pos.n, 3);
    assert_eq!(status_end.pos.row, 0);
}

#[test]
fn test_parse_literal_fail() {
    let status_init = Status::init("abcdefghij");
    assert!(parse_literal(status_init, "bbb").is_err());
}

#[test]
fn test_parse_literal_fail2() {
    let status_init = Status::init("abcdefghij");
    assert!(parse_literal(status_init, "abd").is_err());
}

#[test]
fn test_parse_literal_fail_short_text2parse() {
    let status_init = Status::init("abcd");
    assert!(parse_literal(status_init, "abcdefghij").is_err());
}

#[test]
fn test_parse_literal_with_new_line() {
    let status_init = Status::init(
        "aa
aaaaaaaaaaaaaa",
    );
    let status_end = parse_literal(
        status_init,
        "aa
a",
    ).ok()
        .unwrap();

    assert!(status_end.pos.col == 1);
    assert!(status_end.pos.row == 1);
}

#[test]
fn test_parse_dot() {
    let status = Status::init("ab");

    let status = parse_dot(status).ok().unwrap();
    assert!(status.pos.col == 1);
    assert!(status.pos.n == 1);
    assert!(status.pos.row == 0);

    let status = parse_dot(status).ok().unwrap();
    assert!(status.pos.col == 2);
    assert!(status.pos.n == 2);
    assert!(status.pos.row == 0);

    assert!(parse_dot(status).is_err());
}

#[test]
fn test_parse_match_ok() {
    let status = Status::init("a f0ghi");

    let match_rules = MatchRules::new().with_chars("54321ed_cba");
    let status = parse_match(status, &match_rules).ok().unwrap();
    assert_eq!(status.pos.col, 1);
    assert_eq!(status.pos.n, 1);
    assert_eq!(status.pos.row, 0);

    let status = parse_dot(status).ok().unwrap();

    let match_rules = MatchRules::new().with_bound_chars(&[('f', 'g'), ('h', 'j')]);
    let status = parse_match(status, &match_rules).ok().unwrap();
    assert_eq!(status.pos.col, 3);
    assert_eq!(status.pos.n, 3);
    assert_eq!(status.pos.row, 0);

    assert!(parse_match(status, &match_rules).is_err());
}

#[test]
fn test_parse_match_err() {
    let status = Status::init("a9");

    let match_rules = MatchRules::new().with_chars("ed_cba");
    let status = parse_match(status, &match_rules).ok().unwrap();
    assert_eq!(status.pos.col, 1);
    assert_eq!(status.pos.n, 1);
    assert_eq!(status.pos.row, 0);

    let match_rules = MatchRules::new().with_bound_chars(&[('a', 'z'), ('0', '8')]);
    assert!(parse_match(status, &match_rules).is_err());
}

#[test]
fn test_parse_match_eof_ok() {
    let status = Status::init("a");

    let match_rules = MatchRules::new().with_bound_chars(&[('a', 'z'), ('0', '9')]);
    let status = parse_match(status, &match_rules).ok().unwrap();

    assert!(parse_eof(status).is_ok());
}

#[test]
fn test_parse_match_eof_error() {
    let status = Status::init("ab");

    let match_rules = MatchRules::new().with_bound_chars(&[('a', 'z'), ('0', '9')]);
    let status = parse_match(status, &match_rules).ok().unwrap();

    assert!(parse_eof(status).is_err());
}
