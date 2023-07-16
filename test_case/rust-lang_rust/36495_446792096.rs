
error[E0282]: type annotations needed
  --> src/main.rs:21:28
   |
16 |     let v = "hello world".split_whitespace().map(|x| x.to_string()).collect();
   |         - consider giving `v` a type
...
21 |     let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
   |                            ^ cannot infer type
   |
   = note: type must be known at this point
