
error[E0282]: type annotations needed
 --> src/main.rs:5:20
  |
5 |         may_error()?;
  |                    ^ cannot infer type of error for `?` operator
  |
  = note: `?` implicitly converts the error value into a type implementing `From<()>`
