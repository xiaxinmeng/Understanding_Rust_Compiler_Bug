sh
$ time RUST_INCREMENTAL=1 cargo build --release
   Compiling incremental_bug v0.0.1 (file:///Users/kennytm/Downloads/incremental_bug)
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
11 |     next : U
   |     ^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

    Finished release [optimized] target(s) in 1.12 secs

real	0m1.445s
user	0m0.417s
sys	0m0.297s

$ rustc -vV
rustc 1.19.0-nightly (777ee2079 2017-05-01)
binary: rustc
commit-hash: 777ee20796e80a31d4b7c985dd68eda2941460d6
commit-date: 2017-05-01
host: x86_64-apple-darwin
release: 1.19.0-nightly
LLVM version: 4.0
