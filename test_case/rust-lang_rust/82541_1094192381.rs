
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 [--> src/main.rs:6:9
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)  |
6 |     for (i,j) in make() {
  |         ^^^^^    ------ this expression has type `Option<Vec<(usize, char)>>`
  |         |
  |         expected struct `Vec`, found tuple
  |
  = note: expected struct `Vec<(usize, char)>`
              found tuple `(_, _)`
