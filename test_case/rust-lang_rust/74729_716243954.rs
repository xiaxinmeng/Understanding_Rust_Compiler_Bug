
error[E0425]: cannot find value `foo` in this scope
 --> src/lib.rs:8:13
  |
1 | struct Foo;
  | ----------- similarly named unit struct `Foo` defined here
...
8 |     if Some(foo) = Foo::get() {
  |             ^^^
  |
help: you might have meant to use pattern matching
  |
8 |     if let Some(foo) = Foo::get() {
  |        ^^^
help: a unit struct with a similar name exists
  |
8 |     if Some(Foo) = Foo::get() {
  |             ^^^
