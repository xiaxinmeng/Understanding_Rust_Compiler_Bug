
   Compiling demo v0.1.0 (/home/nmx/demo)
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-unknown-linux-musl` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-musl`

error: cannot find macro `println` in this scope
 --> src/main.rs:2:5
  |
2 |     println!("Hello, world!");
  |     ^^^^^^^

error: requires `sized` lang_item

For more information about this error, try `rustc --explain E0463`.
error: could not compile `demo` due to 3 previous errors
[root@host-10-89-235-184 target]# rustup target add
error: error: The following required arguments were not provided:
    <target>...

USAGE:
    rustup target add [OPTIONS] <target>...

