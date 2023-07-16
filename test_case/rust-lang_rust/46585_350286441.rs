
error[E0576]: cannot find associated type `Item` in trait `Trait`
  --> src/main.rs:23:54
   |
23 |     fn get_next(&mut self) -> Option<<Foo as Trait>::Item> {
   |                                                      ^^^^ not found in `Trait`
