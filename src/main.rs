
extern crate dynparser;
use dynparser::grammar::grammar;


use dynparser::{symbol, text2parse, parse};


fn main() {
    let parsed = parse(&text2parse(r#"main = ( hello )"#),
                       &symbol("grammar"),
                       &grammar());

    match parsed {
        Err(err) => println!("error... {} ___________", err),
        Ok(res) => println!("Ok... {:?} ___________", res),
    };
}
