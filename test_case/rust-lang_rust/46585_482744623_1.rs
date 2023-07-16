
error[E0223]: ambiguous associated type
  --> src/main.rs:23:38
   |
23 |     fn get_next(&mut self) -> Option<Self::Item> {
   |                                      ^^^^^^^^^^ help: use fully-qualified syntax: `<Foo as Trait>::Item`
