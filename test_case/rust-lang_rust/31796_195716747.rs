
$ rustc --target=armv5te-unknown-linux-musl --emit llvm-bc src/main.rs
$ llc main.bc -o main.s
error: Invalid ID
