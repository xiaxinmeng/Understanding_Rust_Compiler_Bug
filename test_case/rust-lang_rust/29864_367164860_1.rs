
error[E0310]: the parameter type `T` may not live long enough
 --> src/lib.rs:5:9
  |
5 | impl<T> F for T where T: Copy {}
  |      -  ^
  |      |
  |      help: consider adding an explicit lifetime bound `T: 'static`...
  |
note: ...so that the type `T` will meet its required lifetime bounds
 --> src/lib.rs:5:9
  |
5 | impl<T> F for T where T: Copy {}
