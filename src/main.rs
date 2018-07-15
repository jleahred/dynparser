extern crate dynparser;
use dynparser::{parse, rules_from_peg};

fn main() {
    let rules = rules_from_peg(
        r#"
main    =   [abA-Z]
        "#,
    );

    //println!("{:#?}", rules);

    // let result = parse("abcd", &rules);
    // match result {
    //     Ok(ast) => println!("{:#?}", ast),
    //     Err(e) => println!("Error: {:?}", e),
    // };
}
