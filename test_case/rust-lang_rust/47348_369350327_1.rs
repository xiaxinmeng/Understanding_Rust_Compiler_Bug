
   Compiling playground v0.0.1 (file:///playground)
error[E0308]: mismatched types
  --> src/main.rs:10:5
   |
9  | fn test0(foo: impl Trait)  {
   |                            - possibly return type missing here?
10 |     foo - foo
   |     ^^^^^^^^^ expected (), found associated type
   |
   = note: expected type `()`
              found type `<impl Trait as std::ops::Sub>::Output`
