 nocode
error: casting `&f64` as `i16` is invalid
 --> foo.rs:2:35
  |
2 |     vec![0.0].iter().map(|s| s as i16).collect::<Vec<i16>>();
  |                              -    ^^^ cannot cast `&f64` as `i64`
  |                              |
  |                              did you mean `*s`?
