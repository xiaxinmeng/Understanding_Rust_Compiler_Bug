
   Compiling playground v0.0.1 (/playground)
error: this operation will panic at runtime
 --> src/main.rs:8:20
  |
8 |     println!("{}", a[2]);
  |                    ^^^^ index out of bounds: the length is 2 but the index is 2
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: could not compile `playground` due to previous error
