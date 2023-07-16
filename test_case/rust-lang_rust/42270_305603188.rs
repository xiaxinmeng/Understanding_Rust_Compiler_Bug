
error[E0593]: type mismatch in closure arguments
 --> src/main.rs:4:30
  |
4 |     let b: Vec<_> = a.iter().map(|x: (u32, u32)| 45).collect(); 
  |                              ^^^ ------------------ takes a `(u32, u32)`
  |                              |
  |                              expected closure that takes a `&(u32, u32)`
