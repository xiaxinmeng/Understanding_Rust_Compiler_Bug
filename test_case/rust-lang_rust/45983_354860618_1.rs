
error[E0495]: borrowed data cannot be used outside closure
 --> src/main.rs:7:27
  |
6 |     let x = None;
  |         - binding declared outside of the closure
7 |     give_any(|y| x = Some(y));
  |              --- -        ^ data from the parameter `y` is only usable inside its closure
  |              |   |
  |              |   you're assigning to this binding declared outside of the closure
  |              the closure you can't escape
