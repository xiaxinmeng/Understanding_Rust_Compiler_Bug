
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 [--> src/main.rs:3:9
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)  |
3 |     let (x, y) = &xs;
  |         ^^^^^^   --- this expression has type `&Option<Vec<({integer}, &str)>>`
  |         |
  |         expected enum `Option`, found tuple
  |
  = note: expected enum `Option<Vec<({integer}, &str)>>`
            found tuple `(_, _)`

error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 [--> src/main.rs:4:9
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)  |
4 |     for (x, y) in &xs {
  |         ^^^^^^    --- this expression has type `Option<&Vec<({integer}, &str)>>`
  |         |
  |         expected struct `Vec`, found tuple
  |
  = note: expected struct `Vec<({integer}, &str)>`
              found tuple `(_, _)`
