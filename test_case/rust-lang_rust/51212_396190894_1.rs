
error[E0412]: cannot find type `Debug` in this scope
 --> src/main.rs:2:13
  |
2 |     let x: &Debug = &22;
  |             ^^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
  |
1 | use std::fmt::Debug;
  |
