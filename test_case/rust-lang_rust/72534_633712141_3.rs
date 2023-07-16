
cargo +stage1 build
   Compiling binding2 v0.1.0 (/Users/chris/Desktop/tests/rustlang-tests/binding2)
error: `..` patterns are not allowed here
 --> src/main.rs:5:19
  |
5 |         (_a, _x @ ..) => {}
  |                   ^^
  |
  = note: only allowed in tuple, tuple struct, and slice patterns

error[E0308]: mismatched types
 --> src/main.rs:5:9
  |
3 |     match x {
  |           - this expression has type `({integer}, {integer}, {integer})`
4 |         (_a, 1, 1) => {}
5 |         (_a, _x @ ..) => {}
  |         ^^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected tuple `({integer}, {integer}, {integer})`
             found tuple `(_, _)`

error: aborting due to 2 previous errors
