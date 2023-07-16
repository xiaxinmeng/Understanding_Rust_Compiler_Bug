
error[E0599]: no method named `bar` found for struct `Foo<Inner>` in the current scope
  --> file5.rs:24:9
   |
1  | struct Foo<T> {
   | ------------- method `bar` not found for this
...
5  | struct Inner {
   | ------------ this type doesn't satisfy the bound `Inner: std::clone::Clone`
   |
   = note: the method `bar` exists but the following trait bounds were not satisfied:
           `Inner: std::clone::Clone` which is required by `Foo<Inner>: Bar`
   = help: items from traits can only be used if the trait is implemented and in scope
