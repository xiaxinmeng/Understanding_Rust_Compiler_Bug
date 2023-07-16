
$ rustc --target=armv5te-unknown-linux-musl --emit llvm-bc src/main.rs
$ llc main.bc -o main.s -march=arm -float-abi=soft -mattr=+v5te,+soft-float -O2
