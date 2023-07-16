
error[E0425]: cannot find value `x` in `const` scope
 --> src/main.rs:3:19
  |
2 |     let x: u32 = 22;
  |         - this binding is not in scope for `const` bindings
3 |     const Y: u8 = x;
  |                   ^ not found in `const` scope
  = note: `const` bindings can't access non-`const` bindings, like `x`
