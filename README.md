# DynParser

A small and simple Dynamic Parser

NOT COMPLETED. WORKING ON!!!

## Usage

Add to `cargo.toml`

```toml
[dependencies]
pending...
```

Watch examples below

## Modifications

0.1.0 Not completed

## TODO/DONE

### TODO

- mostly all

### DONE

---

## Input

### Grammar

#### Rule elements enumeration

Examples below

| token    | Description                                            |
| -------- | ------------------------------------------------------ |
| `=`      | On left, symbol, on right expresion defining symbol    |
| `symbol` | On right, it's an string without quotes                |
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

Referencing symbols

Symbol

```peg
main = hi
hi   = "Hello world"
```

Or conditions `/`

```peg
[source, peg]
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

### Text

Hey, I'm a text parser, I need a text to parse ;-P

If you want to parse text indentation sensitive, I recomend you the lib
[indentation_flattener](https://github.com/jleahred/indentation_flattener)

The only consideration about the text to parse, is the type. It's not a generic String, it has to be
a more concrete `Text2Parse`

```rust
pending...
```

## Output

### AST

Well, you can see code on... let say `ast.rs` (not surprising)

```rust
pending...

#[derive(Debug)]
pub struct Node {
    pub kind: K,
    pub val: V,
    pub nodes: Box<Vec<Node>>,
}
```

An ast, is a `root` node, witch have subnodes and recursivily, we got a tree.

Next are the kind types of a node.

```rust
pending...

pub enum K {
    Root,
    EAnd,
    ENot,
    ERepeat,
    ALit,
    AMatch,
    ADot,
    ASymbref,
    AEof,
}
```

The ones who start with `Exxx` are `Expressions` nodes. Starting with `Axxxx` we have the atom
nodes.

With method `get_pruned` we can remove non interesting nodes.

## API

It works with concrete types vs general types (reducing use of types like String, u32 or usize)

Constants

```rust
## pending
```

Concrete types

```rust
## pending
```

Functions to call

```rust
## pending
```

Error type

```rust
## pending
```

Thats all

Look into lib.rs

## Examples

You can look into tests.rs.

Simple example

```rust
## pending...
```

Complex example

```rust
## pending...
```

More examples on tests.rs

## Hacking the code

The grammar is a set of rules

```rust
pending...
type Rules = HashMap<Symbol, Expression>;
```

A Symbol is just a String

```rust
pending...
#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct Symbol(pub String);
```

An expression can be one of...

```rust
pending...
#[derive(Debug)]
pub enum Expression {
    Simple(Atom),
    Or(MultiExpr),
    And(MultiExpr),
    Not(Box<Expression>),
    Repeat(Box<Expression>, NRep, Option<NRep>), // min max
}
```

An atom can be just...

```rust
pending...
#[derive(Debug, PartialEq)]
pub enum Atom {
    Literal(String),
    Match(String, Vec<(char, char)>),
    Dot,
    Symbol(String),
    Nothing,
}
```

### A grammar for the grammar

A grammar to define the grammar to be parsed by de parser. ;-P

I will define the grammar using the this parser grammar definition rules.

```peg
grammar = rule+
rule    = symbol  _  "="  expr
_       = " "*
```

Here we relax the verification to keep the grammar as simple as possible.

A grammar is a set of rules, where a rule is the symbol name followed by an expression.

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

And not to:

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


_               =  (" "
                /   "\n"
                /   comment)*

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
    let parsed = parse(&text2parse(r#"h= asdf (hi"#),
                       &symbol("grammar"),
                       &grammar());

    match parsed {
        Err(err) => println!("error... {} ___________", err),
        Ok(res) => println!("Ok... {:?} ___________", res),
    };
```

At the beggining it finished with no errors, but not consuming the hole input.
Wich is an error.

Showing an error informing that we didn't consume full input, is not the best.

```peg
pending...
error... in pos: r:1, c:9, n:8 >h= asdf < -> unexpected >(hi<
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

To improve this message, I added deep_error on Status for these situations

```rust
pending...
#[derive(Debug, Clone)]
pub(crate) struct Status<'a> {
    text2parse: &'a str,
    it_parsing: Chars<'a>,
    pos: Possition,
    deep_error: Option<Error>,
}
```

Now the new error message in these circunstances will be:

```
pending...
## error... in pos: r:1, c:7, n:6 >h=a (b< -> s.and_expr > s.compl_expr > s.simpl_par > s.parenth_expr > lit. expected ")", got ""
```

Much better!!!

In any case, to improve error messages, would be interesting to have something like:

```peg
pending...
parenth_expr    = "(" * expr _ ")"
                / "(" _ expr \_ -> error("mismatch parenthesis")
```

The or brunch will execute if there is no closing parenthesis and we can
write an specific error message.
