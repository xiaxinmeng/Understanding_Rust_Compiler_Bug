
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in generic type due to conflicting requirements
 --> src/main.rs:5:5
  |
5 |     fn next(&mut self) -> Option<&str> { Some("h") }
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the method `next` is defined in the trait as returning `Option<Self::Item>`, which is defined in the impl as `&'a str`...
 --> src/main.rs:4:5
  |
4 |     type Item = &'a str;
  |     ^^^^^^^^^^^^^^^^^^^^
note: ...but the implementation of `next` returns `Option<&str>`:
 --> src/main.rs:5:5
  |
5 |     fn next(&mut self) -> Option<&str> { Some("h") }
  |                           ^^^^^^^^^^^^
help: change the return type for `next` to match the trait:
 --> src/main.rs:5:5
  |
5 |     fn next(&mut self) -> Option<&'a str> { Some("h") }
  |                           ^^^^^^^^^^^^^^^

