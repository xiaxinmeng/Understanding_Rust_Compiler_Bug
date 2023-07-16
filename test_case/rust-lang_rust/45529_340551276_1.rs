
$ rustc +nightly -Z thinlto -C codegen-units=16 -O foo.rs  --crate-type rlib --emit llvm-ir
