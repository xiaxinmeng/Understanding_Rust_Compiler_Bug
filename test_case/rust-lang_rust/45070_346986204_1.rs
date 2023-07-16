
error[E0599]: no method named `clone` found for type `main::S` in the current scope
 --> src/main.rs:5:7
  |
4 |     struct S;
  |     --------- method `clone` not found for this
5 |     S.clone();
  |       ^^^^^
  |
  = help: items from traits can only be used if the trait is implemented and in scope
  = note: the following trait defines an item `clone`, perhaps you need to implement it:
          candidate #1: `serde::export::Clone`
