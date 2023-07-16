
error[E0515]: cannot return value referencing function parameter `xs`
  --> lib.rs:15:5
   |
11 |     for ref x in xs {
   |         ----- `xs` is borrowed here
...
15 |     result
   |     ^^^^^^ returns a value referencing data owned by the current function
