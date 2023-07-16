
error: higher-ranked lifetime error
 --> src/main.rs:7:5
  |
7 |     impl_check()
  |     ^^^^^^^^^^
  |
  = note: could not prove `for<'a, 'b> u8: Service<&'a &'b u8>`
