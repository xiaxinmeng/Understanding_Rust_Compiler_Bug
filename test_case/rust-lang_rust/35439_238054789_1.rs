
spacey@masonjar:~/dvcs/github/rust-lang/rust$ RUST_NEW_ERROR_FORMAT=true ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc src/test/compile-fail/E0010.rs
error[E0010]: allocations are not allowed in constants
  --> src/test/compile-fail/E0010.rs:13:24
   |
13 | const CON : Box<i32> = box 0; //~ ERROR E0010
   |                        ^^^^^ allocation not allowed in constants

error: aborting due to previous error
