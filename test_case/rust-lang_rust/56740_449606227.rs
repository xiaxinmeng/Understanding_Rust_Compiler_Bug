
error[EXXXX]: cannot index `String` or string slice (`str`)
 --> main.rs:2:24
  |
  |     let _value = "test"[0];
  |                        ^^^ cannot index here
  |
  |
  = help: consider using `.chars()` or `.to_bytes()
