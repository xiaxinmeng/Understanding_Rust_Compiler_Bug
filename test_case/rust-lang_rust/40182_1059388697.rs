
error[E0277]: `Rc<u32>` cannot be sent between threads safely
 --> src/main.rs:9:53
  |
9 |     let x = if b { Box::new(Rc::new(1u32)) } else { Box::new(0u32) as Box<dyn T + Send> };
  |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Rc<u32>` cannot be sent between threads safely
  |
  = help: the trait `Send` is not implemented for `Rc<u32>`
  = note: required for the cast to the object type `dyn T + Send`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to previous error
