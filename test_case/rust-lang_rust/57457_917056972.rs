
error[E0599]: no method named `f` found for struct `S` in the current scope
  --> src/main.rs:15:7
   |
6  |     pub struct S;
   |     ------------- method `f` not found for this
...
15 |     s.f();
   |       ^ method not found in `S`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `T` defines an item `f`, perhaps you need to implement it
  --> src/main.rs:2:5
   |
2  |     trait T {
   |     ^^^^^^^
