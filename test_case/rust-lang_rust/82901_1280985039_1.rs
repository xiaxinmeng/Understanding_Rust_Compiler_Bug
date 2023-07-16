console
error[E0282]: type annotations needed for `Option<T>`
 --> src/main.rs:5:9
  |
5 |     let mut junks = None;
  |         ^^^^^^^^^
...
8 |             junks.get_or_insert_default().insert(value);
  |                                           ------ type must be known at this point
