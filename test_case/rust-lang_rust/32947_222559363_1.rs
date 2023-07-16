
rustup run nightly cargo test
   Compiling 32947 v0.1.0 (file:///private/tmp/32947)
src/lib.rs:7:5: 7:16 warning: struct field is never used: `state`, #[warn(dead_code)] on by default
src/lib.rs:7     state: Mu64, // Without this field, it works fine.
                 ^~~~~~~~~~~
src/lib.rs:7:5: 7:16 warning: struct field is never used: `state`, #[warn(dead_code)] on by default
src/lib.rs:7     state: Mu64, // Without this field, it works fine.
                 ^~~~~~~~~~~
src/lib.rs:31:9: 31:20 warning: variable does not need to be mutable, #[warn(unused_mut)] on by default
src/lib.rs:31     let mut trouble = Trouble::new();
                      ^~~~~~~~~~~
     Running target/debug/32947-044b564592bca0e2

running 1 test
error: Process didn't exit successfully: `/private/tmp/32947/target/debug/32947-044b564592bca0e2` (signal: 11, SIGSEGV: invalid memory reference)
