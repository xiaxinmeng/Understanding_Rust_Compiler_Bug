
error[E0308]: mismatched types
  --> src/main.rs:10:5
   |
9  | fn test0(foo: impl Trait) {
   |                           - possibly return type missing here?
10 |     foo - foo
   |     ^^^^^^^^^ expected `()`, found associated type
   |
   = note:    expected unit type `()`
           found associated type `<impl Trait as Sub>::Output`
   = help: consider constraining the associated type `<impl Trait as Sub>::Output` to `()`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
