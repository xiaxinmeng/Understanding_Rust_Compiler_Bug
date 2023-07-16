
error[E0599]: no method named `bar` found for struct `Foo<Inner>` in the current scope
  --> file5.rs:24:9
   |
1  | struct Foo<T> {
   | ------------- method `bar` not found for this
...
5  | struct Inner {
   | ------------ the method `bar` but this type doesn't satisfy the bound `Inner: std::clone::Clone`
...
9  | trait Bar {
   | --------- this trait defines an item `bar`
...
24 |     foo.bar();
   |         ^^^ method not found in `Foo<Inner>`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
