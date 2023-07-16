sh
$ cargo clean

$ time RUST_INCREMENTAL=1 cargo build --release
   Compiling incremental_bug v0.0.1 (file:///root/incremental_bug)
warning: field is never used: `name`
 --> src/main.rs:9:5
  |
9 |     name : String,
  |     ^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: field is never used: `next`
  --> src/main.rs:11:5
   |
11 |     next : U,
   |     ^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

    Finished release [optimized] target(s) in 0.51 secs

real	0m0.613s
user	0m0.532s
sys	0m0.052s

$ rustc -vV
rustc 1.19.0-nightly (06fb4d256 2017-04-30)
binary: rustc
commit-hash: 06fb4d25642a3f223db1441972dd5962085cfba1
commit-date: 2017-04-30
host: x86_64-unknown-linux-gnu
release: 1.19.0-nightly
LLVM version: 4.0
