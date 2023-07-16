shell
$ rustc --emit llvm-ir --target x86_64-pc-windows-msvc --crate-type rlib -C debuginfo=2 foo.rs
