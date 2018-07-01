#[macro_use]
extern crate dynparser;
use dynparser::parse;

fn main() {
    let rules = rules!{
       "main"   =>  and!{
                        lit!("a"),
                        or!(
                            and!(lit!("bcc")),
                            and!(
                                lit!("bc"),
                                lit!("e")
                            ),
                            lit!("bcaa")
                        )
                    }
    };

    let result = parse("abcd", &rules);
    match result {
        Ok(_) => println!("OK"),
        Err(e) => println!("Error: {:?}", e),
    };
}
