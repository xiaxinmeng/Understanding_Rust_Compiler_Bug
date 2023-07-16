
$ cargo +1.60 build
   Compiling mycrate v0.1.0 (/rust-tests)
error: this operation will panic at runtime
   --> src/main.rs:129:13
    |
129 |     let _ = xs[7];
    |             ^^^^^ index out of bounds: the length is 5 but the index is 7
    |
    = note: `#[deny(unconditional_panic)]` on by default

error: could not compile `mycrate` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
