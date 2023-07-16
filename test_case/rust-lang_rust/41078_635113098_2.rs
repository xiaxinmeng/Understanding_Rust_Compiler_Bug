
error: borrowed data cannot be stored outside of its closure
 --> src/main.rs:3:40
  |
2 |     let mut fields: Vec<&str> = Vec::new();
  |                         - cannot infer an appropriate lifetime...
3 |     let pusher = |a: &str| fields.push(a);
  |         ------   ---------             ^ cannot be stored outside of its closure
  |         |        |
  |         |        borrowed data cannot outlive this closure
  |         ...so that variable is valid at time of its declaration

error: aborting due to previous error

error: could not compile `playground`.
