# DynParser

A small and simple Dynamic Parser

It's not a compile time parser.

You can create and modify the grammar on runtime.

In order to create the grammar, you can build a map, or you can use
macros to use a better syntax.

It's also possible to pass a PEG string to create the grammar rules.

In order to use a PEG string, we need to parse it. We could parse with...
Hey!!! myself could parse it. OK, OK

As we can generate the rules with a PEG string, the parser for the PEG
string, could be written in a PEG string.

A parser parsing it's own parser :-P

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

- Create rules from PEG
- add errors to grammar
- Upload to crates.io
  - update usage
  - update links to doc
- more examples in doc
- tail recursion parsing rule
- macro for eof

## Basic example

You will configure a set of rules to parse.

The rule is composed of a name followed by an arrow and an expression to be parsed.

A basic example

Lets create the next grammar:

    main    =   "a" ( "bc" "c"
                    / "bcdd
                    / b_and_c  d_or_z
                    )

    b_and_c =   "b" "c"
    d_or_z  =   "d" / "z"

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

```rust
pub enum Node {
    Val(String),
    Rule((String, Vec<Node>)),
    EOF,
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
                        rule!("rule2")
                    }
    };

    let rules = rules.add("rule2", lit!("bcd"));

    assert!(parse("aabcd", &rules).is_ok())
}
```

Of course, you could need to add (or merge) several rules at once

And ofcourse, you can add several rules at once

```rust
#[macro_use]  extern crate dynparser;
use dynparser::parse;
fn main() {
    let r = rules!{
       "main"   =>  and!{
                        rep!(lit!("a"), 1, 5),
                        rule!("rule2")
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

Recursion to match parentheses

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

and_expr        =   compl_expr   (" "  and_expr)*

compl_expr      =   "!" simpl_par
                /   simpl_par ("*" / "+")

simpl_par       =   (simple / parenth_expr)


parenth_expr    =   "("  expr ")"
simple          =   atom
```

Descendant definition

| expr       | Description                                                                                                |
| ---------- | ---------------------------------------------------------------------------------------------------------- |
| simpl_par  | It's an atom or a parenthesis experssion                                                                   |
| compl_expr | Complete expresssion. It's a full subtree expression It can have negation or (zero or more or one or more) |
| and_expr   | Sequence of expressions separated by space                                                                 |
| or_expr    | Sequence of expression separated by "/"                                                                    |

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
grammar         =   rule+

rule            =   symbol  _  "="  _   expr  (_eol / eof)  _

expr            =   or_expr

or_expr         =   and_expr    (_ "/"  _  or_expr)*

and_expr        =   compl_expr  (  " "  _  and_expr)*

compl_expr      =   simpl_par ("*" / "+")?
                /   "!" simpl_par

simpl_par       =   (simple / parenth_expr)


parenth_expr    =   "("  _  expr  _  ")"
simple          =   atom



atom            =   literal
                /   match
                /   dot
                /   symbol

literal         =   "\u{34}"  (!"\u{34}" .)*  "\u{34}"
match           =   "["  ( (.  "-"  .)  /  (!"]") )+   "]"
dot             =   "."
symbol          =   [a-zA-Z0-9_]+


_               =  (  " "
                      /   "\n"
                      /   comment
                   )*

_eol            = " "*  "\n"
                / comment

comment         =  "//" (!"/n" .)* "/n"
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
