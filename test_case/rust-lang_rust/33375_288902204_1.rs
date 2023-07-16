
error[E0038]: the trait `Foo` cannot be made into an object
  --> quickie.rs:11:1
   |
11 |   fn lookup() -> Box<&'static Foo> {
   |  _^ starting here...
12 | |     Box::new(&BAZ)
13 | | }
   | |_^ ...ending here: the trait `Foo` cannot be made into an object
   |
   = note: method `apply` has no receiver
