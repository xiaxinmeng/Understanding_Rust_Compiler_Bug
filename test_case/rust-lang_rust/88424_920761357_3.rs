rust
error: `~const` is not allowed here
  --> ...\test2.rs:46:58
   |
46 |     pub unsafe fn new_unchecked(i: Ti) -> Self where Ti: ~const ToFromUsize {
   |                                                          ^^^^^^^^^^^^^^^^^^
   |
   = note: only allowed on bounds on traits' associated types and functions, const fns, const impls and its associated functions

