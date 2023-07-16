
error[E0593]: closure takes 2 arguments but 1 argument is required
 --> test.rs:5:40
  |
5 |     let it = v.into_iter().enumerate().map(|i, x| i);
  |                                        ^^^ ------ takes 2 arguments
  |                                        |
  |                                        expected closure that takes 1 argument
  = help: consider changing the closure signature like this:
5 |     let it = v.into_iter().enumerate().map(|(i, x|) i);
