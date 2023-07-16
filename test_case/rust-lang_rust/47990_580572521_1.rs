
error: the `f` method cannot be invoked on a trait object
 --> file.rs:8:14
  |
2 |     fn f<T>(&self) where Self: Sized {}
  |                                ----- this has a `Sized` requirement
...
8 |     receiver.f::<Param>();
  |              ^
