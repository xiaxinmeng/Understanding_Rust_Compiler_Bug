
error[E0599]: no method named `push` found for type `&mut <[X] as std::borrow::ToOwned>::Owned` in the current scope
  --> src/main.rs:10:30
   |
10 |         self.values.to_mut().push(x);
   |                              ^^^^
