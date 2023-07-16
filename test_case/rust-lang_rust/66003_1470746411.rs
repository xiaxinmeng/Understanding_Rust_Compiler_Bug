
error: custom attribute panicked
 --> proc-macro-bug/src/main.rs:7:1
  |
7 | #[some_macro(0)]
  | ^^^^^^^^^^^^^^^^
  |
  = help: message: use-after-free of `proc_macro` symbol

error: could not compile `proc-macro-bug` (bin "proc-macro-bug") due to previous error
