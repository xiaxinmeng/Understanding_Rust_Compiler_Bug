
error[E0282]: type annotations needed
 --> src/lib.rs:4:13
  |
2 |     match serde_json::from_str(input).unwrap() {
  |           ------------------------------------ this method call resolves to `T`
3 |         Ok(v) => Ok(v),
4 |         Err(why) => Err(format!("JSON decode failed: {:?}", why)),
  |             ^^^ cannot infer type
