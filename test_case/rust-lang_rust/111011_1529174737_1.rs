
Compiling playground v0.0.1 (/playground)
error[[E0308]](https://doc.rust-lang.org/nightly/error_codes/E0308.html): mismatched types
 --> src/main.rs:5:9
  |
5 |     foo({});
  |         ^^ expected `Box<_>`, found `()`
  |
  = note: expected struct `Box<_>`
          found unit type `()`

error[[E0308]](https://doc.rust-lang.org/nightly/error_codes/E0308.html): mismatched types
 --> src/main.rs:6:12
  |
6 |     bar(|| {});
  |            ^^ expected `Box<_>`, found `()`
  |
  = note: expected struct `Box<_>`
          found unit type `()`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` (bin "playground") due to 2 previous errors
