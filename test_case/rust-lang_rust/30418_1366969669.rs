
error[E0409]: variable `i` is bound inconsistently across alternatives separated by `|`
 --> src/lib.rs:8:32
  |
8 |       opts::a(ref i) | opts::b(i) => {}
  |                   -            ^ bound in different ways
  |                   |
  |                   first binding

error[E0308]: mismatched types
 --> src/lib.rs:8:32
  |
7 |     match x {
  |           - this expression has type `opts`
8 |       opts::a(ref i) | opts::b(i) => {}
  |               -----            ^ expected `&isize`, found `isize`
  |               |
  |               first introduced with type `&isize` here
  |
  = note: in the same arm, a binding must have the same type in all alternatives
help: consider adding `ref`
  |
8 |       opts::a(ref i) | opts::b(ref i) => {}
  |                                +++
