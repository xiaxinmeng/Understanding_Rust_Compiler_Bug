sh
william@xubuntu-dtrain:~/Projects/MCVE/proc-macro-ice$ cargo +nightly build
   Compiling proc-macro-ice v0.1.0 (/home/william/Projects/MCVE/proc-macro-ice)
error: custom attribute panicked
 --> src/main.rs:3:1
  |
3 | #[tester]
  | ^^^^^^^^^
  |
  = help: message: explicit panic

error: could not compile `proc-macro-ice` due to previous error
