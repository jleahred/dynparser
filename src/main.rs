// extern crate dynparser;
// use dynparser::ast::{self, get_node_val};
// fn main() {
//     let ast: ast::Node = ast::Node::Val("hello".to_string());

//     let val = get_node_val(&ast).unwrap();

//     assert!(val == "hello");
// }

// extern crate dynparser;
// use dynparser::{parse, rules_from_peg};

// fn main() {
//     let rules = rules_from_peg(
//         r#"

// main            =   letter letter_or_num+

// letter          =   [a-zA-Z]

// letter_or_num   =   letter
//                 /   number

// number          =   [0-9]

//         "#,
//     ).map_err(|e| {
//         println!("{}", e);
//         panic!("FAIL");
//     })
//         .unwrap();

//     println!("{:#?}", rules);

//     let result = parse("a2AA456bzJ88", &rules);
//     match result {
//         Ok(ast) => println!("{:#?}", ast),
//         Err(e) => println!("Error: {:?}", e),
//     };
// }

// extern crate dynparser;
// use dynparser::{parse, rules_from_peg};

// fn main() {
//     let rules = rules_from_peg(
//         r#"

//     main    =   "a" ( "bc" "c"
//                     / "bcdd"
//                     / b_and_c  d_or_z
//                     )

//     b_and_c =   "b" "c"
//     d_or_z  =   "d" / "z"

//         "#,
//     ).unwrap();

//     assert!(parse("abcz", &rules).is_ok());
//     // assert!(parse("abcdd", &rules).is_ok());
//     // assert!(parse("abcc", &rules).is_ok());
//     // assert!(parse("bczd", &rules).is_err());
// }

extern crate dynparser;
use dynparser::{parse, rules_from_peg};

fn main() {
    let rules = rules_from_peg(
        r#"

    main    =   "hello" " "  "world"   
            /   "hola"
            /   "hola"  " "  "mundo"
                
        "#,
    ).map_err(|e| {
        println!("{}", e);
        panic!("FAIL");
    })
        .unwrap();

    println!("{:#?}", rules);

    let result = parse("hola mundo", &rules);
    match result {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => println!("Error: {:?}", e),
    };

    // assert!(parse("ab", &rules).is_ok());
    // assert!(parse("c", &rules).is_ok());
    // assert!(parse("de", &rules).is_ok());
    // assert!(parse("abc", &rules).is_err());
}
