
error: the `query` method cannot be invoked on a trait object
  --> src/lib.rs:14:11
   |
14 |         1.query::<dyn ToString>("")
   |           ^^^^^
   = note: the method receiver is a trait object because of the generic type parameter
   |
14 |         1.query::<dyn ToString>("")
   |                   ^^^^^^^^^^^^
   = note: the trait `crate::Example` defines this method but it's not available because
   |
2  |     fn query<Q>(self, q: Q);
   |              - this has a `Sized` requirement
