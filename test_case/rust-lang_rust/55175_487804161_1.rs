
error[E0308]: mismatched types
 --> src/main.rs:8:13
  |
8 |         Foo(&bar) => {
  |             ^^^^ expected i32, found reference
  |
  = note: expected type `i32`
             found type `&_`
  = help: did you mean `bar: &i32`?
