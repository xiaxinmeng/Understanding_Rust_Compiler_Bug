
$ touch empty.rs
$ rustup run stable-i686   rustc --emit=llvm-ir --crate-type rlib -o x86.ll empty.rs
$ rustup run stable-x86_64 rustc --emit=llvm-ir --crate-type rlib -o x64.ll empty.rs
