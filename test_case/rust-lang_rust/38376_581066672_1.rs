
error[E0038]: the trait `Bar` cannot be made into an object
 --> file.rs:5:6
  |
3 | trait Bar: Sized { }
  |       ---  ----- ...because it requires `Self: Sized`
  |       |
  |       this trait cannot be made into an object...
4 |
5 | impl Foo for Bar { }
  |      ^^^ the trait `Bar` cannot be made into an object
