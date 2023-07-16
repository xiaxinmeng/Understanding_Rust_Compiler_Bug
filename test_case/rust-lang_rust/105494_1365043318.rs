
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/main.rs:5:12
  |
2 |     let mut path: String = "/usr".to_string();
  |                   ------ expected due to this type
...
5 |     path = format!("{}/{}", path, folder).as_str();
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `String`, found `&str`
  |
help: try removing the method call
  |
5 -     path = format!("{}/{}", path, folder).as_str();
5 +     path = format!("{}/{}", path, folder);
  |

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` due to previous error
