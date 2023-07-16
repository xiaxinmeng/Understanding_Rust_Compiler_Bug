
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/main.rs:4:9
  |
4 |     let S(_, _) = S;
  |         ^^^^^^^   - this expression has type `fn(u8, u8) -> S {S}`
  |         |
  |         expected fn item, found struct `S`
  |
  = note: expected fn item `fn(u8, u8) -> S {S}`
              found struct `S`
