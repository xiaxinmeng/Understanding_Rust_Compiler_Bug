
error: the `query` method cannot be invoked on a trait object
  --> file.rs:14:11
   |
2  |     fn query<Q>(self, q: Q);
   |              - this has a `Sized` requirement
...
14 |         1.query::<dyn ToString>("")
   |           ^^^^^
   |
   = note: another candidate was found in the following trait, perhaps add a `use` for it:
           `use Example;`
