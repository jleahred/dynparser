#[macro_use]
extern crate dynparser;
use dynparser::parse;

fn main() {
    let rules = rules!{
       "main"   =>  and!{
                        lit!("a"),
                        or!(
                            and!(lit!("bc"), lit!("c")),
                            lit!("bcdd"),
                            and!(
                                rule!("b_and_c"),
                                rule!("d_or_z")
                            )
                        )
                    },
        "b_and_c"  => and!(lit!("b"), lit!("c")),
        "d_or_z"  => or!(lit!("d"), lit!("z"))
    };

    let result = parse("abcd", &rules);
    match result {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => println!("Error: {:?}", e),
    };
}
