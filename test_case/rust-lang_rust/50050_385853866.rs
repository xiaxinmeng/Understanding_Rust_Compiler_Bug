
$ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc foo.rs --crate-type proc-macro
$ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc bar.rs -L .
error[E0425]: cannot find value `b` in this scope
 --> bar.rs:5:1
  |
5 | / foo::foo! {
6 | |     let a = 3;
7 | | }
  | |_^ not found in this scope

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
