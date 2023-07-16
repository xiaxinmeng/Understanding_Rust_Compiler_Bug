
error[E0310]: the parameter type `T` may not live long enough
  --> ../../src/test/ui/lifetimes/lifetime-doesnt-live-long-enough.rs:28:5
   |
28 |     foo: &'static T
   |     ^^^^^^^^^^^^^^^
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...
note: ...so that the reference type `&'static T` does not outlive the data it points at
  --> ../../src/test/ui/lifetimes/lifetime-doesnt-live-long-enough.rs:28:5
   |
28 |     foo: &'static T
   |     ^^^^^^^^^^^^^^^
