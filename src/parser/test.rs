//-----------------------------------------------------------------------
//
//  mod parser  TEST
//
//-----------------------------------------------------------------------

use parser::{expression::parse, Status};

#[test]
fn test_parse_lit() {
    let status_init = Status::init("aaaaaaaaaaaaaaaa");

    let expr = lit!("aaa");

    let result = parse(status_init, &expr).ok().unwrap();
    assert!(result.0.pos.col == 3);
    assert!(result.0.pos.n == 3);
    assert!(result.0.pos.row == 0);
}

#[test]
fn test_parse_and_ok() {
    let status_init = Status::init("aabbcc");

    let expr = and![lit!("aa"), and![lit!("bb"), lit!("cc")]];

    let result = parse(status_init, &expr).ok().unwrap();
    assert_eq!(result.0.pos.col, 6);
    assert_eq!(result.0.pos.n, 6);
    assert_eq!(result.0.pos.row, 0);
}

#[test]
fn test_parse_or_ok() {
    let status_init = Status::init("aabb");

    let expr = or![lit!("bb"), and![lit!("aa"), lit!("bb")]];

    let result = parse(status_init, &expr).ok().unwrap();
    assert_eq!(result.0.pos.col, 4);
    assert_eq!(result.0.pos.n, 4);
    assert_eq!(result.0.pos.row, 0);
}

#[test]
fn test_parse_not_ok() {
    let status_init = Status::init("aa");

    let expr = not!(lit!("bb"));

    let result = parse(status_init, &expr).ok().unwrap();
    assert_eq!(result.0.pos.col, 0);
    assert_eq!(result.0.pos.n, 0);
    assert_eq!(result.0.pos.row, 0);
}

#[test]
fn test_parse_repeat_ok() {
    {
        let status_init = Status::init("aaaaaa");

        let expr = rep![lit!("aa"), 3];

        let result = parse(status_init, &expr).ok().unwrap();
        assert_eq!(result.0.pos.col, 6);
        assert_eq!(result.0.pos.n, 6);
        assert_eq!(result.0.pos.row, 0);
    }

    {
        let status_init = Status::init("aaaaaa");

        let expr = rep![lit!("aa"), 0, 3];

        let result = parse(status_init, &expr).ok().unwrap();
        assert_eq!(result.0.pos.col, 6);
        assert_eq!(result.0.pos.n, 6);
        assert_eq!(result.0.pos.row, 0);
    }
}
