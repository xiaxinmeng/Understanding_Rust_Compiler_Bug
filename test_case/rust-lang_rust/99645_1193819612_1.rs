
error[[E0308]](https://doc.rust-lang.org/stable/error-index.html#E0308): mismatched types
 --> src/main.rs:9:14
  |
9 |         self(a);
  |              ^ lifetime mismatch
  |
  = note: expected struct `A<'a>`
             found struct `A<'_>`
note: ...
