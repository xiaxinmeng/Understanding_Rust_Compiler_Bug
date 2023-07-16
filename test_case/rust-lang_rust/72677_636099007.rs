
error: `_x @` is not allowed in a tuple
 --> src/main.rs:4:14
  |
4 |         (_a, ref mut _x @ ..) => {}
  |              ^^^^^^^^^^^^^^^ this is only allowed in slice patterns
  |
  = help: remove this and bind each tuple field independently
help: if you don't need to use the contents of _x, discard the tuple's remaining fields
  |
4 |         (_a, ..) => {}
  |              ^^

error: aborting due to previous error
