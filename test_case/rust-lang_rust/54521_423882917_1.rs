
   Compiling playground v0.0.1 (/playground)
error[E0423]: expected value, found struct `Vec`
 --> src/main.rs:2:22
  |
2 |     println!("{:?}", Vec::<usize>>::new());
  |                      ^^^^^^^^^^^^ did you mean `Vec { /* fields */ }`?

error[E0425]: cannot find function `new` in the crate root
 --> src/main.rs:2:37
  |
2 |     println!("{:?}", Vec::<usize>>::new());
  |                                     ^^^ not found in the crate root
