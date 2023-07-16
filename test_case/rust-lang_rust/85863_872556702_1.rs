
error: cannot specialize on `'static` lifetime
  --> src/lib.rs:11:1
   |
11 | impl SpecTrait for MyEmptyIterator<fn(&'static ())> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
