
error[E0308]: mismatched types
 --> src/main.rs:2:24
  |
2 |       if let Some(_) = a {
  |  ________________________^
3 | |         println!("Foo");
4 | |     } else {
  | |_____^ expected enum `Option`, found `()`
  |
  = note:   expected enum `Option<u32>`
          found unit type `()`
