
error[E0594]: cannot assign to `x`, as it is not declared as mutable
  --> src/main.rs:12:9
   |
8  |     let x = NonCopy::new(42);
   |         - help: consider changing this to be mutable: `mut x`
...
12 |         x = v;
   |         ^ cannot assign
