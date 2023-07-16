
error[E0034]: multiple applicable items in scope
  --> src/lib.rs:26:13
   |
26 |         arg.method()
   |             ^^^^^^ multiple `method` found
   |
note: candidate #1 is defined in an impl of the trait `Trait1` for the type `u8`
  --> src/lib.rs:9:5
   |
9  |     fn method(&self) {}
   |     ^^^^^^^^^^^^^^^^
note: candidate #2 is defined in an impl of the trait `Trait2` for the type `u8`
  --> src/lib.rs:13:5
   |
13 |     fn method(&self) {}
   |     ^^^^^^^^^^^^^^^^
