
  --> src/main.rs:13:18
   |
13 |         .map(|x| x.as_ref())
   |                  -^^^^^^^^^
   |                  |
   |                  returns a value referencing data owned by the current function
   |                  `x` is borrowed here
