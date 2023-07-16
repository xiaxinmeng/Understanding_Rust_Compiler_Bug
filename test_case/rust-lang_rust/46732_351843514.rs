
error: missing `struct` for struct definition
  --> src/main.rs:11:8
   |
11 |     pub Foo { text }    // NB: Syntax error here
   |        ^
help: add `struct` here to parse `Foo` as a public struct
   |
11 |     pub struct Foo { text }    // NB: Syntax error here
   |         ^^^^^^

error[E0308]: mismatched types
  --> src/main.rs:7:23
   |
7  |   pub fn parse() -> Foo {
   |  _______________________^
8  | |     let args: Vec<String> = env::args().collect();
9  | |     let text = args[1].clone();
10 | |     
11 | |     pub Foo { text }    // NB: Syntax error here
12 | | }
   | |_^ expected struct `Foo`, found ()
   |
   = note: expected type `Foo`
              found type `()`
