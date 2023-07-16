
error[E0308]: mismatched types
 --> src/lib.rs:5:38
  |
5 |         if /*let*/ Some(reference) = reference {
  |                                      ^^^^^^^^^
  |                                      |
  |                                      expected enum `Option`, found integer
  |                                      help: try using a variant of the expected enum: `Some(reference)`
  |
  = note: expected enum `Option<Option<{integer}>>`
             found enum `Option<{integer}>`

error[E0308]: mismatched types
 --> src/lib.rs:5:20
  |
5 |         if /*let*/ Some(reference) = reference {
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`

error: aborting due to 2 previous errors
