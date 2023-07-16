
swei@macbook:~/code/examples/workday/src
$ rustc --version
rustc 1.41.0-nightly (412f43ac5 2019-11-24)

swei@macbook:~/code/examples/workday/src
$ cargo build
   Compiling workday v0.1.0 (/Users/swei/code/examples/workday)
error[E0282]: type annotations needed
 --> src/main.rs:6:28
  |
6 |     let example_closure = |x| x;
  |                            ^ consider giving this closure parameter a type

