
error[E0038]: the trait `Bar` cannot be made into an object
 --> src/lib.rs:5:6
  |
5 | impl Foo for Bar { }
  |      ^^^ the trait `Bar` cannot be made into an object
  |
  = note: the trait cannot require that `Self : Sized`
