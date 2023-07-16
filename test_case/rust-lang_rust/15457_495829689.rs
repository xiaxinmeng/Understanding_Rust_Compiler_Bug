
error[E0507]: cannot move out of borrowed content
 --> file2.rs:7:9
  |
7 |         self.option.map(|x| x)
  |         ^^^^^^^^^^^
  |         |
  |         cannot move out of borrowed content
  |         help: consider borrowing the `Option`'s content: `self.option.as_ref()`

