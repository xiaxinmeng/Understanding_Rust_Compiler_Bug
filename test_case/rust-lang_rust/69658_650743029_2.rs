rust
error[E0599]: no method named `call_clone` found for struct `Cloner<Inner<C>>` in the current scope
  --> src\main.rs:38:12
   |
4  | pub struct Inner<C> {
   | ------------------- doesn't satisfy `Inner<C>: std::clone::Clone`
...
8  | struct Cloner<C> {
   | ---------------- method `call_clone` not found for this
...
38 |     cloner.call_clone(&inner);
   |            ^^^^^^^^^^ method not found in `Cloner<Inner<C>>`
   |
   = note: the method `call_clone` exists but the following trait bounds were not satisfied:
           `Inner<C>: std::clone::Clone`
