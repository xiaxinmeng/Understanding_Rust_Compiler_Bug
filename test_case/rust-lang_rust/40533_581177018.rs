
error[E0038]: the trait `Foo` cannot be made into an object
 --> file.rs:7:19
  |
3 | trait Foo: Super<<Self as Foo>::Bar> {
  |       ---  ------------------------- ...because it cannot use `Self` as a type parameter in the supertraits or `where`-clauses
  |       |
  |       this trait cannot be made into an object...
...
7 | type BoxFoo = Box<dyn Foo<Bar = u8>>;
  |                   ^^^^^^^^^^^^^^^^^ the trait `Foo` cannot be made into an object
