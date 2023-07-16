
error[E0409]: variable `i` is bound in inconsistent ways within the same match arm
 --> src/main.rs:8:32
  |
8 |       opts::a(ref i) | opts::b(i) => {}
  |                   -            ^ bound in different ways
  |                   |
  |                   first binding

error[E0308]: mismatched types
 --> src/main.rs:8:32
  |
8 |       opts::a(ref i) | opts::b(i) => {}
  |                                ^ expected &isize, found isize
  |
  = note: expected type `&isize`
             found type `isize`
