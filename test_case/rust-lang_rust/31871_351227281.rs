
error[E0594]: cannot assign to captured outer variable in an `FnMut` closure
  --> src/main.rs:12:9
   |
8  |     let x = NonCopy::new(42);
   |         - help: consider making `x` mutable: `mut x`
...
12 |         x = v;
   |         ^^^^^
