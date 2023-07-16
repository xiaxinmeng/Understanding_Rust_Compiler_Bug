
error[E0391]: cycle detected when computing predicates of `Foo`
  --> src\main.rs:5:1
   |
5  | struct Foo {
   | ^^^^^^^^^^
   |
note: ...which requires computing predicates of `Foo`...
  --> src\main.rs:5:1
   |
5  | struct Foo {
   | ^^^^^^^^^^
note: ...which requires computing inferred outlives predicates of `Foo`...
  --> src\main.rs:5:1
   |
5  | struct Foo {
   | ^^^^^^^^^^
   = note: ...which requires computing the inferred outlives predicates for items in this crate...
note: ...which requires computing type of `Foo::bar`...
  --> src\main.rs:6:5
   |
6  |     bar: Self::Bar,
   |     ^^^^^^^^^^^^^^
note: ...which requires computing normalized predicates of `Foo`...
  --> src\main.rs:5:1
   |
5  | struct Foo {
   | ^^^^^^^^^^
   = note: ...which again requires computing predicates of `Foo`, completing the cycle
note: cycle used when collecting item types in top-level module
