
rustc 1.13.0 (2c6933acc 2016-11-07)
error[E0507]: cannot move out of borrowed content
 --> <anon>:9:26
  |
9 |         println!("{:?}", y.field.unwrap_or(X));
  |                          ^ cannot move out of borrowed content
