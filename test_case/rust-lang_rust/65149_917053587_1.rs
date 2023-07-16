
error: generic parameters may not be used in const operations
 --> src/lib.rs:5:40
  |
5 | fn make_array<B : Bounded>() -> [bool; B::COUNT] {
  |                                        ^^^^^^^^ cannot perform const operation using `B`
  |
  = note: type parameters may not be used in const expressions

error: constant expression depends on a generic parameter
 --> src/lib.rs:6:13
  |
6 |     [false; B::COUNT]
  |             ^^^^^^^^
  |
  = note: this may fail depending on what value the parameter takes
