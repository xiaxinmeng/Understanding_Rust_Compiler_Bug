
error[E0623]: lifetime mismatch
 --> src/lib.rs:5:9
  |
2 |     mut x: &(),
  |            ---
3 |     y: &(),
  |        --- these two types are declared with different lifetimes...
4 | ) {
5 |     x = y;
  |         ^ ...but data from `y` flows into `x` here
  |
  = note: each elided lifetime in input position becomes a distinct lifetime
help: consider introducing a named lifetime parameter
  |
1 ~ fn foo<'a>(
2 ~     mut x: &'a (),
3 ~     y: &'a (),
  |
