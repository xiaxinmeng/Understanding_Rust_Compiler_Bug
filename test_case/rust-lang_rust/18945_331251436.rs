
error: chained comparison operators require parentheses
 --> src/main.rs:2:9
  |
2 |       Some<7>
  |  _________^
3 | | }
  | |_^
  |
  = help: use `::<...>` instead of `<...>` if you meant to specify type arguments

error: expected expression, found `}`
 --> src/main.rs:3:1
  |
3 | }
  | ^

error[E0308]: mismatched types
 --> src/main.rs:1:27
  |
1 |   fn fun() -> Option<usize> {
  |  ___________________________^
2 | |     Some<7>
3 | | }
  | |_^ expected enum `std::option::Option`, found ()
  |
  = note: expected type `std::option::Option<usize>`
             found type `()`
