
error: `Self` is not valid in the self type of an impl block
 --> src/lib.rs:5:14
  |
5 |         impl Self { fn bar() {} }
  |              ^^^^
  |
  = note: replace `Self` with a different type

error: could not compile `playground` due to previous error
