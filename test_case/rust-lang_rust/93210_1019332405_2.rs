
$ rustc lib.rs --crate-type=lib && rustc --extern lib=liblib.rlib main.rs --edition=2021 
error[E0609]: no field `hey` on type `A`
 --> main.rs:2:23
  |
2 |     lib::A::default().hey;
  |                       ^^^ unknown field
  |
  = note: available fields are: `hello`
