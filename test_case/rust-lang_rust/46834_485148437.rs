
error[E0594]: cannot assign to captured outer variable in an `Fn` closure
 --> src/main.rs:5:17
  |
5 |     bar(move || x = 1);
  |                 ^^^^^
  |
  = note: `Fn` closures cannot capture their enclosing environment for modifications
help: consider changing this closure to take self by mutable reference
 --> src/main.rs:5:9
  |
5 |     bar(move || x = 1);
  |         ^^^^^^^^^^^^^

warning: variable does not need to be mutable
 --> src/main.rs:4:9
  |
4 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: #[warn(unused_mut)] on by default

error: aborting due to previous error
