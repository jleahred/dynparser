extern crate dynparser;
use dynparser::ast;

fn main() {
    let ast_before_compact: ast::Node = ast::Node::Rule((
        "root".to_string(),
        vec![ast::Node::Rule((
            "root".to_string(),
            vec![
                ast::Node::Val("hello".to_string()),
                ast::Node::Val(" ".to_string()),
                ast::Node::Val("world".to_string()),
            ],
        ))],
    ));

    let ast_after_compact = ast::Node::Rule((
        "root".to_string(),
        vec![ast::Node::Rule((
            "root".to_string(),
            vec![ast::Node::Val("hello world".to_string())],
        ))],
    ));

    assert!(ast_before_compact.compact() == ast_after_compact)
}

// fn main() {
//     let rules = rules_from_peg(
//         r#"
// main    =   [abA-Z]
//         "#,
//     );

//     //println!("{:#?}", rules);

//     // let result = parse("abcd", &rules);
//     // match result {
//     //     Ok(ast) => println!("{:#?}", ast),
//     //     Err(e) => println!("Error: {:?}", e),
//     // };
// }
