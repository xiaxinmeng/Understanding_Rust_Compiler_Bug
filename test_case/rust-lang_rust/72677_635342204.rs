
error: `_x @` is not allowed in a tuple
 --> src/main.rs:4:14
  |
4 |         (_a, ref mut _x @ ..) => {}
  |              ^^^^^^^^^^^^^^^ is only allowed in a slice
  |
help: replace with `..` or use a different valid pattern
  |
4 |         (_a, ..) => {}
  |              ^^

error[E0308]: mismatched types
 --> src/main.rs:4:9
  |
3 |     match x {
  |           - this expression has type `({integer}, {integer}, {integer})`
4 |         (_a, ref mut _x @ ..) => {}
  |         ^^^^^^^^^^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 1 element
  |
  = note: expected tuple `({integer}, {integer}, {integer})`
             found tuple `(_,)`

error: aborting due to 2 previous errors
