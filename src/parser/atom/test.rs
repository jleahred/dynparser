//-----------------------------------------------------------------------
//
//  mod parser::atom  TEST
//
//-----------------------------------------------------------------------
use super::Status;
use super::{parse_dot, parse_eof, parse_literal, parse_match, MatchRules};

#[test]
fn test_parse_literal_ok() {
    let rules = rules!{};
    let status_init = Status::init("aaaaaaaaaaaaaaaa", &rules);
    let (status_end, _) = parse_literal(status_init, "aaa").ok().unwrap();

    assert!(status_end.pos.col == 3);
    assert!(status_end.pos.n == 3);
    assert!(status_end.pos.row == 0);
}

#[test]
fn test_parse_literal_ok2() {
    let rules = rules!{};
    let status_init = Status::init("abcdefghij", &rules);
    let (status_end, _) = parse_literal(status_init, "abc").ok().unwrap();

    assert_eq!(status_end.pos.col, 3);
    assert_eq!(status_end.pos.n, 3);
    assert_eq!(status_end.pos.row, 0);
}

#[test]
fn test_parse_literal_fail() {
    let rules = rules!{};
    let status_init = Status::init("abcdefghij", &rules);
    assert!(parse_literal(status_init, "bbb").is_err());
}

#[test]
fn test_parse_literal_fail2() {
    let rules = rules!{};
    let status_init = Status::init("abcdefghij", &rules);
    assert!(parse_literal(status_init, "abd").is_err());
}

#[test]
fn test_parse_literal_fail_short_text2parse() {
    let rules = rules!{};
    let status_init = Status::init("abcd", &rules);
    assert!(parse_literal(status_init, "abcdefghij").is_err());
}

#[test]
fn test_parse_literal_with_new_line() {
    let rules = rules!{};
    let status_init = Status::init(
        "aa
aaaaaaaaaaaaaa",
        &rules,
    );
    let (status_end, _) = parse_literal(
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
    let rules = rules!{};
    let status = Status::init("ab", &rules);

    let (status, _) = parse_dot(status).ok().unwrap();
    assert!(status.pos.col == 1);
    assert!(status.pos.n == 1);
    assert!(status.pos.row == 0);

    let (status, _) = parse_dot(status).ok().unwrap();
    assert!(status.pos.col == 2);
    assert!(status.pos.n == 2);
    assert!(status.pos.row == 0);

    assert!(parse_dot(status).is_err());
}

#[test]
fn test_parse_match_ok() {
    let rules = rules!{};
    let status = Status::init("a f0ghi", &rules);

    let match_rules = MatchRules::new().with_chars("54321ed_cba");
    let (status, _) = parse_match(status, &match_rules).ok().unwrap();
    assert_eq!(status.pos.col, 1);
    assert_eq!(status.pos.n, 1);
    assert_eq!(status.pos.row, 0);

    let (status, _) = parse_dot(status).ok().unwrap();

    let match_rules = MatchRules::new().with_bound_chars(vec![('f', 'g'), ('h', 'j')]);
    let (status, _) = parse_match(status, &match_rules).ok().unwrap();
    assert_eq!(status.pos.col, 3);
    assert_eq!(status.pos.n, 3);
    assert_eq!(status.pos.row, 0);

    assert!(parse_match(status, &match_rules).is_err());
}

#[test]
fn test_parse_match_err() {
    let rules = rules!{};
    let status = Status::init("a9", &rules);

    let match_rules = MatchRules::new().with_chars("ed_cba");
    let (status, _) = parse_match(status, &match_rules).ok().unwrap();
    assert_eq!(status.pos.col, 1);
    assert_eq!(status.pos.n, 1);
    assert_eq!(status.pos.row, 0);

    let match_rules = MatchRules::new().with_bound_chars(vec![('a', 'z'), ('0', '8')]);
    assert!(parse_match(status, &match_rules).is_err());
}

#[test]
fn test_parse_match_eof_ok() {
    let rules = rules!{};
    let status = Status::init("a", &rules);

    let match_rules = MatchRules::new().with_bound_chars(vec![('a', 'z'), ('0', '9')]);
    let (status, _) = parse_match(status, &match_rules).ok().unwrap();

    assert!(parse_eof(status).is_ok());
}

#[test]
fn test_parse_match_eof_error() {
    let rules = rules!{};
    let status = Status::init("ab", &rules);

    let match_rules = MatchRules::new().with_bound_chars(vec![('a', 'z'), ('0', '9')]);
    let (status, _) = parse_match(status, &match_rules).ok().unwrap();

    assert!(parse_eof(status).is_err());
}
