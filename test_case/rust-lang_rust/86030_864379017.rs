rust
error: Undefined Behavior: unwinding past the topmost frame of the stack
  --> /playground/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:43:1
   |
43 | / fn lang_start<T: crate::process::Termination + 'static>(
44 | |     main: fn() -> T,
45 | |     argc: isize,
46 | |     argv: *const *const u8,
...  |
52 | |     )
53 | | }
   | |_^ unwinding past the topmost frame of the stack
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
           
   = note: inside `std::rt::lang_start::<()>` at /playground/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:43:1

error: aborting due to previous error
