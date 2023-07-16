
error[E0433]: failed to resolve. Use of undeclared type or module `Trait`
  --> src/main.rs:21:46
   |
21 |     fn get_next(&mut self) -> Option<<Foo as Trait>::Item> {
   |                                              ^^^^^ Use of undeclared type or module `Trait`
