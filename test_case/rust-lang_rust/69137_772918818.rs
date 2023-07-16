
error[E0309]: the parameter type `F` may not live long enough
 --> src/main.rs:4:4
  |
4 |  = impl Fn<(A,), Output = impl FnOnce(B) -> C + 'a> + 'a;
  |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: consider adding an explicit lifetime bound `F: 'a`...
  = note: ...so that the type `[closure@src/main.rs:9:5: 9:29]` will meet its required lifetime bounds

error[E0309]: the parameter type `F` may not live long enough
 --> src/main.rs:4:27
  |
4 |  = impl Fn<(A,), Output = impl FnOnce(B) -> C + 'a> + 'a;
  |                           ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: consider adding an explicit lifetime bound `F: 'a`...
  = note: ...so that the type `[closure@src/main.rs:9:14: 9:29]` will meet its required lifetime bounds

error: aborting due to 2 previous errors
