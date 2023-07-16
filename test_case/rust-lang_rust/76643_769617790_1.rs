
error[E0308]: mismatched types
 --> src/lib.rs:6:9
  |
5 |     fn my_clone(&self) -> MyStruct {
  |                           -------- expected `MyStruct` because of return type
6 |         self.clone()
  |         ^^^^^^^^^^^^ expected struct `MyStruct`, found `&MyStruct`
