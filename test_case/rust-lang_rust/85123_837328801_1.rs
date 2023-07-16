
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src/lib.rs:9:15
   |
9  |         match self {
   |               ^^^^
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined on the impl at 6:6...
  --> src/lib.rs:6:6
   |
6  | impl<'a> Foo<'a> {
   |      ^^
note: ...so that the types are compatible
  --> src/lib.rs:9:15
   |
9  |         match self {
   |               ^^^^
   = note: expected `&Foo<'_>`
              found `&Foo<'a>`
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the expression is assignable
  --> src/lib.rs:10:23
   |
10 |             Baz(x) => Baz(x.to_owned()),
   |                       ^^^^^^^^^^^^^^^^^
   = note: expected `Foo<'static>`
              found `Foo<'_>`
