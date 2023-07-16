
error: expected one of `(` or `<`, found `{`
 --> src/main.rs:7:9
  |
7 |     Foo { text: "".to_string() }
  |         ^ expected one of `(` or `<` here

error[E0308]: mismatched types
 --> src/main.rs:5:23
  |
5 |   pub fn parse() -> Foo {
  |  _______________________^
6 | |     fn
7 | |     Foo { text: "".to_string() }
8 | | }
  | |_^ expected struct `Foo`, found ()
  |
  = note: expected type `Foo`
             found type `()`

error: aborting due to 2 previous errors
