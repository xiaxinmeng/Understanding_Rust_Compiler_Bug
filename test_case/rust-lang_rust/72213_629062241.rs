text
 ✗ cargo doc  
 Documenting foo v0.1.0 (file:///tmp/tmp.NxnOdGgulp/foo)
error[E0309]: the parameter type `L` may not live long enough
 --> src/lib.rs:9:5
  |
8 | struct Words<'a, L> {
  |                  - help: consider adding an explicit lifetime bound `L: 'a`...
9 |     _m: std::marker::PhantomData<&'a L>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: ...so that the reference type `&'a L` does not outlive the data it points at
 --> src/lib.rs:9:5
  |
9 |     _m: std::marker::PhantomData<&'a L>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
