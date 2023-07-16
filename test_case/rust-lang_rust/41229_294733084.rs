
 $ rustc -vV
rustc 1.18.0-nightly (ad36c2f55 2017-04-09)
binary: rustc
commit-hash: ad36c2f5528d617db66c244d8bcbfc4b36da0ca0
commit-date: 2017-04-09
host: x86_64-apple-darwin
release: 1.18.0-nightly
LLVM version: 3.9

 $ rustc src/main.rs
warning: unused variable: `s`
 --> src/main.rs:1:16
  |
1 | pub fn example(ref s: str) {}
  |                ^^^^^
  |
  = note: #[warn(unused_variables)] on by default

 $ ./main

 $ 
