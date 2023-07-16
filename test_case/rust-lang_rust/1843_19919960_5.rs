
+ rustc --link-args ntwice.o play-foreign3.rs
play-foreign3.rs:30:33: 30:39 error: mismatched types: expected `extern "Rust" fn(extern "C" fn(i32) -> i32, i32) -> i32` but found `*u8` (expected extern fn but found *-ptr)
play-foreign3.rs:30         let y = ntwice::callback(rtwice, 4);
                                                     ^~~~~~
error: aborting due to previous error
