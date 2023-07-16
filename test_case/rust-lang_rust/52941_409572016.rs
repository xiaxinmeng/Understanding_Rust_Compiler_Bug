
error[E0594]: cannot assign to `foo.x` which is behind a `&` reference
 --> src/main.rs:3:5
  |
2 |     let foo = &Bar{x: 5}; // or any function that returns a shared reference
  |               ---------- help: consider changing this to be a mutable reference: `&mut Bar{x: 5}`
3 |     foo.x -= 12;
  |     ^^^^^^^^^^^ `foo` is a `&` reference, so the data it refers to cannot be written
