
/tmp/conch-runtime 
$ cargo clean; time cargo test --lib
...
^C  Building [======================================================> ] 99/100: conch-runtime                                 
real	7m6.081s
user	1m41.996s
sys	0m5.601s

//yes I C-c-ed it!

$ rustv
!! former-master-installed (default)
!! Executing '/home/user/.cargo/bin/rustc' in pwd='/home/user' with args(1): '-vV'
rustc 1.37.0-dev (cdd743755 2019-06-13)
binary: rustc
commit-hash: cdd743755adef8eb27410a9f4e480d3cf74abeaf
commit-date: 2019-06-13
host: x86_64-unknown-linux-gnu
release: 1.37.0-dev
LLVM version: 8.0
!! former-master-installed (default)
!! Executing '/home/user/build/2nonpkgs/rust.stuff/cargo/cargo//target/release//cargo' in pwd='/home/user' with args(1): '-vV'
cargo 1.37.0-dev (fa05862c 2019-06-13)
release: 1.37.0
commit-hash: fa05862cd0c6b899b801fda0f256ac5b9bae69d9
commit-date: 2019-06-13
