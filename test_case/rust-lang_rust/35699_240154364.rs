
  --> src/test/compile-fail/E0409.rs:15:23
15 |         (0, ref y) | (y, 0) => {} //~ ERROR E0409
   |                 -     ^ bound in different ways
   |                 |
   |                 first binding
