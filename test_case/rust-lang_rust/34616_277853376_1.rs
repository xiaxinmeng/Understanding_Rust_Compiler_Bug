
>cargo build
   Compiling guessing_game v0.1.0 (file:///D:/_git/rust/guessing_game)
error: use of unstable library feature 'rand': use `rand` from crates.io (see issue #27703)
 --> src\main.rs:1:1
  |
1 | extern crate rand;
  | ^^^^^^^^^^^^^^^^^^

error: use of unstable library feature 'rand': use `rand` from crates.io (see issue #27703)
 --> src\main.rs:4:5
  |
4 | use rand::Rng;
  |     ^^^^^^^^^
