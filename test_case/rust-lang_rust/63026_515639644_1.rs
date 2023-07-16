
error[E0515]: cannot return value referencing local variable `__next`
  --> file9.rs:28:11
   |
19 |                           let ref x = __next;
   |                               ----- `__next` is borrowed here
...
28 |           result
   |           ^^^^^^ returns a value referencing data owned by the current function
