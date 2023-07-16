

error[E0310]: the parameter type `T` may not live long enough
 --> src/lib.rs:5:5
  |
4 | fn new_funny_handle<T: Funny + Default>() -> Box<Funny> {
  |                     -- help: consider adding an explicit lifetime bound `T: 'static`...
5 |     Box::new(T::default())
  |     ^^^^^^^^^^^^^^^^^^^^^^
  |
note: ...so that the type `T` will meet its required lifetime bounds
 --> src/lib.rs:5:5
  |
5 |     Box::new(T::default())
  |     ^^^^^^^^^^^^^^^^^^^^^^
