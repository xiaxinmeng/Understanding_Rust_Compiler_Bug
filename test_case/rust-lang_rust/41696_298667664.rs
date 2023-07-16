
$ cargo build --release
   Compiling incremental_bug v0.0.1 (file:///home/fl3/fun/outrage_test/incremental_bug)
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

    Finished release [optimized] target(s) in 443.30 secs
