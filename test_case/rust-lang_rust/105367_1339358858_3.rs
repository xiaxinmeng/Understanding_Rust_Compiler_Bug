rust
error[[E0308]](https://doc.rust-lang.org/stable/error-index.html#E0308): mismatched types
 --> src/lib.rs:2:23
  |
2 |     let x: &mut str = "qwerty";  // x is declared as `mut`
  |            --------   ^^^^^^^^ types differ in mutability
  |            |
  |            expected due to this
  |
  = note: expected mutable reference `&mut str`
                     found reference `&'static str`
