
error[E0433]: failed to resolve: use of undeclared type or module `Trait`
  --> src/main.rs:23:46
   |
23 |     fn get_next(&mut self) -> Option<<Foo as Trait>::Item> {
   |                                              ^^^^^ use of undeclared type or module `Trait`
