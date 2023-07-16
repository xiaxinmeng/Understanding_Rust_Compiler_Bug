
    error: expected `=`, found `where`
     --> <source>:6:14
      |
    6 |     type Out where T: Clone = T;
      |              ^^^^^
    error[E0046]: not all trait items implemented, missing: `Out`
     --> <source>:5:1
      |
    2 |       type Out;
      |       --------- `Out` from trait
    ...
    5 | / impl<T> Foo for Box<T> {
    6 | |     type Out where T: Clone = T;
    7 | | }
      | |_^ missing `Out` in implementation
    error: aborting due to 2 previous errors
    