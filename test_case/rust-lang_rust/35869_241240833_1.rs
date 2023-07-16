
error[E0053]: method `foo` has an incompatible type for trait
 --> <anon>:8:5
  |
8 |     fn foo(&self, _: &[u8]) {}
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected str, found slice
  |
  = note: expected type `fn(&Bar, &str)`
  = note:    found type `fn(&Bar, &[u8])`
