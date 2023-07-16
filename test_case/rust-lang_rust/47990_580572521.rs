
error: the `bar` method cannot be invoked on a trait object
  --> file.rs:10:9
   |
5  |     fn bar<T>(&self) {}
   |            - this has a `Sized` requirement
...
10 |     foo.bar::<Trait>(&foo);
   |         ^^^
