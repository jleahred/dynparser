// extern crate dynparser;
// use dynparser::ast::{self, get_node_val};
// fn main() {
//     let ast: ast::Node = ast::Node::Val("hello".to_string());

//     let val = get_node_val(&ast).unwrap();

//     assert!(val == "hello");
// }

extern crate dynparser;
use dynparser::{parse, rules_from_peg};

fn main() {
    let rules = rules_from_peg(
        r#"

main    =   [134a-jk-z]+

        "#,
    ).map_err(|e| {
        println!("{}", e);
        panic!("FAIL");
    })
        .unwrap();

    println!("{:#?}", rules);

    let result = parse("4133abcdefg", &rules);
    match result {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => println!("Error: {:?}", e),
    };
}
