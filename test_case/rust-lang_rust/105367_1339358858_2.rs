rust
warning: variable does not need to be mutable
 --> src/lib.rs:2:9
  |
2 |     let mut x = "qwerty";  // x is declared as `mut`
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

error[[E0596]](https://doc.rust-lang.org/stable/error-index.html#E0596): cannot borrow `*x` as mutable, as it is behind a `&` reference
 --> src/lib.rs:3:5
  |
2 |     let mut x = "qwerty";  // x is declared as `mut`
  |         ----- consider changing this binding's type to be: `&mut str`
3 |     x.make_ascii_lowercase();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
warning: `playground` (lib) generated 1 warning
error: could not compile `playground` due to previous error; 1 warning emitted
