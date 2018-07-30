# DynParser

A small and simple Dynamic Parser

It's not a compile time parser.

You can create and modify the grammar on runtime.

In order to create the grammar, you can build a map, or you can use
macros to use a better syntax. But, the easier way, is to use a `peg` grammar.

More info about the `peg` syntax bellow.

You can also generate `rust` code from rules generated from `peg`.

In fact, in order to use a `peg` grammar, you have to parse it.
How to parse a `peg` grammar? Well, this is a parser, therefore...

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

- Move to an isolated module IVector
- insert and test EOF

```
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
```

- Create rules from PEG
  - document rules from peg
  - calculator parser example
- generate code from rules
- add errors to grammar
- Upload to crates.io
  - update usage
  - update links to doc
- more examples in doc
- tail recursion parsing rule
- macro for eof
- rules path on errors configurable (due to performance)
  - check. is it interesting to detail branches on or?

## Basic example

You will configure a set of rules to parse.

The rule is composed of a name followed by an arrow and an expression to be parsed.

A basic example

Lets create the next grammar:

    main    =   "a" ( "bc" "c"
                    / "bcdd"
                    / b_and_c  d_or_z
                    )

    b_and_c =   "b" "c"
    d_or_z  =   "d" / "z"

### Just from peg

```rust
extern crate dynparser;
use dynparser::{parse, rules_from_peg};

fn main() {
    let rules = rules_from_peg(
        r#"

    main    =   "a" ( "bc" "c"
                    / "bcdd"
                    / b_and_c  d_or_z
                    )

    b_and_c =   "b" "c"
    d_or_z  =   "d" / "z"

        "#,
    ).unwrap();

    assert!(parse("abcz", &rules).is_ok());
    assert!(parse("abcdd", &rules).is_ok());
    assert!(parse("abcc", &rules).is_ok());
    assert!(parse("bczd", &rules).is_err());
}
```

The exit will be the next AST

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

```rust
pub enum Node {
    Val(String),
    Rule((String, Vec<Node>)),
    EOF,
}
```

This is a dynamic parser, you can add rules at execution time.

pending: example

### Generating the rules by hand with macros

You can create this grammar and parse the string "abcd" with macros like:

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
                                ref_rule!("b_and_c"),
                                ref_rule!("d_or_z")
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

Adding a rule on execution time:

```rust
#[macro_use]  extern crate dynparser;
use dynparser::parse;
fn main() {
    let rules = rules!{
       "main"   =>  and!{
                        rep!(lit!("a"), 1, 5),
                        ref_rule!("rule2")
                    }
    };

    let rules = rules.add("rule2", lit!("bcd"));

    assert!(parse("aabcd", &rules).is_ok())
}
```

Of course, you could need to add (or merge) several rules at once

And of course, you can add several rules at once

```rust
#[macro_use]  extern crate dynparser;
use dynparser::parse;
fn main() {
    let r = rules!{
       "main"   =>  and!{
                        rep!(lit!("a"), 1, 5),
                        ref_rule!("rule2")
                    }
    };
    let r = r.merge(rules!{"rule2" => lit!("bcd")});
    assert!(parse("aabcd", &r).is_ok())
}
```

`merge` takes the ownership of both set of rules and returns a "new" (in fact modified)
set of rules. This helps to reduce mutability

"main" rule is the entry point.

More information in doc (link pending)

## Example 2

Lets create the next grammar:

```
    main            =   letter letter_or_num+

    letter          =   [a-zA-Z]

    letter_or_num   =   letter
                    /   number

    number          =   [0-9]
```

This grammar will accept a letter, followed from one or more letters or
numbers

### Just from peg

Quiet direct...

```rust
    extern crate dynparser;
    use dynparser::{parse, rules_from_peg};

    fn main() {
        let rules = rules_from_peg(
            r#"

    main            =   letter letter_or_num+

    letter          =   [a-zA-Z]

    letter_or_num   =   letter
                    /   number

    number          =   [0-9]

            "#,
        ).unwrap();

        assert!(parse("a2AA456bzJ88", &rules).is_ok());
    }
```

If you want to print more information...

```rust
    extern crate dynparser;
    use dynparser::{parse, rules_from_peg};

    fn main() {
        let rules = rules_from_peg(
            r#"

    main            =   letter letter_or_num+

    letter          =   [a-zA-Z]

    letter_or_num   =   letter
                    /   number

    number          =   [0-9]

            "#,
        ).map_err(|e| {
            println!("{}", e);
            panic!("FAIL");
        })
            .unwrap();

        println!("{:#?}", rules);

        let result = parse("a2AA456bzJ88", &rules);
        match result {
            Ok(ast) => println!("{:#?}", ast),
            Err(e) => println!("Error: {:?}", e),
        };
    }
```

Just it (remember, more information about the peg grammar bellow)

## PEG

### Rule elements enumeration

Examples below

| token    | Description                                            |
| -------- | ------------------------------------------------------ |
| `=`      | On left, symbol, on right expresion defining symbol    |
| `symbol` | It's an string without quotes                          |
| `.`      | Any char                                               |
| `"..."`  | Literal delimited by quotes                            |
| `space`  | Separate tokens and Rule concatenation (and operation) |
| `/`      | Or operation                                           |
| `(...)`  | A expression composed of sub expresions                |
| `?`      | One optional                                           |
| `*`      | Repeat 0 or more                                       |
| `+`      | Repeat 1 or more                                       |
| `!`      | negate expression                                      |
| `[...]`  | Match chars. It's a list or ranges (or both)           |
| `->`     | pending...                                             |
| `:`      | pending...                                             |

Let's see by example

#### Rules by example

A simple literal string.

```peg
main = "Hello world"
```

Concatenation (and)

```peg
main = "Hello "  "world"
```

Referencing symbols (rule)

Symbol

```peg
main = hi
hi   = "Hello world"
```

Or `/`

```peg
main = "hello" / "hi"
```

Or multiline

```peg
main
    = "hello"
    / "hi"
    / "hola"
```

Or multiline 2

```peg
main = "hello"
     / "hi"
     / "hola"
```

Or disorganized

```peg
main = "hello"
     / "hi" / "hola"
```

Parenthesis

```peg
main = ("hello" / "hi")  " world"
```

Just multiline

Multiline1

```peg
main
    = ("hello" / "hi")  " world"
```

Multiline2

```peg
main
    = ("hello" / "hi")
    " world"
```

Multiline3

```peg
main = ("hello" / "hi")
     " world"
```

It is recomended to use or operator `/` on each new line and `=` on first line, like

Multiline organized

```peg
main = ("hello" / "hi")  " world"
     / "bye"
```

One optional

```peg
main = ("hello" / "hi")  " world"?
```

Repetitions

```peg
main         = one_or_more_a / zero_or_many_b
one_or_more  = "a"+
zero_or_many = "b"*
```

Negation will not move current possition

Next example will consume all chars till get an "a"

Negation

```peg
main = (!"a" .)* "a"
```

Consume till

```peg
comment = "//" (!"\n" .)*
        / "/*" (!"*/" .)* "*/"
```

Match a set of chars.
Chars can be defined by range.

```peg
number  = digit+ ("." digit+)?
digit   = [0-9]
a_or_b  = [ab]
id      = [_a-zA-Z][_a-zA-Z0-9]*

a_or_b_or_digit  = [ab0-9]
```

Simple recursion

one or more "a" recursive

```peg
as  = "a" as
    / "a"

//  simplified with `+`
ak = "a"+
```

Recursion to match parenthesis

Recursion match par

```peg
match_par = "(" match_par ")"
          / "(" ")"
```

Grammar bellow (on hacking the code)...

## Text

Hey, I'm a text parser, I need a text to parse ;-P

If you want to parse text indentation sensitive, I recomend you the lib
[indentation_flattener](https://github.com/jleahred/indentation_flattener)

```rust
pending...
```

## A grammar for the grammar

A grammar to define the grammar to be parsed by de parser. ;-P

I will define the grammar using the this parser grammar definition rules.

A grammar is a set of rules.

A rule, is a symbol followed by `=` and an expression

```peg
grammar = rule+
rule    = symbol "="  expr
```

Here we relax the verification to keep the grammar as simple as possible.
It's missing also the non significant spaces.

About the expression.

As you know, it's important to accept valid inputs, but also it's important to
build an AST with proper pritority.

Next grammar:

```peg
main    =  "A" "B"  /  "B" "C"
```

It's equivalent to:

```peg
main    =  ("A" "B")  /  ("B" "C")
```

But not to:

```peg
main    =  (("A" "B")  /  "B") "C"
```

To represent this priority, the expression rule has to be defined in a descendant priority way:

```peg
expr            =   or_expr

or_expr         =   and_expr     ("/"  or_expr)*

and_expr        =   simpl_expr   (" "  and_expr)*

simpl_expr      =   "!" atom_or_par
                /   simpl_par ("*" / "+")

atom_or_par     =   (atom / parenth_expr)


parenth_expr    =   "("  expr ")"
```

Descendant definition

| expr        | Description                                                                              |
| ----------- | ---------------------------------------------------------------------------------------- |
| atom_or_par | It's an atom or a parenthesis experssion                                                 |
| rep_or_neg  | It's not a composition of `and` or `or` expressions. It can have negation or repetitions |
| parenth     | It's an expressions with parenthesis                                                     |
| and         | Sequence of expressions separated by space                                               |
| or          | Sequence of expression separated by "/"                                                  |

Now, it's the `atom` turn:

```peg
atom    =   literal
        /   match
        /   dot
        /   symbol

literal =   "\""  (!"\"" .)*  "\""
match   =   "["  ((.  "-"  .)  /  (.))+   "]"
dot     =   "."
symbol  =   [a-zA-Z0-9_]+
```

Hey, what about comments?

What about non significative spaces and carry return?

It will be defined on "\_" symbol

```peg
main            =   grammar

grammar         =   rule+

rule            =   symbol  _  "="  _   expr  (_ / eof)

expr            =   or

or              =   and         ( _ "/" _  or  )*

and             =   rep_or_neg  (   " " _  and )*

rep_or_neg      =   atom_or_par ("*" / "+" / "?")?
                /   "!" atom_or_par

atom_or_par     =   (atom / parenth)


parenth         =   "("  _  expr  _  ")"



atom            =   literal
                /   match
                /   dot
                /   symbol

literal         =   _"  (!_" .)*  _"
_"              =   "\u{34}"

match           =   "["
                        (
                            (mchars+  mbetween*)
                            / mbetween+
                        )
                    "]"
mchars          =   (!"]" !(. "-") .)+
mbetween        =   (.  "-"  .)

dot             =   "."
symbol          =   [_'a-zA-Z0-9][_'"a-zA-Z0-9]+


_               =  (  " "
                      /   eol
                      /   comment
                   )*

eol             = ("\r\n"  \  "\n"  \  "\r")

comment         =  "//" (!eol .)* "/n"
                /  "/*" (!"*/" .)* "*/"
```

That's ok an works fine, but we can inprove error messages...

In order to improve error messages, would be interesting to modify the grammar.

Look this code:

```rust
pending...
```

At the beggining it finished with no errors, but not consuming the hole input.
Wich is an error.

Showing an error informing that we didn't consume full input, is not the best.

```
pending...
```

The reason is on

```peg
pending...
...
and_expr        =   compl_expr  (  " "  _  and_expr)*
...
```

Here, we said, "hey, try to look for a sequence, or not `*`"

And is not, then the parser say, I matched the rule, I have to continue verifying other
previus branches. But there are no previus partial applied brunchs.
Then the parser ends not consuming all the input.

To improve error messages, would be interesting to have something like:

```peg
pending...
parenth_expr    = "(" * expr _ ")"
                / "(" _ expr _ -> error("mismatch parenthesis")
```

The or brunch will be executed if there is no closing parenthesis and we can
write an specific error message.
