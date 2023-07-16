
error: this operation will panic at runtime
 --> <source>:3:20
  |
3 |     println!("{}", xs[10]);
  |                    ^^^^^^ index out of bounds: the length is 3 but the index is 10
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: aborting due to previous error
