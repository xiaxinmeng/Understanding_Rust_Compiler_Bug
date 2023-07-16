
Compiling playground v0.0.1 (/playground)
error[[E0004]](https://doc.rust-lang.org/stable/error-index.html#E0004): non-exhaustive patterns: `(Enum::One, Enum::Two)`, `(Enum::One, Enum::Three)`, `(Enum::Two, Enum::Two)` and 1 more not covered
  --> src/main.rs:10:11
   |
10 |     match (a, b) {
   |           ^^^^^^ patterns `(Enum::One, Enum::Two)`, `(Enum::One, Enum::Three)`, `(Enum::Two, Enum::Two)` and 1 more not covered
   |
   = note: the matched value is of type `(Enum, Enum)`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown, or multiple match arms
   |
14 ~         (Three, _) => {}
15 +         _ => todo!()
   |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `playground` due to previous error
