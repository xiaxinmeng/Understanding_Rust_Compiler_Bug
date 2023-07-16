
error[E0384]: cannot assign twice to immutable variable `foo`
 --> src/main.rs:4:2
  |
3 |  let foo = Bar{x: 0f32};
  |      ---
  |      |
  |      first assignment to `foo`
  |      consider changing this to `mut foo`
4 |  foo.x = y;
  |  ^^^^^^^^^ cannot assign twice to immutable variable
