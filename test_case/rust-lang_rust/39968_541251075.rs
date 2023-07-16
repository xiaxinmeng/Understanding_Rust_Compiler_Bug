
error[E0308]: mismatched types
 --> src/lib.rs:3:20
  |
3 |         if false { break; }
  |                    ^^^^^
  |                    |
  |                    expected i32, found ()
  |                    help: give it a value of the expected type: `break 42`
  |
  = note: expected type `i32`
             found type `()`
