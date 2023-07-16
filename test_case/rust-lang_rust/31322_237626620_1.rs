
   Compiling core v0.0.0 (file:///home/jirutjak/rustc-1.10.0/src/libcore)
src/libcore/intrinsics.rs:290:5: 290:42 error: unrecognized intrinsic function: `return_address` [E0093]
src/libcore/intrinsics.rs:290     pub fn return_address() -> *const u8;
                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/libcore/intrinsics.rs:290:5: 290:42 help: run `rustc --explain E0093` to see a detailed explanation
error: aborting due to previous error
error: Could not compile `core`.
