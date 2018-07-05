# DynParser

A small and simple Dynamic Parser

It's not a compile time parser.

You can create and modify the grammar on runtime.

## Usage

Add to `cargo.toml`

```toml
[dependencies]
dynparser = "0.1.0"
```

Watch examples below

## Modifications

0.1.0 First version

## TODO

- Upload to crates.io
  - update usage
  - update links to doc
- more examples in doc
- tail recursion parsing rule
- macro for eof

## Input

You will configure a set of rules to parse.

The rule is composed of a name followed by an arrow and an expression to be parsed.

A basic example

Lets create the next grammar:

    main = "a" ( "bc" "c"
               / "bcdd
               / b_and_c  d_or_z
               )

    b_and_c = "b" "c"
    d_or_z  = "d" / "z"

You can create this grammar and parse the string "abcd" with:

```rust
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
```

The exit will be next AST

    Rule(
        (
            "main",
            [
                Val(
                    "a"
                ),
                Rule(
                    (
                        "b_and_c",
                        [
                            Val(
                                "b"
                            ),
                            Val(
                                "c"
                            )
                        ]
                    )
                ),
                Rule(
                    (
                        "d_or_z",
                        [
                            Val(
                                "d"
                            )
                        ]
                    )
                )
            ]
        )
    )

The AST type is:

    pub enum Node {
        Val(String),
        Rule((String, Vec<Node>)),
        EOF,
    }

More information in doc (link pending)
