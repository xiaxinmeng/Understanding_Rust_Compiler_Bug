
rustup run nightly cargo test --release
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
     Running target/release/32947-044b564592bca0e2

running 1 test
test this_causes_sigsegv ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests 32947

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
