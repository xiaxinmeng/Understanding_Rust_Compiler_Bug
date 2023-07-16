
$ cargo +77a8ed98fc66821a410b0c4696ba99b07b713467 run
   Compiling segfault v0.1.0 (file:///Users/felipenoris/Documents/src/learnrust/segfault)
error: index out of bounds: the len is 1 but the index is 1
 --> src/main.rs:3:20
  |
3 |     println!("{}", x[1]);
  |                    ^^^^
  |
  = note: #[deny(const_err)] on by default

error: aborting due to previous error

error: Could not compile `segfault`.

To learn more, run the command again with --verbose.
