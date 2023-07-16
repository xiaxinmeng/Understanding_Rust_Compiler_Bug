
error[E0495]: borrowed data cannot be used outside closure
 --> src/main.rs:7:27
  |
7 |     give_any(|y| x = Some(y));
  |                           ^ data from the parameter `y` is only usable inside the closure
  |
