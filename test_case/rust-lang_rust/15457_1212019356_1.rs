
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/main.rs:7:9
  |
6 |     fn method(&self) -> Option<Vec<u8>> {
  |                         --------------- expected `Option<Vec<u8>>` because of return type
7 |         self.option.as_ref().map(|x| x)
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Vec`, found `&Vec<u8>`
  |
  = note: expected enum `Option<Vec<u8>>`
             found enum `Option<&Vec<u8>>`
