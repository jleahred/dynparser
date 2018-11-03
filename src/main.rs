extern crate dynparser;
use dynparser::peg::peg2code;

fn main() {
    peg2code::print_rules2parse_peg();
}

//  --------------------------
//  modules

// extern crate dynparser;
// use dynparser::{parse, rules_from_peg};

// fn main() {
//     let rules = rules_from_peg(
//         r#"

//     main    =   as  /  b.bs  /  abc.abs  /  abc.c.cs

//     as  =   'a'+

//     b {
//         bs  =  'b'+
//     }

//     abc {
//         abs =  .as  /  .b.bs

//         c {
//             cs  =   'c'+
//         }
//     }

//         "#,
//     ).unwrap();

//     assert!(parse("abcz", &rules).is_ok());
// }

// //  --------------------------------------------------------------------------

// extern crate dynparser;
// use dynparser::{peg, rules_from_peg};

// fn main() {
//     let rules = rules_from_peg(
//         r#"

// main            =   grammar

// grammar         =   rule+

// rule            =   _  symbol  _  "="  _  expr  _eol _

// expr            =   or

// or              =   and         ( _ "/" _  or  )*

// and             =   rep_or_neg  ( _1 _ !(symbol _ "=") and )*

// rep_or_neg      =   atom_or_par ("*" / "+" / "?")?
//                 /   "!" atom_or_par

// atom_or_par     =   (atom / parenth)

// parenth         =   "("  _  expr  _  ")"

// atom            =   literal
//                 /   match
//                 /   dot
//                 /   symbol

// literal         =   _"  (  "\\" .
//                         /  !_" .
//                         )*  _"
// _"              =   "\""

// symbol          =   [_'a-zA-Z0-9] [_'"a-zA-Z0-9]*

// eol             =   ("\r\n"  /  "\n"  /  "\r")
// _eol            =   " "*  eol

// match           =   "["
//                         (
//                             (mchars  mbetween*)
//                             / mbetween+
//                         )
//                     "]"

// mchars          =   (!"]" !(. "-") .)+
// mbetween        =   (.  "-"  .)

// dot             =   "."

// _               =   (  " "
//                         /   eol
//                     )*

// _1              =   (" " / eol)

// "#,
//     ).map_err(|e| {
//         println!("{}", e);
//         panic!("FAIL");
//     }).unwrap();

//     println!("{}", peg::gcode::rust_from_rules(&rules))
// }

//  --------------------------------------------------------------------------

// extern crate dynparser;
// use dynparser::{parse, rules_from_peg};

// fn main() {
//     let rules = rules_from_peg(
//         r#"
//          main    =   '('  main  ( ')'  /  error("unbalanced parenthesys") )
//                  /   'hello'
//         "#,
//     ).map_err(|e| {
//         println!("{}", e);
//         panic!("FAIL");
//     }).unwrap();

//     println!("{:#?}", rules);

//     let result = parse("a2AA456bzJ88", &rules);
//     match result {
//         Ok(ast) => println!("{:#?}", ast),
//         Err(e) => println!("Error: {:?}", e),
//     };
// }

//  --------------------------
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

//  --------------------

// extern crate dynparser;
// use dynparser::peg;
// use dynparser::{parse, rules_from_peg};

// fn main() {
//     let peg_rules = r#"

// main            =   (aaa / "bbb")*  zzz
// aaa             =   "aaa"
// zzz             =   "zzz"

//         "#;
//     let rules = rules_from_peg(peg_rules)
//         .map_err(|e| {
//             println!("{}", e);
//             panic!("FAIL");
//         }).unwrap();

//     // println!("{:#?}", rules);

//     // let result = parse("a2Z", &rules);
//     // match result {
//     //     Ok(ast) => {
//     //         println!("{:#?}", ast.compact().flatten());
//     //     }
//     //     Err(e) => println!("Error: {:?}", e),
//     // };
//     //    println!("{:#?}", rules_from_peg(&peg_rules))
//     println!("{}", peg::gcode::rust_from_rules(&rules));

//     let rules2 = peg::rules_from_peg2(peg_rules)
//         .map_err(|e| {
//             println!("{}", e);
//             panic!("FAIL");
//         }).unwrap();
//     println!("{}", peg::gcode::rust_from_rules(&rules2));
// }

// //  --------------------------------------------------------------------------

// #[macro_use]
// extern crate dynparser;
// use dynparser::{ast, idata, parse};

// fn main() {
//     let rules = rules!(
//         "rep_or_neg" => or!(and!(ref_rule!("atom_or_par"), rep!(or!(lit!("*"), lit!("+"), lit!("?")), 0, 1)), and!(lit!("!"), ref_rule!("atom_or_par")))
//        , "literal" => and!(ref_rule!("_\""), rep!(or!(and!(lit!("\\"), dot!()), and!(not!(ref_rule!("_\"")), dot!())), 0), ref_rule!("_\""))
//        , "eol" => or!(lit!("\r\n"), lit!("\n"), lit!("\r"))
//        , "mchars" => rep!(and!(not!(lit!("]")), not!(and!(dot!(), lit!("-"))), dot!()), 1)
//        , "mbetween" => and!(dot!(), lit!("-"), dot!())
//        , "atom_or_par" => or!(ref_rule!("atom"), ref_rule!("parenth"))
//        , "dot" => lit!(".")
//        , "or" => and!(ref_rule!("and"), rep!(and!(ref_rule!("_"), lit!("/"), ref_rule!("_"), ref_rule!("or")), 0))
//        , "_eol" => and!(rep!(lit!(" "), 0), ref_rule!("eol"))
//        , "rule" => and!(ref_rule!("_"), ref_rule!("symbol"), ref_rule!("_"), lit!("="), ref_rule!("_"), ref_rule!("expr"), ref_rule!("_eol"), ref_rule!("_"))
//        , "symbol" => and!(ematch!(chlist "_'"  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), rep!(ematch!(chlist "_'\""  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), 0))
//        , "main" => ref_rule!("grammar")
//        , "match" => and!(lit!("["), or!(and!(rep!(ref_rule!("mchars"), 1), rep!(ref_rule!("mbetween"), 0)), rep!(ref_rule!("mbetween"), 1)), lit!("]"))
//        , "grammar" => rep!(ref_rule!("rule"), 1)
//        , "and" => and!(ref_rule!("rep_or_neg"), rep!(and!(ref_rule!("_1"), ref_rule!("_"), not!(and!(ref_rule!("symbol"), ref_rule!("_"), lit!("="))), ref_rule!("and")), 0))
//        , "_" => rep!(or!(lit!(" "), ref_rule!("eol")), 0)
//        , "parenth" => and!(lit!("("), ref_rule!("_"), ref_rule!("expr"), ref_rule!("_"), lit!(")"))
//        , "expr" => ref_rule!("or")
//        , "_\"" => lit!("\"")
//        , "atom" => or!(ref_rule!("literal"), ref_rule!("match"), ref_rule!("dot"), ref_rule!("symbol"))
//        , "_1" => or!(lit!(" "), ref_rule!("eol"))
//     );

//     let parsed = parse(
//         r#"

// main            =   "Hello world"

// "#,
//         &rules,
//     ).map_err(|e| {
//         println!("{:?}", e);
//         panic!("FAIL");
//     }).unwrap();

//     println!(
//         "{:#?}",
//         parsed
//             .prune(&["_", "_eol"])
//             .compact()
//             .flatten()
//             .iter()
//             .filter(|n| {
//                 let remove = [
//                     "main",
//                     "grammar",
//                     "atom_or_par",
//                     "atom_or_par",
//                     "rep_or_neg",
//                 ];
//                 use idata::IVec;
//                 let remove = remove.iter().fold(vec![], |acc, item| {
//                     acc.ipush(format!("begin.{}", item))
//                         .ipush(format!("end.{}", item))
//                 });
//                 match n {
//                     ast::Node::Rule((name, _)) => !remove.contains(name),
//                     _ => true,
//                 }
//             }).collect::<Vec<_>>()
//     );
// }
