
error: unterminated raw string
 --> src/main.rs:3:15
  |
3 |     let baz = r##"quxx"#;
  |               ^        -
  |               |        |
  |               |        the raw string needs two trailing `#`, but found 1
  |               |        help: close the raw string: `##`
  |               unterminated raw string
